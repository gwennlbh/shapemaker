use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Serialize, Deserialize, Clone)]
pub struct Probe {
    pub id: u32,
    pub added_at: String,
    /// Maps automation parameter indices to their names
    pub automation_names: HashMap<usize, String>,
    /// MIDI In signal name
    pub midi_name: String,
    /// Audio In signal name
    pub audio_name: String,

    #[serde(skip)]
    pub datapoints: Vec<Datapoint>,
}

#[derive(Clone)]
pub enum Datapoint {
    Automation(usize, usize, f32),
    Midi(usize, Vec<u8>),
    Audio(usize, Vec<f32>),
}

impl Probe {
    /// Returns a new probe with the `added_at` field set to the current time.
    pub fn with_added_at_now(&self) -> Self {
        return Self {
            added_at: chrono::Utc::now().to_rfc3339(),
            ..self.clone()
        };
    }

    pub fn store(&mut self, datapoint: Datapoint) {
        self.datapoints.push(datapoint);
    }
}

impl Display for Probe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "probe {} [", self.id)?;
        if !self.automation_names.is_empty() {
            write!(f, "automations {:?}", self.automation_names)?;
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

impl Default for Probe {
    fn default() -> Self {
        Self {
            id: 0,
            audio_name: String::new(),
            midi_name: String::new(),
            added_at: chrono::Utc::now().to_rfc3339(),
            datapoints: Vec::new(),
            automation_names: HashMap::new(),
        }
    }
}
