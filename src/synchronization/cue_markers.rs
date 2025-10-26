use crate::synchronization::sync::Syncable;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::{collections::HashMap, io::Read, path::PathBuf, process::Stdio};

use super::sync::TimestampMS;

pub struct CueMarkersSynchronizer {
    pub path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct FFprobeChapterTags {
    title: String,
}

#[derive(Debug, Deserialize)]
struct FFprobeChapter {
    // id: usize,
    // time_base: String,

    // start: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    start_time: f32,

    // end: usize,
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    // end_time: f32,
    tags: FFprobeChapterTags,
}

#[derive(Debug, Deserialize)]
struct FFprobeOutput {
    chapters: Vec<FFprobeChapter>,
}

impl Syncable for CueMarkersSynchronizer {
    fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    fn load(
        &self,
        progress: Option<&indicatif::ProgressBar>,
    ) -> super::sync::SyncData {
        let mut ffprobe = std::process::Command::new("ffprobe")
            .args(["-v", "error"])
            .args(["-i", &self.path.to_string_lossy()])
            .args(["-output_format", "json"])
            .arg("-show_chapters")
            .stdout(Stdio::piped())
            .spawn()
            .expect(&format!(
                "Couldn't run ffprobe to get chapters of {:?}",
                self.path
            ));

        let mut raw_output = String::new();
        ffprobe
            .stdout
            .take()
            .expect("Coudln't get stdout of ffprobe run")
            .read_to_string(&mut raw_output)
            .expect("Couldn't read ffprobe stdout");

        let output: FFprobeOutput =
            serde_json::from_str(&raw_output).expect("Invalid ffprobe output");

        super::sync::SyncData {
            stems: HashMap::new(),
            bpm: None,
            markers: output
                .chapters
                .iter()
                .map(|ch| {
                    (
                        (ch.start_time.to_owned() * 1_000.0) as TimestampMS,
                        ch.tags.title.clone(),
                    )
                })
                .collect(),
        }
    }
}
