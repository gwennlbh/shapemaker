use std::fmt::Display;
use serde::{Deserialize, Serialize};
use chrono::format;

#[derive(Serialize, Deserialize, Clone)]
pub struct Probe {
    pub id: u32,
    pub added_at: String,
    pub automation_name: String,
    pub midi_name: String,
    pub audio_name: String,
}

impl Probe {
    /// Returns a new probe with the `added_at` field set to the current time.
    pub fn with_added_at_now(&self) -> Self {
        return Self {
            added_at: chrono::Utc::now().to_rfc3339(),
            ..self.clone()
        };
    }
}

impl Display for Probe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "probe {} [", self.id)?;
        if !self.automation_name.is_empty() {
            write!(f, "automation \"{}\"", self.automation_name)?;
            if !self.midi_name.is_empty() || !self.audio_name.is_empty() {
                write!(f, " ")?;
            }
        }
        if !self.midi_name.is_empty() {
            write!(f, "midi \"{}\"", self.midi_name)?;
            if !self.audio_name.is_empty() {
                write!(f, " ")?;
            }
        }
        if !self.audio_name.is_empty() {
            write!(f, "audio \"{}\"", self.audio_name)?;
        }
        write!(f, "]")?;
        return Ok(());
    }
}
