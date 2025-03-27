use super::animation::LayerAnimationUpdateFunction;
use super::context::Context;
use crate::synchronization::audio::MusicalDurationUnit;
use crate::synchronization::midi::MidiSynchronizer;
use crate::synchronization::sync::{SyncData, Syncable};
use crate::ui::{self, setup_progress_bar, Log as _};
use crate::{Canvas, ColoredObject};
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime};
use indicatif::ProgressBar;
use measure_time::debug_time;
#[allow(unused)]
use std::sync::{Arc, Mutex};
use std::{fmt::Formatter, panic, path::PathBuf};

pub type BeatNumber = usize;
pub type FrameNumber = usize;
pub type Millisecond = usize;

pub type RenderFunction<C> =
    dyn Fn(&mut Canvas, &mut Context<C>) -> anyhow::Result<()>;
pub type CommandAction<C> =
    dyn Fn(String, &mut Canvas, &mut Context<C>) -> anyhow::Result<()>;

/// Arguments: canvas, context, previous rendered beat, previous rendered frame
pub type HookCondition<C> =
    dyn Fn(&Canvas, &Context<C>, BeatNumber, FrameNumber) -> bool;

/// Arguments: canvas, context, current milliseconds timestamp
pub type LaterRenderFunction =
    dyn Fn(&mut Canvas, Millisecond) -> anyhow::Result<()>;

/// Arguments: canvas, context, previous rendered beat
pub type LaterHookCondition<C> = dyn Fn(&Canvas, &Context<C>, BeatNumber) -> bool;

pub struct Video<C> {
    pub fps: usize,
    pub initial_canvas: Canvas,
    pub hooks: Vec<Hook<C>>,
    pub commands: Vec<Box<Command<C>>>,
    pub frames: Vec<Canvas>,
    pub frames_output_directory: &'static str,
    pub syncdata: SyncData,
    pub audiofile: PathBuf,
    pub resolution: u32,
    pub duration_override: Option<usize>,
    pub start_rendering_at: usize,
    pub progress_bar: indicatif::ProgressBar,

    #[cfg(feature = "mp4")]
    pub encoder: Option<Arc<Mutex<video_rs::Encoder>>>,
}

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

pub struct Command<C> {
    pub name: String,
    pub action: Box<CommandAction<C>>,
}

impl<C> std::fmt::Debug for Command<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("name", &self.name)
            .field("action", &"Box<CommandAction>")
            .finish()
    }
}

impl<AdditionalContext: Default> Default for Video<AdditionalContext> {
    fn default() -> Self {
        Self::new(Canvas::new(vec!["root"]))
    }
}

impl<AdditionalContext: Default> Video<AdditionalContext> {
    pub fn new(canvas: Canvas) -> Self {
        Self {
            fps: 30,
            initial_canvas: canvas,
            hooks: vec![],
            commands: vec![],
            frames: vec![],
            frames_output_directory: "frames/",
            resolution: 1920,
            syncdata: SyncData::default(),
            audiofile: PathBuf::new(),
            duration_override: None,
            start_rendering_at: 0,
            progress_bar: setup_progress_bar(0, ""),
            #[cfg(feature = "mp4")]
            encoder: None,
        }
    }

    pub fn sync_audio_with(self, sync_data_path: &str) -> Self {
        debug_time!("sync_audio_with");
        if sync_data_path.ends_with(".mid") || sync_data_path.ends_with(".midi") {
            let loader = MidiSynchronizer::new(sync_data_path);
            let syncdata = loader.load(Some(&self.progress_bar));
            self.progress_bar.finish();
            self.progress_bar.log(
                "Loaded",
                &format!(
                    "{} notes from {sync_data_path}",
                    syncdata
                        .stems
                        .values()
                        .map(|v| v.notes.len())
                        .sum::<usize>(),
                ),
            );
            return Self { syncdata, ..self };
        }

        panic!("Unsupported sync data format");
    }

    pub fn with_hook(self, hook: Hook<AdditionalContext>) -> Self {
        let mut hooks = self.hooks;
        hooks.push(hook);
        Self { hooks, ..self }
    }

