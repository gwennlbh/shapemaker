use super::audio::Stem;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, ops::Range, path::PathBuf};

pub type TimestampMS = usize;

pub trait Syncable {
    fn new(path: impl Into<PathBuf>) -> Self;
    fn load(&self, progress: Option<&indicatif::ProgressBar>)
    -> Result<SyncData>;
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SyncData {
    pub stems: HashMap<String, Stem>,
    pub markers: HashMap<TimestampMS, String>,
    pub bpm: Option<usize>,
}

impl SyncData {
    pub fn merge_with(&mut self, other: SyncData) {
        self.bpm = other.bpm.or(self.bpm);
        self.stems.extend(other.stems);
        self.markers.extend(other.markers);
    }

    pub fn marker_ms_range(
        &self,
        marker: impl Display,
    ) -> Option<Range<TimestampMS>> {
        let start = self.markers.iter().find_map(|(&ms, m)| {
            if m == &marker.to_string() {
                Some(ms)
            } else {
                None
            }
        })?;

        let end = self.markers.iter().find_map(|(&ms, m)| {
            if m == &marker.to_string() && ms > start {
                Some(ms)
            } else {
                None
            }
        })?;

        Some(start..end)
    }
}
