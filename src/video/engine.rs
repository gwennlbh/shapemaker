use super::{context::Context, hooks::format_duration, Video};
use crate::rendering::svg;
use crate::{Canvas, Object, Point, SVGRenderable};
use anyhow::Result;
use measure_time::debug_time;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;
use std::time::Duration;

/// What data is sent to the output by the rendering engine for each rendered frame
pub enum EngineOutput {
    Finished,
    Frame(EngineProgression, svg::Node),
}

pub struct EngineProgression {
    pub ms: usize,
    pub timestamp: String,
    pub scene_name: Option<String>,
}

impl<'a, C: Default> Context<'a, C> {
    pub fn engine_progression(&self) -> EngineProgression {
        EngineProgression {
            ms: self.ms,
            timestamp: self.timestamp.clone(),
            scene_name: self.current_scene.clone(),
        }
    }
}

impl<AdditionalContext: Default> Video<AdditionalContext> {
    pub fn render(
        &self,
        output: SyncSender<EngineOutput>,
        controller: impl Fn(&Context<AdditionalContext>) -> EngineControl,
    ) -> Result<usize> {
        debug_time!("render");

        let mut rendered_frames_count: usize = 0;
        let mut context = Context {
            frame: 0,
            scene_frame: None,
            current_scene: None,
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
            scene_started_at_ms: None,
        };

        let mut canvas = self.initial_canvas.clone();

        let mut previous_rendered_beat = 0;
        let mut previous_rendered_frame = 0;

        let render_ms_range = self.start_rendering_at + 0..self.duration_ms();

        self.progress_bar.set_length(render_ms_range.len() as u64);

        for _ in render_ms_range {
            context.ms += 1_usize;
            context.timestamp = format_duration(context.ms).to_string();
            context.beat_fractional =
                (context.bpm * context.ms) as f32 / (1000.0 * 60.0);
            context.beat = context.beat_fractional as usize;
            context.frame = self.fps * context.ms / 1000;
            context.scene_frame = context
                .scene_started_at_ms
                .map(|start_ms| self.fps * (context.ms - start_ms) / 1000);

            let control = controller(&context);

            let (stop_before, stop_after, skip_rendering, skip_hooks) = (
                control.stop_rendering_beforehand(),
                control.stop_rendering_afterwards(),
                !control.render_this_one(),
                !control.run_hooks_on_this_one(),
            );

            if stop_before {
                break;
            }

            if skip_hooks {
                continue;
            }

            if let EngineControl::RenderFromCanvas(new_canvas) = control {
                canvas = new_canvas;
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

            if !skip_rendering && context.frame != previous_rendered_frame {
                output.send(EngineOutput::Frame(
                    context.engine_progression(),
                    canvas.render_to_svg(
                        canvas.colormap.clone(),
                        canvas.cell_size,
                        canvas.object_sizes,
                        "",
                    )?,
                ))?;

                rendered_frames_count += 1;

                previous_rendered_beat = context.beat;
                previous_rendered_frame = context.frame;
            }

            if stop_after {
                println!(
                    "Stopping rendering as requested after frame {}",
                    context.frame
                );
                break;
            }
        }

        output.send(EngineOutput::Finished)?;

        println!("Rendered {rendered_frames_count} frames");
        Ok(rendered_frames_count)
    }

    pub fn render_single_frame(
        &self,
        frame_no: usize,
    ) -> Result<(String, svg::Node)> {
        debug_time!("render_single_frame");
        let (tx, rx) = std::sync::mpsc::sync_channel::<EngineOutput>(2);

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
        for output in rx.iter() {
            match output {
                EngineOutput::Finished => break,
                EngineOutput::Frame(progression, svg) => {
                    return Ok((progression.timestamp, svg))
                }
            }
        }

        return Err(anyhow::format_err!(
            "Renderer did not output any non-empty frames"
        ));
    }

    pub fn render_all_frames(
        &self,
        output: SyncSender<EngineOutput>,
    ) -> Result<usize> {
        self.render(output, |_| EngineControl::Render)
    }
}

/// Tells the rendering engine what to do with a frame
pub enum EngineControl {
    /// Don't run hooks or anything on this frame
    Ignore,
    /// Skip to the next frame, don't render this one
    Skip,
    /// Render this frame as usual
    Render,
    /// Render this frame and stop rendering afterwards
    Finish,
    /// Don't render this frame and stop rendering
    Stop,
    /// Set canvas and then render this frame
    RenderFromCanvas(Canvas),
}

impl EngineControl {
    pub fn render_this_one(&self) -> bool {
        match self {
            EngineControl::RenderFromCanvas(_)
            | EngineControl::Render
            | EngineControl::Finish => true,
            EngineControl::Ignore | EngineControl::Skip | EngineControl::Stop => {
                false
            }
        }
    }

    pub fn run_hooks_on_this_one(&self) -> bool {
        match self {
            EngineControl::Ignore => false,
            _ => true,
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
