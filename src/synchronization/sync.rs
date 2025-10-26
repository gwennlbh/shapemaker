use super::audio::Stem;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

pub type TimestampMS = usize;

pub trait Syncable {
    fn new(path: impl Into<PathBuf>) -> Self;
    fn load(&self, progress: Option<&indicatif::ProgressBar>) -> SyncData;
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SyncData {
    pub stems: HashMap<String, Stem>,
    pub markers: HashMap<TimestampMS, String>,
    pub bpm: usize,
}

impl SyncData {
    pub fn union(self, other: SyncData) -> Self {
        let mut combined = Self::default();

        combined.bpm = other.bpm;

        combined.stems.extend(self.stems);
        combined.stems.extend(other.stems);

        combined.markers.extend(self.markers);
        combined.markers.extend(other.markers);

        combined
    }
}
