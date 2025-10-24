use super::{beacon::connect_to_beacon, probe::Datapoint, Probe};
use anyhow::Result;
use nih_plug::params::FloatParam;
use std::{fmt::Display, net::TcpStream};
use tungstenite::{stream::MaybeTlsStream, WebSocket};

pub struct RemoteProbe {
    pub id: u32,
    pub out: WebSocket<MaybeTlsStream<TcpStream>>,
    pub pointsbuffer: Vec<Datapoint>,
}

impl RemoteProbe {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            out: connect_to_beacon().unwrap(),
            pointsbuffer: Vec::new(),
        }
    }

    pub fn register(&mut self) -> Result<()> {
        let probe = Probe {
            id: self.id,
            ..Default::default()
        };

        self.out
            .send(
                format!(
                    "? hi {}",
                    serde_json::to_string(&probe)
                        .expect("Failed to serialize probe")
                )
                .into(),
            )
            .expect("Failed to send register probe message");

        Ok(())
    }

    pub fn update(&mut self, probe: Probe) -> Result<()> {
        self.out
            .send(
                format!(
                    "{} hi {}",
                    self.id,
                    serde_json::to_string(&probe)
                        .expect("Failed to serialize probe")
                )
                .into(),
            )
            .expect("Failed to send update probe message");
        Ok(())
    }

    pub fn timestamp() -> usize {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize
    }

    /// Store a automation data point.
    pub fn store_automation(
        &mut self,
        timestamp: usize,
        param_id: usize,
        param: &FloatParam,
    ) -> Result<()> {
        self.store(Datapoint::Automation(timestamp, param_id, param.value()))
    }

    /// Store a audio data point.
    pub fn store_audio(
        &mut self,
        timestamp: usize,
        samples: Vec<f32>,
    ) -> Result<()> {
        self.store(Datapoint::Audio(timestamp, samples))
    }

    /// Store a midi data point.
    pub fn store_midi(&mut self, timestamp: usize, data: &[u8]) -> Result<()> {
        self.store(Datapoint::Midi(timestamp, data.to_vec()))
    }

    /// Store a data point.
    pub fn say(&mut self, msg: impl Display) -> Result<()> {
        self.out
            .write(format!("{} say {}", self.id, msg).into())
            .expect("Failed to send say message");
        Ok(())
    }

    pub fn store(&mut self, datapoint: Datapoint) -> Result<()> {
        self.pointsbuffer.push(datapoint);
        if self.pointsbuffer.len() >= 100 {
            self.say("flushing buffer of datapoints")?;
            for datapoint in self.pointsbuffer.drain(..) {
                self.out
                    .write(
                        format!(
                            "{} {}",
                            self.id,
                            match &datapoint {
                                Datapoint::Automation(ts, param_id, value) => {
                                    format!("{ts} % {} {}", param_id, value)
                                }
                                Datapoint::Midi(ts, data) => {
                                    format!(
                                        "{ts} # 0 {}",
                                        data.iter()
                                            .map(|b| b.to_string())
                                            .collect::<Vec<String>>()
                                            .join(" ")
                                    )
                                }
                                Datapoint::Audio(ts, data) => {
                                    format!(
                                        "{ts} ~ {}",
                                        data.iter()
                                            .map(|f| f.to_string())
                                            .collect::<Vec<String>>()
                                            .join(" ")
                                    )
                                }
                            }
                        )
                        .into(),
                    )
                    .expect("Failed to send store datapoint message");
            }
            self.out.flush().expect("Failed to flush probe connection");
        }

        Ok(())
    }
}

impl Drop for RemoteProbe {
    fn drop(&mut self) {
        self.out
            .close(None)
            .expect("Failed to close probe connection");
    }
}
