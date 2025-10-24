use super::{context::Context, hooks::milliseconds_to_timestamp, Video};
use crate::rendering::stringify_svg;
use crate::SVGRenderable;
use anyhow::Result;
use measure_time::debug_time;
use std::sync::mpsc::SyncSender;
use std::time::Duration;

impl<AdditionalContext: Default> Video<AdditionalContext> {
    pub fn render(
        &self,
        output: SyncSender<(Duration, String)>,
        controller: impl Fn(&Context<AdditionalContext>) -> EngineControl,
    ) -> Result<usize> {
        debug_time!("render");

        let mut rendered_frames_count: usize = 0;
        let mut context = Context {
            frame: 0,
            beat: 0,
            beat_fractional: 0.0,
            timestamp: "00:00:00.000".to_string(),
            ms: 0,
            bpm: self.syncdata.bpm,
            syncdata: &self.syncdata,
            extra: AdditionalContext::default(),
            later_hooks: vec![],
            audiofile: self.audiofile.clone(),
            duration_override: self.duration_override,
        };

        let mut canvas = self.initial_canvas.clone();

        let mut previous_rendered_beat = 0;
        let mut previous_rendered_frame = 0;

        let render_ms_range = self.start_rendering_at + 0..self.duration_ms();

        self.progress_bar.set_length(render_ms_range.len() as u64);

        for _ in render_ms_range {
            context.ms += 1_usize;
            context.timestamp = milliseconds_to_timestamp(context.ms).to_string();
            context.beat_fractional =
                (context.bpm * context.ms) as f32 / (1000.0 * 60.0);
            context.beat = context.beat_fractional as usize;
            context.frame = self.fps * context.ms / 1000;

            let control = controller(&context);

            if control.stop_rendering_beforehand() {
                println!(
                    "Stopping rendering as requested before frame {}",
                    context.frame
                );
                break;
            }

            if context.marker() != "" {
                self.progress_bar.println(format!(
                    "{}: marker {}",
                    context.timestamp,
                    context.marker()
                ));
            }

            if context.marker().starts_with(':') {
                let marker_text = context.marker();
                let commandline = marker_text.trim_start_matches(':').to_string();

                for command in &self.commands {
                    if commandline.starts_with(&command.name) {
                        let args = commandline
                            .trim_start_matches(&command.name)
                            .trim()
                            .to_string();
                        (command.action)(args, &mut canvas, &mut context)?;
                    }
                }
            }

            // Render later hooks first, so that for example animations that aren't finished yet get overwritten by next frame's hook, if the next frames touches the same object
            // This is way better to cancel early animations such as fading out an object that appears on every note of a stem, if the next note is too close for the fade-out to finish.

            let mut later_hooks_to_delete: Vec<usize> = vec![];

            for (i, hook) in context.later_hooks.iter().enumerate() {
                if (hook.when)(&canvas, &context, previous_rendered_beat) {
                    (hook.render_function)(&mut canvas, context.ms)?;
                    if hook.once {
                        later_hooks_to_delete.push(i);
                    }
                } else if !hook.once {
                    later_hooks_to_delete.push(i);
                }
            }

            for i in later_hooks_to_delete {
                if i < context.later_hooks.len() {
                    context.later_hooks.remove(i);
                }
            }

            for hook in &self.hooks {
                if (hook.when)(
                    &canvas,
                    &context,
                    previous_rendered_beat,
                    previous_rendered_frame,
                ) {
                    (hook.render_function)(&mut canvas, &mut context)?;
                }
            }

            if control.render_this_one()
                && context.frame != previous_rendered_frame
            {
                output.send((
                    Duration::from_millis(context.ms as _),
                    stringify_svg(canvas.render_to_svg(
                        canvas.colormap.clone(),
                        canvas.cell_size,
                        canvas.object_sizes,
                        "",
                    )?),
                ))?;

                rendered_frames_count += 1;

                previous_rendered_beat = context.beat;
                previous_rendered_frame = context.frame;
            }

            if control.stop_rendering_afterwards() {
                println!(
                    "Stopping rendering as requested after frame {}",
                    context.frame
                );
                break;
            }
        }

        output.send((Duration::from_millis(context.ms as _), "".to_string()))?;

        println!("Rendered {rendered_frames_count} frames");
        Ok(rendered_frames_count)
    }

    pub fn render_single_frame(
        &self,
        frame_no: usize,
    ) -> Result<(Duration, String)> {
        let (tx, rx) = std::sync::mpsc::sync_channel::<(Duration, String)>(2);

        self.render(tx, |ctx| {
            if ctx.frame == frame_no {
                EngineControl::Finish
            } else if ctx.frame < frame_no {
                EngineControl::Skip
            } else {
                EngineControl::Stop
            }
        })?;

        println!("Waiting for rendered frame...");
        for (timecode, svg) in rx.iter() {
            if svg.is_empty() {
                continue;
            }

            return Ok((timecode, svg));
        }

        return Err(anyhow::format_err!(
            "Renderer did not output any non-empty frames"
        ));
    }

    pub fn render_all_frames(
        &self,
        output: SyncSender<(Duration, String)>,
    ) -> Result<usize> {
        self.render(output, |_| EngineControl::Render)
    }
}

/// Tells the rendering engine what to do with a frame
pub enum EngineControl {
    /// Skip to the next frame, don't render this one
    Skip,
    /// Render this frame as usual
    Render,
    /// Render this frame and stop rendering afterwards
    Finish,
    /// Don't render this frame and stop rendering
    Stop,
}

impl EngineControl {
    pub fn render_this_one(&self) -> bool {
        match self {
            EngineControl::Render | EngineControl::Finish => true,
            EngineControl::Skip | EngineControl::Stop => false,
        }
    }

    pub fn stop_rendering_beforehand(&self) -> bool {
        match self {
            EngineControl::Stop => true,
            _ => false,
        }
    }

    pub fn stop_rendering_afterwards(&self) -> bool {
        match self {
            EngineControl::Finish => true,
            _ => false,
        }
    }
}