    pub fn init(
        self,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, context, _, _| context.frame == 0),
            render_function: Box::new(render_function),
        })
    }

    pub fn on(
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

    pub fn each_beat(
        self,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(
                move |_,
                      context,
                      previous_rendered_beat,
                      previous_rendered_frame| {
                    previous_rendered_frame != context.frame
                        && (context.ms == 0
                            || previous_rendered_beat != context.beat)
                },
            ),
            render_function: Box::new(render_function),
        })
    }

    pub fn every(
        self,
        amount: f32,
        unit: MusicalDurationUnit,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        let beats = match unit {
            MusicalDurationUnit::Beats => amount,
            MusicalDurationUnit::Halfs => amount / 2.0,
            MusicalDurationUnit::Quarters => amount / 4.0,
            MusicalDurationUnit::Eighths => amount / 8.0,
            MusicalDurationUnit::Sixteenths => amount / 16.0,
            MusicalDurationUnit::Thirds => amount / 3.0,
        };

        self.with_hook(Hook {
            when: Box::new(move |_, context, _, _| {
                context.beat_fractional % beats < 0.01
            }),
            render_function: Box::new(render_function),
        })
    }

    pub fn each_frame(
        self,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        let hook = Hook {
            when: Box::new(move |_, context, _, previous_rendered_frame| {
                context.frame != previous_rendered_frame
            }),
            render_function: Box::new(render_function),
        };
        self.with_hook(hook)
    }

    pub fn each_n_frame(
        self,
        n: usize,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, context, _, previous_rendered_frame| {
                context.frame != previous_rendered_frame && context.frame % n == 0
            }),
            render_function: Box::new(render_function),
        })
    }

    /// threshold is a value between 0 and 1: current amplitude / max amplitude of stem
    pub fn on_stem(
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
    pub fn on_note(
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
    pub fn on_note_end(
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
    pub fn with_note(
        self,
        stems: &'static str,
        cutoff_amplitude: f32,
        layer_name: &'static str,
        object_name: &'static str,
        create_object: &'static dyn Fn(
            &Canvas,
            &mut Context<AdditionalContext>,
        ) -> Result<ColoredObject>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, ctx, _, _| {
                stems.split(',').any(|stem_name| {
                    ctx.stem(stem_name).notes.iter().any(|note| note.is_on())
                })
            }),
            render_function: Box::new(move |canvas, ctx| {
                let object = create_object(canvas, ctx)?;
                canvas.layer(layer_name).set_object(object_name, object);
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

    pub fn at_frame(
        self,
        frame: usize,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        self.with_hook(Hook {
            when: Box::new(move |_, context, _, _| context.frame == frame),
            render_function: Box::new(render_function),
        })
    }

    pub fn when_remaining(
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

    pub fn at_timestamp(
        self,
        timestamp: &'static str,
        render_function: &'static RenderFunction<AdditionalContext>,
    ) -> Self {
        let hook = Hook {
            when: Box::new(move |_, context, _, previous_rendered_frame| {
                if previous_rendered_frame == context.frame {
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

    pub fn command(
        self,
        command_name: &'static str,
        action: &'static CommandAction<AdditionalContext>,
    ) -> Self {
        let mut commands = self.commands;
        commands.push(Box::new(Command {
            name: command_name.to_string(),
            action: Box::new(action),
        }));
        Self { commands, ..self }
    }

    pub fn bind_amplitude(
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
                canvas.layer(layer).flush();
                Ok(())
            }),
        })
    }

    pub fn total_frames(&self) -> usize {
        self.fps * (self.duration_ms() + self.start_rendering_at) / 1000
    }

    pub fn duration_ms(&self) -> usize {
        if let Some(duration_override) = self.duration_override {
            return duration_override;
        }

        self.syncdata
            .stems
            .values()
            .map(|stem| stem.duration_ms)
            .max()
            .expect("No audio sync data provided. Use .sync_audio_with() to load a MIDI file, or provide a duration override.")
    }

    pub fn setup_progress_bar(&self) -> ProgressBar {
        ui::setup_progress_bar(self.total_frames() as u64, "Rendering")
    }
}

pub fn milliseconds_to_timestamp(ms: usize) -> String {
    format!(
        "{}",
        DateTime::from_timestamp_millis(ms as i64)
            .unwrap()
            .format("%H:%M:%S%.3f")
    )
}
