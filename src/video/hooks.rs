use super::animation::LayerAnimationUpdateFunction;
use super::context::Context;
use crate::synchronization::audio::MusicalDurationUnit;
use crate::{Canvas, ColoredObject};
use anyhow::Result;
use chrono::NaiveDateTime;
use std::{fmt::Formatter, panic};

pub type BeatNumber = usize;
pub type FrameNumber = usize;
pub type Millisecond = usize;

pub type RenderFunction<C> =
    dyn Fn(&mut Canvas, &mut Context<C>) -> anyhow::Result<()> + Send + Sync;

pub type CommandAction<C> = dyn Fn(String, &mut Canvas, &mut Context<C>) -> anyhow::Result<()>
    + Send
    + Sync;

/// Arguments: canvas, context, previous rendered beat, previous rendered frame
pub type HookCondition<C> =
    dyn Fn(&Canvas, &Context<C>, BeatNumber, FrameNumber) -> bool + Send + Sync;

/// Arguments: canvas, context, current milliseconds timestamp
pub type LaterRenderFunction =
    dyn Fn(&mut Canvas, Millisecond) -> anyhow::Result<()> + Send + Sync;

/// Arguments: canvas, context, previous rendered beat
pub type LaterHookCondition<C> =
    dyn Fn(&Canvas, &Context<C>, BeatNumber) -> bool + Send + Sync;

pub struct Hook<C> {
    pub when: Box<HookCondition<C>>,
    pub render_function: Box<RenderFunction<C>>,
}

pub struct LaterHook<C> {
    pub when: Box<LaterHookCondition<C>>,
    pub render_function: Box<LaterRenderFunction>,
    /// Whether the hook should be run only once
    pub once: bool,
}

impl<C> std::fmt::Debug for Hook<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hook")
            .field("when", &"Box<HookCondition>")
            .field("render_function", &"Box<RenderFunction>")
            .finish()
    }
}

pub trait AttachHooks<AdditionalContext>: Sized {
    fn with_hook(self, hook: Hook<AdditionalContext>) -> Self;

