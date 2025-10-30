use super::{Video, context::Context};
use crate::SVGRenderable;
use crate::rendering::svg;
use crate::ui::{Log, Pretty};
use anyhow::Result;
use measure_time::debug_time;
use std::sync::mpsc::SyncSender;

pub type EngineController<C: Default> = dyn Fn(&Context<'_, C>) -> EngineControl;

/// What data is sent to the output by the rendering engine for each rendered frame
pub enum EngineOutput {
    Finished,
    Frame {
        svg: svg::Node,
        dimensions: (usize, usize),
    },
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
            timestamp: self.timestamp().pretty(),
            scene_name: self.current_scene.clone(),
        }
    }
}

impl<C: Default> Video<C> {
    pub fn render(
        &self,
        output: SyncSender<EngineOutput>,
        controller: &EngineController<C>,
    ) -> Result<usize> {
        debug_time!("render");

        let mut context = Context {
            rendered_frames: 0,
            ms: 0,
            current_scene: None,
            fps: self.fps,
            syncdata: &self.syncdata,
            extra: C::default(),
            inner_hooks: vec![],
            audiofile: self.audiofile.clone(),
            duration_override: self.duration_override,
            scene_started_at_ms: None,
            bpm: self
                .syncdata
                .bpm
                .expect("No sync source could determine the BPM"),
        };

        let mut canvas = self.initial_canvas.clone();

        let mut previous_rendered_beat = 0;
        let mut previous_rendered_frame = 0;

        let pb = self.progress_bars.rendering.clone();
        pb.set_prefix("Rendering");
        pb.set_message("");
        pb.set_position(0);
        pb.set_length(self.duration_ms() as _);

        for _ in 0..self.total_duration_ms() {
            context.ms += 1;

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

            pb.inc(1);
            pb.set_message(match context.current_scene {
                Some(ref scene) => {
                    format!("{}: {scene}", context.timestamp())
                }
                None => format!("{}", context.timestamp()),
            });

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

            for (i, hook) in context.inner_hooks.iter().enumerate() {
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
                if i < context.inner_hooks.len() {
                    context.inner_hooks.remove(i);
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

            if context.frame() != previous_rendered_frame {
                if !skip_rendering {
                    output.send(EngineOutput::Frame {
                        dimensions: (canvas.width(), canvas.height()),
                        svg: canvas.render_to_svg(
                            canvas.colormap.clone(),
                            canvas.cell_size,
                            canvas.object_sizes,
                            "",
                        )?,
                    })?;
                }

                context.rendered_frames += 1;

                previous_rendered_beat = context.beat();
                previous_rendered_frame = context.frame();
            }

            if stop_after {
                break;
            }
        }

        output.send(EngineOutput::Finished)?;

        pb.finish();
        pb.log(
            "Rendered",
            &format!(
                "{} frames in {}",
                context.rendered_frames,
                pb.elapsed().pretty()
            ),
        );
        self.progress.remove(&pb);

        Ok(context.rendered_frames)
    }

    /// Render a single frame at the given frame number. Skip all hooks, expect for `render_ahead`
    /// frames before the requested one.
    pub fn render_frame(
        &self,
        frame_no: usize,
        render_ahead: usize,
    ) -> Result<svg::Node> {
        debug_time!("render_single_frame");
        let (tx, rx) = std::sync::mpsc::sync_channel::<EngineOutput>(2);

        let render_ahead_range = frame_no.saturating_sub(render_ahead)..frame_no;

        self.render(tx, &move |ctx| {
            if ctx.frame() == frame_no {
                EngineControl::Finish
            } else if render_ahead_range.contains(&ctx.frame()) {
                EngineControl::Walk
            } else {
                EngineControl::Skip
            }
        })?;

        for output in rx.iter() {
            match output {
                EngineOutput::Finished => break,
                EngineOutput::Frame { svg, .. } => return Ok(svg),
            }
        }

        return Err(anyhow::format_err!(
            "Renderer did not output any non-empty frames"
        ));
    }
}

/// Tells the rendering engine what to do with a frame
pub enum EngineControl {
    /// Don't run hooks or anything on this frame
    Skip,
    /// Skip to the next frame, don't render this one
    Walk,
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
            EngineControl::Skip | EngineControl::Walk | EngineControl::Stop => {
                false
            }
        }
    }

    pub fn run_hooks_on_this_one(&self) -> bool {
        match self {
            EngineControl::Skip => false,
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
