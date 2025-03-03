extern crate env_logger;
extern crate ws;

use super::Probe;
use anyhow::Result;
use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

const BEACON_PORT: u16 = 8080;

#[inline]
pub fn beacon_url() -> String {
    return format!("ws://localhost:{BEACON_PORT}");
}

pub fn connect_to_beacon<T: FnMut(&ws::Sender) -> ()>(mut action: T) -> Result<()> {
    ws::connect(beacon_url(), |out| {
        action(&out);

        move |_msg| out.close(ws::CloseCode::Normal)
    })?;
    Ok(())
}

pub fn register_probe(probe: Probe) -> Result<()> {
    connect_to_beacon(|beacon| {
        beacon
            .send(format!(
                "+ probe {}",
                serde_json::to_string(&probe).expect("Failed to serialize probe")
            ))
            .expect("Failed to send register probe message");
    })?;
    Ok(())
}

pub fn unregister_probe(id: u32) -> Result<()> {
    connect_to_beacon(|beacon| {
        beacon
            .send(format!("- probe {}", id))
            .expect("Failed to send unregister probe message");
    })?;
    Ok(())
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
            move |msg| match msg {
                ws::Message::Text(text) => match split3(&text) {
                    ("+", "probe", probe_json) => match serde_json::from_str::<Probe>(probe_json) {
                        Ok(probe) => {
                            let mut beacon = get_beacon();
                            beacon.probes.push(probe);
                            out.send("^ probe added")
                        }
                        Err(_) => out.send("! probe invalid JSON"),
                    },
                    ("-", "probe", id_str) => {
                        let id = id_str.parse::<u32>().unwrap();
                        let mut beacon = get_beacon();
                        let probe_index = beacon.probes.iter().position(|probe| probe.id == id);
                        match probe_index {
                            Some(probe_index) => {
                                let removed_probe = beacon.probes.remove(probe_index);
                                out.send(format!("^ probe {} removed", removed_probe.id))
                            }
                            None => out.send(format!("! probe {id} not found")),
                        }
                    }
                    ("=", "probe", "*") => {
                        let beacon = get_beacon();
                        let body = serde_json::to_string(&beacon.probes).unwrap();
                        out.send(body)
                    }
                    ("=", "probe", id_str) => {
                        let id = id_str.parse::<u32>().unwrap();
                        let beacon = get_beacon();
                        let probe = beacon.probes.iter().find(|probe| probe.id == id);
                        match probe {
                            Some(probe) => out.send(serde_json::to_string(probe).unwrap()),
                            None => out.send(format!("! probe {id} not found")),
                        }
                    }
                    _ => out.send("! invalid command"),
                },
                ws::Message::Binary(_) => todo!(),
            }
        })?;
        Ok(())
    }
}

fn split3(subject: &str) -> (&str, &str, &str) {
    let mut parts = subject.splitn(3, ' ');
    let first = parts.next().unwrap_or_default();
    let second = parts.next().unwrap_or_default();
    let third = parts.next().unwrap_or_default();
    return (first, second, third);
}
