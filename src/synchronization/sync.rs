use super::audio::Stem;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type TimestampMS = usize;

pub trait Syncable {
    fn new(path: &str) -> Self;
    fn load(&self, progress: Option<&indicatif::ProgressBar>) -> SyncData;
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SyncData {
    pub stems: HashMap<String, Stem>,
    pub markers: HashMap<TimestampMS, String>,
    pub bpm: usize,
}
