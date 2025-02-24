use std::collections::HashMap;
use std::str::FromStr;
use std::{
    fmt::Formatter,
    fs::create_dir_all,
    panic,
    path::{Path, PathBuf},
};

use ffmpeg::{codec, format, media, Packet};
use ffmpeg_next::encoder;
extern crate ffmpeg_next as ffmpeg;
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime};
use indicatif::{ProgressBar, ProgressIterator};
use measure_time::info_time;
use video_rs::Time;

use crate::{
    sync::SyncData,
    ui::{self, setup_progress_bar, Log as _},
    Canvas, ColoredObject, Context, LayerAnimationUpdateFunction, MidiSynchronizer,
    MusicalDurationUnit, Syncable,
};

pub type BeatNumber = usize;
pub type FrameNumber = usize;
pub type Millisecond = usize;

pub type RenderFunction<C> = dyn Fn(&mut Canvas, &mut Context<C>) -> anyhow::Result<()>;
pub type CommandAction<C> = dyn Fn(String, &mut Canvas, &mut Context<C>) -> anyhow::Result<()>;

/// Arguments: canvas, context, previous rendered beat, previous rendered frame
pub type HookCondition<C> = dyn Fn(&Canvas, &Context<C>, BeatNumber, FrameNumber) -> bool;

/// Arguments: canvas, context, current milliseconds timestamp
pub type LaterRenderFunction = dyn Fn(&mut Canvas, Millisecond) -> anyhow::Result<()>;

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
    encoder: Option<video_rs::Encoder>,
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
            encoder: None,
        }
    }

    fn setup_encoder(&mut self, output_path: &str) -> anyhow::Result<()> {
        let (width, height) = self.initial_canvas.resolution_to_size(self.resolution);

        self.encoder = Some(
            video_rs::Encoder::new(
                PathBuf::from_str(output_path)?,
                video_rs::encode::Settings::preset_h264_yuv420p(
                    width as usize,
                    height as usize,
                    false,
                ),
            )
            .expect("Failed to build encoder"),
        );

        Ok(())
    }

    pub fn sync_audio_with(self, sync_data_path: &str) -> Self {
        info_time!("sync_audio_with");
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

    pub fn init(self, render_function: &'static RenderFunction<AdditionalContext>) -> Self {
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
            when: Box::new(move |_, context, _, _| context.marker() == marker_text),
            render_function: Box::new(render_function),
        })
    }

    pub fn each_beat(self, render_function: &'static RenderFunction<AdditionalContext>) -> Self {
        self.with_hook(Hook {
            when: Box::new(
                move |_, context, previous_rendered_beat, previous_rendered_frame| {
                    previous_rendered_frame != context.frame
                        && (context.ms == 0 || previous_rendered_beat != context.beat)
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
            when: Box::new(move |_, context, _, _| context.beat_fractional % beats < 0.01),
            render_function: Box::new(render_function),
        })
    }

    pub fn each_frame(self, render_function: &'static RenderFunction<AdditionalContext>) -> Self {
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
                    .map(|n| ctx.stem(n.trim()))
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
                stems
                    .split(',')
                    .any(|stem_name| ctx.stem(stem_name).notes.iter().any(|note| note.is_on()))
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
                        || ctx.stem(stem_name).notes.iter().any(|note| note.is_off())
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
                            NaiveDateTime::parse_from_str(timestamp, "%H:%M:%S%.3f").unwrap();
                        current_time == criteria_time
                    }
                    "seconds" => {
                        let current_time: NaiveDateTime =
                            NaiveDateTime::parse_from_str(timestamp, "%H:%M:%S").unwrap();
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

    // Saves PNG frames to disk. Returns number of frames written.
    pub fn render_frames(&mut self) -> Result<usize> {
        let mut written_frames_count: usize = 0;
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

        let render_ms_range = 0..self.duration_ms() + self.start_rendering_at;

        self.progress_bar.set_length(render_ms_range.len() as u64);

        for _ in render_ms_range
            .into_iter()
            .progress_with(self.progress_bar.clone())
        {
            context.ms += 1_usize;
            context.timestamp = milliseconds_to_timestamp(context.ms).to_string();
            context.beat_fractional = (context.bpm * context.ms) as f32 / (1000.0 * 60.0);
            context.beat = context.beat_fractional as usize;
            context.frame = self.fps * context.ms / 1000;

            self.progress_bar.set_message(context.timestamp.clone());

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

            if context.frame != previous_rendered_frame {
                info_time!("render_frame");
                self.encoder
                    .as_mut()
                    .expect("Encoder was not initialized")
                    .encode(
                        &canvas.render_to_hwc_frame(self.resolution)?,
                        Time::from_secs_f64(context.ms as f64 * 1e-3),
                    )?;

                written_frames_count += 1;

                previous_rendered_beat = context.beat;
                previous_rendered_frame = context.frame;
            }
        }

        Ok(written_frames_count)
    }

    pub fn setup_progress_bar(&self) -> ProgressBar {
        ui::setup_progress_bar(self.total_frames() as u64, "Rendering")
    }

    pub fn render(&mut self, output_file: String) -> Result<()> {
        info_time!("render");

        // create_dir_all(self.frames_output_directory)?;
        // remove_dir_all(self.frames_output_directory)?;
        // create_dir(self.frames_output_directory)?;
        create_dir_all(Path::new(&output_file).parent().unwrap())?;

        self.setup_encoder(&output_file)?;

        self.progress_bar.set_position(0);
        self.progress_bar.set_prefix("Rendering");
        self.progress_bar.set_message("");

        let frames_written = self.render_frames()?;

        self.encoder
            .as_mut()
            .expect("Encoder is missing somehow")
            .finish()?;

        self.progress_bar.log(
            "Rendered",
            &format!("{} frames to {}", frames_written, output_file),
        );

        self.progress_bar.set_position(0);
        self.progress_bar.set_prefix("Adding");
        self.progress_bar.set_message("audio track");

        Ok(())
    }

    fn add_audio_track(&mut self, output_file: String) -> Result<()> {
        todo!("Look into https://github.com/zmwangx/rust-ffmpeg/blob/master/examples/transcode-x264.rs and maybe contribute to video-rs (see https://github.com/oddity-ai/video-rs/issues/44)");
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
