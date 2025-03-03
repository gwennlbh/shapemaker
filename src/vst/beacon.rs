use std::fmt::Display;

use serde::{Deserialize, Serialize};
use ureq::http::Uri;

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

const BEACON_PORT: u16 = 8080;

#[inline]
pub fn beacon_url() -> Uri {
    return format!("http://localhost:{BEACON_PORT}")
        .parse()
        .expect("Invalid default beacon URL");
}

pub mod requests {
    use anyhow::Result;
    use ureq::{http::Response, Body};

    use super::{beacon_url, Probe};

    pub fn register_probe(probe: Probe) -> Result<Response<Body>> {
        return Ok(
            ureq::put(format!("{}/probes/{}", beacon_url(), probe.id).as_str())
                .content_type("application/json")
                .send(
                    serde_json::to_string(&probe)
                        .expect("Failed to serialize probe")
                        .as_bytes(),
                )?,
        );
    }

    pub fn unregister_probe(id: u32) -> Result<Response<Body>> {
        return Ok(ureq::delete(format!("{}/probes/{}", beacon_url(), id).as_str()).call()?);
    }
}

pub mod server {
    use std::{
        str::FromStr,
        sync::{Mutex, MutexGuard},
    };

    use super::{Probe, BEACON_PORT};
    use anyhow::Result;
    use futures_util::{future, FutureExt};
    use gotham::{
        self,
        helpers::http::response::{create_empty_response, create_response},
        hyper::{body, Body, StatusCode},
        mime::Mime,
        prelude::{DefineSingleRoute, DrawRoutes, StaticResponseExtender},
        router::build_simple_router,
        state::{FromState, State, StateData},
    };
    use once_cell::sync::Lazy;
    use serde::Deserialize;

    #[derive(Default)]
    pub struct Beacon {
        pub probes: Vec<Probe>,
    }

    static BEACON: Lazy<Mutex<Beacon>> = Lazy::new(|| Mutex::new(Beacon::default()));

    pub fn get_beacon() -> MutexGuard<'static, Beacon> {
        return BEACON.lock().unwrap();
    }

    #[derive(Debug, Deserialize, StateData, StaticResponseExtender)]
    struct PathExtractorId {
        id: u32,
    }

    impl Beacon {
        pub fn new() -> Self {
            return Self::default();
        }

        pub fn start() -> Result<()> {
            let router = build_simple_router(|route| {
                route.get("/probes").to(|state| {
                    let beacon = get_beacon();
                    let body =
                        serde_json::to_string(&beacon.probes).expect("Failed to serialize probes");
                    let response = create_response(
                        &state,
                        StatusCode::OK,
                        Mime::from_str("application/json").unwrap(),
                        body,
                    );
                    (state, response)
                });
                route
                    .delete("/probes/:id")
                    .with_path_extractor::<PathExtractorId>()
                    .to(|state: State| {
                        let id = state.borrow::<PathExtractorId>().id;
                        let mut beacon = get_beacon();
                        println!("Beacon: removing probe {id}");
                        let probe_index = beacon.probes.iter().position(|probe| probe.id == id);
                        match probe_index {
                            Some(probe_index) => {
                                let removed_probe = beacon.probes.remove(probe_index);
                                println!("Beacon: removed {removed_probe}");
                                let response =
                                    create_empty_response(&state, StatusCode::NO_CONTENT);
                                (state, response)
                            }
                            None => {
                                println!("Beacon: probe {id} not found");
                                let response = create_empty_response(&state, StatusCode::NOT_FOUND);
                                (state, response)
                            }
                        }
                    });
                route
                    .put("/probes/:id")
                    .with_path_extractor::<PathExtractorId>()
                    .to(|mut state: State| {
                        let id = state.borrow::<PathExtractorId>().id;
                        let f =
                            body::to_bytes(Body::take_from(&mut state)).then(move |full_body| {
                                match full_body {
                                    Ok(valid_body) => {
                                        let content = String::from_utf8(valid_body.to_vec())
                                            .expect("Failed to parse body");
                                        match serde_json::from_str::<Probe>(&content) {
                                            Ok(new_probe) => {
                                                if new_probe.id != id {
                                                    return future::err((
                                                        state,
                                                        anyhow::anyhow!(
                                                            "Probe ID does not match URL ID"
                                                        )
                                                        .into(),
                                                    ));
                                                }

                                                println!("Beacon: registering {new_probe}");
                                                let mut beacon = get_beacon();
                                                beacon.probes.push(new_probe);
                                                let response = create_empty_response(
                                                    &state,
                                                    StatusCode::CREATED,
                                                );
                                                future::ok((state, response))
                                            }
                                            Err(_) => future::err((
                                                state,
                                                anyhow::anyhow!("Failed to parse probe").into(),
                                            )),
                                        }
                                    }
                                    Err(e) => future::err((state, e.into())),
                                }
                            });
                        f.boxed()
                    });
            });
            println!("Starting beacon server on port {}", BEACON_PORT);
            gotham::start(format!("127.0.0.1:{BEACON_PORT}"), router).map_err(|e| e.into())
        }
    }
}