    fn init(
        self,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, context, _, _| context.rendered_frames == 0),
            render_function: Box::new(render_function),
        })
    }

    // TODO The &'static requirement might be possibly liftable, see https://users.rust-lang.org/t/how-to-store-functions-in-structs/58089
    fn on(
        self,
        marker_text: &'static str,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, context, _, _| {
                context.marker() == marker_text
            }),
            render_function: Box::new(render_function),
        })
    }

    fn assign_scene_to(
        self,
        marker_text: &'static str,
        scene_name: &'static str,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, context, _, _| {
                context.marker() == marker_text
            }),
            render_function: Box::new(move |_, context| {
                context.switch_scene(scene_name);
                Ok(())
            }),
        })
    }

    fn each_beat(
        self,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(
                move |_,
                      context,
                      previous_rendered_beat,
                      previous_rendered_frame| {
                    previous_rendered_frame != context.frame()
                        && (context.ms == 0
                            || previous_rendered_beat != context.beat())
                },
            ),
            render_function: Box::new(render_function),
        })
    }

    fn every(
        self,
        amount: f32,
        unit: MusicalDurationUnit,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        let beats = match unit {
            MusicalDurationUnit::Beats => amount,
            MusicalDurationUnit::Halves => amount / 2.0,
            MusicalDurationUnit::Quarters => amount / 4.0,
            MusicalDurationUnit::Eighths => amount / 8.0,
            MusicalDurationUnit::Sixteenths => amount / 16.0,
            MusicalDurationUnit::Thirds => amount / 3.0,
        };

        self.with_hook(Hook {
            when: Box::new(move |_, context, _, _| {
                context.beat_fractional() % beats < 0.01
            }),
            render_function: Box::new(render_function),
        })
    }

    fn each_frame(
        self,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.each_n_frame(1, render_function)
    }

    fn each_n_frame(
        self,
        n: usize,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, context, _, previous_rendered_frame| {
                context.frame() - previous_rendered_frame >= n
            }),
            render_function: Box::new(render_function),
        })
    }

    /// threshold is a value between 0 and 1: current amplitude / max amplitude of stem
    fn on_stem(
        self,
        stem_name: &'static str,
        threshold: f32,
        above_amplitude: &'static RenderFunction<AdditionalContext>,
        below_amplitude: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, context, _, _| {
                context.stem(stem_name).amplitude_relative() > threshold
            }),
            render_function: Box::new(above_amplitude),
        })
        .with_hook(Hook {
            when: Box::new(move |_, context, _, _| {
                context.stem(stem_name).amplitude_relative() <= threshold
            }),
            render_function: Box::new(below_amplitude),
        })
    }

    /// Triggers when a note starts on one of the stems in the comma-separated list of stem names `stems`.
    fn on_note(
        self,
        stems: &'static str,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, ctx, _, _| {
                stems
                    .split(',')
                    .map(|stem_name| ctx.stem(stem_name.trim()))
                    .any(|stem| stem.notes.iter().any(|note| note.is_on()))
            }),
            render_function: Box::new(render_function),
        })
    }

    /// Triggers when a note stops on one of the stems in the comma-separated list of stem names `stems`.
    fn on_note_end(
        self,
        stems: &'static str,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, ctx, _, _| {
                stems
                    .split(',')
                    .map(|n| ctx.stem(n.trim()))
                    .any(|stem| stem.notes.iter().any(|note| note.is_off()))
            }),
            render_function: Box::new(render_function),
        })
    }

    // Adds an object using object_creation on note start and removes it on note end
    fn with_note<ObjectCreator>(
        self,
        stems: &'static str,
        cutoff_amplitude: f32,
        layer_name: &'static str,
        object_name: &'static str,
        create_object: &'static ObjectCreator,
    ) -> Self
    where
        ObjectCreator: Fn(&Canvas, &mut Context<AdditionalContext>) -> Result<ColoredObject>
            + Send
            + Sync,
    {
        self.with_hook(Hook {
            when: Box::new(move |_, ctx, _, _| {
                stems.split(',').any(|stem_name| {
                    ctx.stem(stem_name).notes.iter().any(|note| note.is_on())
                })
            }),
            render_function: Box::new(move |canvas, ctx| {
                let object = create_object(canvas, ctx)?;
                canvas.layer(layer_name).set(object_name, object);
                Ok(())
            }),
        })
        .with_hook(Hook {
            when: Box::new(move |_, ctx, _, _| {
                stems.split(',').any(|stem_name| {
                    ctx.stem(stem_name).amplitude_relative() < cutoff_amplitude
                        || ctx
                            .stem(stem_name)
                            .notes
                            .iter()
                            .any(|note| note.is_off())
                })
            }),
            render_function: Box::new(move |canvas, _| {
                canvas.remove_object(object_name);
                Ok(())
            }),
        })
    }

    fn at_frame(
        self,
        frame: usize,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, context, _, _| context.frame() == frame),
            render_function: Box::new(render_function),
        })
    }

    fn when_remaining(
        self,
        seconds: usize,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, ctx, _, _| {
                ctx.ms >= ctx.duration_ms().max(seconds * 1000) - seconds * 1000
            }),
            render_function: Box::new(render_function),
        })
    }

    fn at_timestamp(
        self,
        timestamp: &'static str,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        let hook = Hook {
            when: Box::new(move |_, context, _, previous_rendered_frame| {
                if previous_rendered_frame == context.frame() {
                    return false;
                }
                let (precision, criteria_time): (&str, NaiveDateTime) =
                    if let Ok(criteria_time_parsed) =
                        NaiveDateTime::parse_from_str(timestamp, "%H:%M:%S%.3f")
                    {
                        ("milliseconds", criteria_time_parsed)
                    } else if let Ok(criteria_time_parsed) =
                        NaiveDateTime::parse_from_str(timestamp, "%M:%S%.3f")
                    {
                        ("milliseconds", criteria_time_parsed)
                    } else if let Ok(criteria_time_parsed) =
                        NaiveDateTime::parse_from_str(timestamp, "%S%.3f")
                    {
                        ("milliseconds", criteria_time_parsed)
                    } else if let Ok(criteria_time_parsed) =
                        NaiveDateTime::parse_from_str(timestamp, "%S")
                    {
                        ("seconds", criteria_time_parsed)
                    } else if let Ok(criteria_time_parsed) =
                        NaiveDateTime::parse_from_str(timestamp, "%M:%S")
                    {
                        ("seconds", criteria_time_parsed)
                    } else if let Ok(criteria_time_parsed) =
                        NaiveDateTime::parse_from_str(timestamp, "%H:%M:%S")
                    {
                        ("seconds", criteria_time_parsed)
                    } else {
                        panic!("Unhandled timestamp format: {}", timestamp);
                    };
                match precision {
                    "milliseconds" => {
                        let current_time: NaiveDateTime =
                            NaiveDateTime::parse_from_str(
                                timestamp,
                                "%H:%M:%S%.3f",
                            )
                            .unwrap();
                        current_time == criteria_time
                    }
                    "seconds" => {
                        let current_time: NaiveDateTime =
                            NaiveDateTime::parse_from_str(timestamp, "%H:%M:%S")
                                .unwrap();
                        current_time == criteria_time
                    }
                    _ => panic!("Unknown precision"),
                }
            }),
            render_function: Box::new(render_function),
        };
        self.with_hook(hook)
    }

    fn bind_amplitude(
        self,
        layer: &'static str,
        stem: &'static str,
        update: &'static LayerAnimationUpdateFunction,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, _, _, _| true),
            render_function: Box::new(move |canvas, context| {
                let amplitude = context.stem(stem).amplitude_relative();
                update(amplitude, canvas.layer(layer), context.ms)?;
                Ok(())
            }),
        })
    }
}
