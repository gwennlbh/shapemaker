use crate::synchronization::sync::Syncable;
use std::{collections::HashMap, fs::File, path::PathBuf};
use symphonia::core::{formats::FormatReader, io::MediaSourceStream};
use symphonia::default::formats::{FlacReader, WavReader};

use super::sync::TimestampMS;

pub struct CueMarkersSynchronizer {
    pub path: PathBuf,
}

impl Syncable for CueMarkersSynchronizer {
    fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    fn load(
        &self,
        progress: Option<&indicatif::ProgressBar>,
    ) -> super::sync::SyncData {
        let markers: HashMap<TimestampMS, String> = HashMap::new();

        let file = File::open(&self.path)
            .expect(&format!("Failed to open {:?} for CUE analysis", self.path));
        let stream = MediaSourceStream::new(Box::new(file), Default::default());
        let reader: Box<dyn FormatReader> =
            match self.path.extension().and_then(|s| s.to_str()) {
                Some("wav") => Box::new(
                    WavReader::try_new(stream, &Default::default())
                        .expect("Failed to create WAV reader for CUE analysis"),
                ),
                Some("flac") => Box::new(
                    FlacReader::try_new(stream, &Default::default())
                        .expect("Failed to create FLAC reader for CUE analysis"),
                ),
                _ => panic!("Unsupported audio format for CUE analysis"),
            };

        for cue in reader.cues() {
            panic!("Found cue {cue:?}");
            if let Some(pb) = progress {
                pb.set_message(format!("{cue:?}"));
            }
        }

        super::sync::SyncData {
            stems: HashMap::new(),
            markers,
            bpm: 120,
        }
    }
}
