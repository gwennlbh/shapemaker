extern crate env_logger;

use crate::vst::probe::Datapoint;

use super::Probe;
use anyhow::Result;
use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};
use tungstenite;

const BEACON_PORT: u16 = 8080;

#[inline]
pub fn beacon_url() -> String {
    return format!("ws://localhost:{BEACON_PORT}");
}

pub fn connect_to_beacon() -> Result<
    tungstenite::WebSocket<
        tungstenite::stream::MaybeTlsStream<std::net::TcpStream>,
    >,
> {
    println!("Connecting to beacon at {}", beacon_url());
    let (socket, _response) = tungstenite::connect(beacon_url())?;
    Ok(socket)
}

#[derive(Default)]
pub struct Beacon {
    pub probes: Vec<Probe>,
}

static BEACON: Lazy<Mutex<Beacon>> = Lazy::new(|| Mutex::new(Beacon::new()));

pub fn get_beacon() -> MutexGuard<'static, Beacon> {
    return BEACON.lock().unwrap();
}

impl Beacon {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn start() -> Result<()> {
        ws::listen(format!("127.0.0.1:{BEACON_PORT}"), |out| {
            println!("Opening beacon connection with a probe...");
            move |msg| {
                println!("Received message: {:?}", msg);
                match msg {
                    ws::Message::Text(text) => match text.split3() {
                        ("?", "hi", probe_json) => {
                            match serde_json::from_str::<Probe>(probe_json) {
                                Ok(probe) => {
                                    let mut beacon = get_beacon();
                                    beacon.probes.push(probe);
                                    out.send("{} probe added!")
                                }
                                Err(_) => out.send("? invalid JSON :/"),
                            }
                        }
                        (id_str, "hi", probe_json) => {
                            match serde_json::from_str::<Probe>(probe_json) {
                                Ok(probe) => {
                                    let mut beacon = get_beacon();
                                    let probe_index =
                                        beacon.probes.iter().position(|p| {
                                            p.id == id_str.parse::<u32>().unwrap()
                                        });
                                    if let None = probe_index {
                                        return out.send(format!(
                                            "{} not found :/",
                                            probe.id
                                        ));
                                    }
                                    beacon.probes[probe_index.unwrap()] = probe;
                                    out.send("{} probe added!")
                                }
                                Err(_) => out.send("? invalid JSON :/"),
                            }
                        }
                        (id_str, "byebye", "") => {
                            let id = id_str.parse::<u32>().unwrap();
                            let mut beacon = get_beacon();
                            let probe_index = beacon
                                .probes
                                .iter()
                                .position(|probe| probe.id == id);
                            match probe_index {
                                Some(probe_index) => {
                                    let removed_probe =
                                        beacon.probes.remove(probe_index);
                                    out.send(format!(
                                        "{} probe removed!",
                                        removed_probe.id
                                    ))
                                }
                                None => out.send(format!("{id} not found :/")),
                            }
                        }
                        ("*", "wtf", "") => {
                            let beacon = get_beacon();
                            let body =
                                serde_json::to_string(&beacon.probes).unwrap();
                            out.send(body)
                        }
                        (id_str, "wtf", "") => {
                            let id = id_str.parse::<u32>().unwrap();
                            let beacon = get_beacon();
                            let probe =
                                beacon.probes.iter().find(|probe| probe.id == id);
                            match probe {
                                Some(probe) => {
                                    out.send(format!(
                                        "probe {} with {} datapoints stored",
                                        probe.id,
                                        probe.datapoints.len()
                                    ))?;
                                    out.send(
                                        serde_json::to_string(probe).unwrap(),
                                    )
                                }
                                None => out.send(format!("{id} not found :/")),
                            }
                        }
                        (id_str, "say", msg) => {
                            let id = id_str.parse::<u32>().unwrap();
                            let beacon = get_beacon();
                            let probe =
                                beacon.probes.iter().find(|probe| probe.id == id);
                            match probe {
                                Some(probe) => {
                                    println!("probe {}: {}", probe.id, msg);
                                    out.send("ok")
                                }
                                None => out.send(format!("{id} not found :/")),
                            }
                        }
                        (probe_id, timestamp, msg) => {
                            let id = probe_id.parse::<u32>().unwrap();
                            let mut beacon = get_beacon();
                            let probe = beacon
                                .probes
                                .iter_mut()
                                .find(|probe| probe.id == id);

                            if let None = probe {
                                return out.send(format!("{id} not found :/"));
                            }

                            let probe = probe.unwrap();
                            let timestamp: usize =
                                timestamp.parse().expect(&format!(
                                    "{timestamp} to be a number (timestamp)"
                                ));

                            match msg.split2() {
                                ("%", data) => match data.split2() {
                                    (paramname, paramvalue) => {
                                        probe.store(Datapoint::Automation(
                                            timestamp,
                                            paramname.parse().expect(&format!(
                                                "{paramname} to be a number"
                                            )),
                                            paramvalue.parse().expect(&format!(
                                                "{paramvalue} to be a number"
                                            )),
                                        ))
                                    }
                                },
                                ("#", data) => probe.store(Datapoint::Midi(
                                    timestamp,
                                    data.as_bytes().to_vec(),
                                )),
                                ("~", data) => probe.store(Datapoint::Audio(
                                    timestamp,
                                    data.split(' ')
                                        .map(|f| f.parse().unwrap())
                                        .collect(),
                                )),
                                _ => {
                                    return out.send("invalid command :/");
                                }
                            }

                            out.send("gotchu!")
                        }
                    },
                    ws::Message::Binary(_) => todo!(),
                }
            }
        })?;
        Ok(())
    }
}

trait FixedSplits {
    fn split3(&self) -> (&str, &str, &str);
    fn split2(&self) -> (&str, &str);
}
impl FixedSplits for str {
    fn split3(&self) -> (&str, &str, &str) {
        let mut parts = self.splitn(3, ' ');
        let first = parts.next().unwrap_or_default();
        let second = parts.next().unwrap_or_default();
        let third = parts.next().unwrap_or_default();
        return (first, second, third);
    }

    fn split2(&self) -> (&str, &str) {
        let mut parts = self.splitn(2, ' ');
        let first = parts.next().unwrap_or_default();
        let second = parts.next().unwrap_or_default();
        return (first, second);
    }
}
