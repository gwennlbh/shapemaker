extern crate env_logger;
extern crate ws;

use super::Probe;
use anyhow::Result;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::{Mutex, MutexGuard};

const BEACON_PORT: u16 = 8080;

#[inline]
pub fn beacon_url() -> String {
    return format!("ws://localhost:{BEACON_PORT}");
}

struct BeaconClient {
    ws: ws::Sender,
}

impl BeaconClient {
    pub fn new() -> Result<Self> {
        let ws = ws::connect(beacon_url(), |out| BeaconClient { ws: out })?;
        return Ok(Self { ws });
    }

    pub fn register_probe(&self, probe: Probe) -> Result<()> {
        return Ok(self.ws.send(format!(
            "+ probe {}",
            serde_json::to_string(&probe).expect("Failed to serialize probe")
        ))?);
    }

    pub fn unregister_probe(&self, id: u32) -> Result<()> {
        return Ok(self.ws.send(format!("- probe {}", id))?);
    }
}

#[derive(Default)]
pub struct Beacon {
    pub probes: Vec<Probe>,
}

static BEACON: Lazy<Mutex<Beacon>> = Lazy::new(|| Mutex::new(Beacon::default()));

pub fn get_beacon() -> MutexGuard<'static, Beacon> {
    return BEACON.lock().unwrap();
}

impl Beacon {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn start() -> Result<()> {
        // let router = build_simple_router(|route| {
        //     route.get("/probes").to(|state| {
        //         let beacon = get_beacon();
        //         let body =
        //             serde_json::to_string(&beacon.probes).expect("Failed to serialize probes");
        //         let response = create_response(
        //             &state,
        //             StatusCode::OK,
        //             Mime::from_str("application/json").unwrap(),
        //             body,
        //         );
        //         (state, response)
        //     });
        //     route
        //         .delete("/probes/:id")
        //         .with_path_extractor::<PathExtractorId>()
        //         .to(|state: State| {
        //             let id = state.borrow::<PathExtractorId>().id;
        //             let mut beacon = get_beacon();
        //             println!("Beacon: removing probe {id}");
        //             let probe_index = beacon.probes.iter().position(|probe| probe.id == id);
        //             match probe_index {
        //                 Some(probe_index) => {
        //                     let removed_probe = beacon.probes.remove(probe_index);
        //                     println!("Beacon: removed {removed_probe}");
        //                     let response = create_empty_response(&state, StatusCode::NO_CONTENT);
        //                     (state, response)
        //                 }
        //                 None => {
        //                     println!("Beacon: probe {id} not found");
        //                     let response = create_empty_response(&state, StatusCode::NOT_FOUND);
        //                     (state, response)
        //                 }
        //             }
        //         });
        //     route
        //         .put("/probes/:id")
        //         .with_path_extractor::<PathExtractorId>()
        //         .to(|mut state: State| {
        //             let id = state.borrow::<PathExtractorId>().id;
        //             let f = body::to_bytes(Body::take_from(&mut state)).then(move |full_body| {
        //                 match full_body {
        //                     Ok(valid_body) => {
        //                         let content = String::from_utf8(valid_body.to_vec())
        //                             .expect("Failed to parse body");
        //                         match serde_json::from_str::<Probe>(&content) {
        //                             Ok(new_probe) => {
        //                                 if new_probe.id != id {
        //                                     return future::err((
        //                                         state,
        //                                         anyhow::anyhow!("Probe ID does not match URL ID")
        //                                             .into(),
        //                                     ));
        //                                 }

        //                                 println!("Beacon: registering {new_probe}");
        //                                 let mut beacon = get_beacon();
        //                                 beacon.probes.push(new_probe);
        //                                 let response =
        //                                     create_empty_response(&state, StatusCode::CREATED);
        //                                 future::ok((state, response))
        //                             }
        //                             Err(_) => future::err((
        //                                 state,
        //                                 anyhow::anyhow!("Failed to parse probe").into(),
        //                             )),
        //                         }
        //                     }
        //                     Err(e) => future::err((state, e.into())),
        //                 }
        //             });
        //             f.boxed()
        //         });
        // });
        // println!("Starting beacon server on port {}", BEACON_PORT);
        // gotham::start(format!("127.0.0.1:{BEACON_PORT}"), router).map_err(|e| e.into())
    }
}
