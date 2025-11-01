use super::Animation;
use super::animation::{AnimationUpdateFunction, LayerAnimationUpdateFunction};
use super::hooks::{InnerHook, InnerHookRenderFunction};
use crate::Timestamp;
use crate::synchronization::audio::{Note, StemAtInstant};
use crate::synchronization::sync::SyncData;
use itertools::Itertools;
use nanoid::nanoid;
use std::fmt::Display;
use std::fs::{self};
use std::path::PathBuf;
use std::time::Duration;

pub struct Context<'a, AdditionalContext = ()> {
    pub ms: usize,
    pub fps: usize,
    pub bpm: usize,
    pub syncdata: &'a SyncData,
    pub audiofile: PathBuf,
    pub inner_hooks: Vec<InnerHook<AdditionalContext>>,
    pub extra: AdditionalContext,
    pub duration_override: Option<Duration>,
    pub current_scene: Option<String>,
    pub scene_started_at_ms: Option<usize>,
    pub rendered_frames: usize,
}

impl<C> Context<'_, C> {
    pub fn timestamp(&self) -> Timestamp {
        Timestamp(self.ms)
    }

    pub fn beat_fractional(&self) -> f32 {
        (self.bpm * self.ms) as f32 / (1000.0 * 60.0)
    }

    pub fn beat(&self) -> usize {
        self.beat_fractional() as usize
    }

    pub fn frame(&self) -> usize {
        self.ms_to_frame(self.ms)
    }

    pub fn scene_frame(&self) -> Option<usize> {
        self.scene_started_at_ms
            .map(|start_ms| self.ms_to_frame(self.ms - start_ms))
    }

    pub fn ms_to_frame(&self, ms: usize) -> usize {
        self.fps * ms / 1000
    }

    pub fn stem(&self, name: &str) -> StemAtInstant {
        let stems = &self.syncdata.stems;
        if !stems.contains_key(name) {
            panic!(
                "No stem named {:?} found. Available stems:\n{}\n",
                name,
                stems
                    .keys()
                    .sorted()
                    .fold(String::new(), |acc, k| format!("{acc}\n\t{k}"))
            );
        }
        StemAtInstant {
            amplitude: *stems[name].amplitude_db.get(self.ms).unwrap_or(&0.0),
            amplitude_max: stems[name].amplitude_max,
            velocity_max: stems[name]
                .notes
                .get(&self.ms)
                .iter()
                .map(|notes| {
                    notes.iter().map(|note| note.velocity).max().unwrap_or(0)
                })
                .max()
                .unwrap_or(0),
            duration: stems[name].duration_ms,
            notes: stems[name].notes.get(&self.ms).cloned().unwrap_or(vec![]),
            playcount: stems[name]
                .notes
                .iter()
                .filter(|&(&ms, notes)| {
                    ms < self.ms
                        && !notes.is_empty()
                        && notes.iter().any(|note| note.is_on())
                })
                .count(),
        }
    }

    pub fn since_start(&self) -> Duration {
        Duration::from_millis(self.ms as _)
    }

    pub fn since_scene_start(&self) -> Option<Duration> {
        self.scene_started_at_ms
            .map(|start_ms| Duration::from_millis((self.ms - start_ms) as _))
    }

    pub fn notes_of_stem(&self, name: &str) -> impl Iterator<Item = Note> + '_ {
        let stem = &self.syncdata.stems[name];
        stem.notes
            .get(&self.ms)
            .into_iter()
            .flat_map(|notes| notes.iter().cloned())
    }

    pub fn dump_syncdata(&self, to: PathBuf) -> anyhow::Result<()> {
        Ok(serde_cbor::to_writer(fs::File::create(to)?, self.syncdata)?)
    }

    pub fn marker(&self) -> String {
        self.syncdata
            .markers
            .get(&self.ms)
            .map(|marker| marker.to_string())
            .unwrap_or_default()
    }

    pub fn latest_marker(&self) -> String {
        self.syncdata
            .markers
            .iter()
            .filter(|&(&ms, _)| ms <= self.ms)
            .max_by_key(|&(&ms, _)| ms)
            .map(|(_, marker)| marker.to_string())
            .unwrap_or_default()
    }

    pub fn duration_ms(&self) -> usize {
        match self.duration_override {
            Some(duration) => duration.as_millis() as _,
            None => self
                .syncdata
                .stems
                .values()
                .map(|stem| stem.duration_ms)
                .max()
                .unwrap(),
        }
    }

    pub fn later_frames(
        &mut self,
        delay: usize,
        render_function: &'static InnerHookRenderFunction,
    ) {
        let current_frame = self.frame();

        self.inner_hooks.insert(
            0,
            InnerHook {
                once: true,
                when: Box::new(move |_, context, _previous_beat| {
                    context.frame() >= current_frame + delay
                }),
                render_function: Box::new(render_function),
            },
        );
    }

    pub fn later_ms(
        &mut self,
        delay: usize,
        render_function: &'static InnerHookRenderFunction,
    ) {
        let current_ms = self.ms;

        self.inner_hooks.insert(
            0,
            InnerHook {
                once: true,
                when: Box::new(move |_, context, _previous_beat| {
                    context.ms >= current_ms + delay
                }),
                render_function: Box::new(render_function),
            },
        );
    }

    pub fn later_beats(
        &mut self,
        delay: f32,
        render_function: &'static InnerHookRenderFunction,
    ) {
        let current_beat = self.beat();

        self.inner_hooks.insert(
            0,
            InnerHook {
                once: true,
                when: Box::new(move |_, context, _previous_beat| {
                    context.beat_fractional() >= current_beat as f32 + delay
                }),
                render_function: Box::new(render_function),
            },
        );
    }

   

    pub fn switch_scene(&mut self, scene_name: impl Display) {
        self.current_scene = Some(scene_name.to_string());
        self.scene_started_at_ms = Some(self.ms);
    }
}
