use crate::{
    Canvas, Scene,
    synchronization::{
        cue_markers::CueMarkersSynchronizer,
        midi::MidiSynchronizer,
        sync::{SyncData, Syncable},
    },
    ui::{self, Log, Pretty},
    video::hooks::{AttachHooks, CommandAction, Hook},
};
use anyhow::Result;
use chrono::DateTime;
use measure_time::debug_time;
use std::{
    collections::HashMap, fmt::Formatter, ops::Range, path::PathBuf,
    time::Duration,
};

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

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Timestamp(pub usize);

impl Timestamp {
    pub fn from_ms_range(range: &Range<usize>) -> Range<Self> {
        Self::from_ms(range.start)..Self::from_ms(range.end)
    }

    pub fn ms(&self) -> usize {
        self.0
    }

    pub fn seconds(&self) -> f64 {
        self.0 as f64 / 1000.0
    }

    pub fn seconds_string(&self) -> String {
        format!("{:.3}", self.seconds())
    }

    pub fn from_seconds(seconds: f64) -> Self {
        Self((seconds * 1000.0) as usize)
    }

    pub fn from_ms(ms: usize) -> Self {
        Self(ms)
    }
}

impl Pretty for Timestamp {
    fn pretty(&self) -> String {
        format!(
            "{}",
            DateTime::from_timestamp_millis(self.ms() as i64)
                .unwrap()
                .format("%H:%M:%S%.3f")
        )
    }
}

impl Pretty for Range<Timestamp> {
    fn pretty(&self) -> String {
        format!("from {} to {}", self.start.pretty(), self.end.pretty())
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self(0)
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty())
    }
}

pub struct VideoProgressBars {
    pub loading: indicatif::ProgressBar,
    pub rendering: indicatif::ProgressBar,
    pub encoding: indicatif::ProgressBar,
}

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
    pub duration_override: Option<Duration>,
    pub start_rendering_at: Timestamp,
    pub progress_bars: VideoProgressBars,
    pub progress: indicatif::MultiProgress,
}

impl<C: Default> AttachHooks<C> for Video<C> {
    fn with_hook(self, hook: Hook<C>) -> Self {
        let mut hooks = self.hooks;
        hooks.push(hook);
        Self { hooks, ..self }
    }
}

impl<C: Default> Default for Video<C> {
    fn default() -> Self {
        Self::new(Canvas::with_layers(vec!["root"]))
    }
}

impl<C: Default> Video<C> {
    pub fn new(canvas: Canvas) -> Self {
        let progress_bars = VideoProgressBars {
            loading: ui::setup_progress_bar(0, "Loading"),
            rendering: ui::setup_progress_bar(0, "Rendering"),
            encoding: ui::setup_progress_bar(0, "Encoding"),
        };

        let progress = indicatif::MultiProgress::new();
        progress.add(progress_bars.loading.clone());
        progress.add(progress_bars.rendering.clone());
        progress.add(progress_bars.encoding.clone());

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
            start_rendering_at: Timestamp::from_ms(0),
            progress_bars,
            progress,
        }
    }

    pub fn sync_audio_with(
        mut self,
        filepath: impl Into<PathBuf>,
    ) -> Result<Self> {
        debug_time!("sync_audio_with");

        let file_path: PathBuf = filepath.into();
        let pb = Some(&self.progress_bars.loading);

        let syncdata = match file_path.extension().and_then(|s| s.to_str()) {
            Some("mid" | "midi") => {
                MidiSynchronizer::new(file_path.clone()).load(pb)
            }
            Some("flac" | "wav") => {
                CueMarkersSynchronizer::new(file_path.clone()).load(pb)
            }
            _ => panic!("Unsupported sync data format"),
        }?;

        let pb = pb.unwrap();

        pb.finish();

        if let Some(bpm) = syncdata.bpm {
            pb.log(
                "BPM",
                &format!("set to {bpm} from {}", (&file_path).pretty()),
            );
        }

        pb.log(
            "Loaded",
            &format!(
                "{things} from {path} in {elapsed}",
                path = (&file_path).pretty(),
                elapsed = (pb.elapsed().pretty()),
                things = (HashMap::from([
                    ("markers", syncdata.markers.len()),
                    ("stems", syncdata.stems.len()),
                    (
                        "notes",
                        syncdata
                            .stems
                            .values()
                            .map(|v| v.notes.len())
                            .sum::<usize>()
                    ),
                ]))
                .pretty(),
            ),
        );

        self.syncdata.merge_with(syncdata);

        Ok(self)
    }

    pub fn ms_to_frames(&self, ms: usize) -> usize {
        self.fps * ms / 1000
    }

    // Duration of the video, taking into account a possible duration override.
    pub fn duration_ms(&self) -> usize {
        match self.duration_override {
            Some(duration) => duration.as_millis() as _,
            None => self.total_duration_ms(),
        }
    }

    pub fn constrained_ms_range(&self) -> Range<usize> {
        let start_ms = self.start_rendering_at.ms();
        let end_ms = start_ms + self.duration_ms();
        start_ms..end_ms.min(self.total_duration_ms())
    }

    pub fn total_ms_range(&self) -> Range<usize> {
        0..self.total_duration_ms()
    }

    /// Duration of the video, without taking into account a possible duration override.
    pub fn total_duration_ms(&self) -> usize {
        self.syncdata
            .stems
            .values()
            .map(|stem| stem.duration_ms)
            .max()
 .expect("No audio sync data provided. Use .sync_audio_with() to load a MIDI file, or provide a duration override.")
    }

    /// Adds hooks from the given scene to the video.
    /// Hooks will be triggered when the current scene matches the scene's name.
    /// Use Context#switch_scene to change scenes during rendering.
    /// See also `with_marked_scene` for a more ergonomic way to add scenes.
    pub fn with_scene(self, mut scene: Scene<C>) -> Self {
        for hook in self.hooks {
            scene.hooks.push(hook);
        }
        Self {
            hooks: scene.hooks,
            ..self
        }
    }

    /// Adds the given scene and a hook that switches to it immediately.
    pub fn with_init_scene(self, scene: Scene<C>) -> Self {
        let scene_name = scene.name.clone();
        self.with_scene(scene).with_hook(Hook {
            when: Box::new(|_, ctx, _, _| ctx.rendered_frames == 0),
            render_function: Box::new(move |_, ctx| {
                ctx.switch_scene(&scene_name);
                Ok(())
            }),
        })
    }

    /// Adds the given scene, and a hook that switches to it when a marker with the same name is reached
    pub fn with_marked_scene(self, scene: Scene<C>) -> Self {
        let scene_name = scene.name.clone();

        self.with_scene(scene).with_hook(Hook {
            when: Box::new(move |_, ctx, _, _| ctx.marker() == scene_name),
            render_function: Box::new(move |_, ctx| {
                ctx.switch_scene(ctx.marker());
                Ok(())
            }),
        })
    }

    pub fn command(
        self,
        command_name: &'static str,
        action: &'static CommandAction<C>,
    ) -> Self {
        let mut commands = self.commands;
        commands.push(Box::new(Command {
            name: command_name.to_string(),
            action: Box::new(action),
        }));
        Self { commands, ..self }
    }
}
