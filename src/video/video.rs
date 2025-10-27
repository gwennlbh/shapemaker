use crate::{
    synchronization::{
        cue_markers::CueMarkersSynchronizer,
        midi::MidiSynchronizer,
        sync::{SyncData, Syncable},
    },
    ui::{self, display_counts, Log},
    video::hooks::{AttachHooks, CommandAction, Hook},
    Canvas, Scene,
};
use indicatif::ProgressBar;
use measure_time::debug_time;
use std::{collections::HashMap, fmt::Formatter, path::PathBuf};

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
            progress_bar: ui::setup_progress_bar(0, ""),
        }
    }

    pub fn sync_audio_with(self, sync_data_path: impl Into<PathBuf>) -> Self {
        debug_time!("sync_audio_with");

        let file_path: PathBuf = sync_data_path.into();
        let pb = Some(&self.progress_bar);

        let syncdata = match file_path.extension().and_then(|s| s.to_str()) {
            Some("mid" | "midi") => {
                MidiSynchronizer::new(file_path.clone()).load(pb)
            }
            Some("flac" | "wav") => {
                CueMarkersSynchronizer::new(file_path.clone()).load(pb)
            }
            _ => panic!("Unsupported sync data format"),
        };

        self.progress_bar.finish();
        self.progress_bar.log(
            "Loaded",
            &format!(
                "{} from {file_path:?}",
                display_counts(HashMap::from([
                    ("markers", syncdata.markers.len()),
                    (
                        "notes",
                        syncdata
                            .stems
                            .values()
                            .map(|v| v.notes.len())
                            .sum::<usize>()
                    ),
                ])),
            ),
        );

        return Self {
            syncdata: self.syncdata.union(syncdata),
            ..self
        };
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
            when: Box::new(|_, ctx, _, _| ctx.frame() == 0),
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
