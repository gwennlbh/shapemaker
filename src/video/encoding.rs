use super::Video;
use crate::Timestamp;
use crate::context::Context;
use crate::ui::{Log, Pretty};
use crate::video::encoders::Encoder;
use crate::video::engine::{EngineControl, EngineController, EngineOutput};
use anyhow::{Result, anyhow};
use measure_time::debug_time;
use std::path::PathBuf;
use std::thread;

impl<C: Default> Video<C> {
    pub fn encode(
        &mut self,
        output_file: impl Into<PathBuf> + Clone,
    ) -> Result<std::time::Duration> {
        let actual_ms_range = self.constrained_ms_range();
        if actual_ms_range != self.total_ms_range() {
            self.progress_bars.rendering.log(
                "Constrained",
                &Timestamp::from_ms_range(&actual_ms_range).pretty(),
            );
        }

        self.encode_controlled(output_file, &move |ctx| {
            if actual_ms_range.contains(&ctx.ms) {
                EngineControl::Render
            } else if ctx.ms > actual_ms_range.end {
                EngineControl::Stop
            } else {
                EngineControl::Skip
            }
        })
    }

    pub fn encode_controlled(
        &mut self,
        output_file: impl Into<PathBuf> + Clone,
        engine_controller: &EngineController<C>,
    ) -> Result<std::time::Duration> {
        debug_time!("encode");

        let encoder = self.setup_encoder(output_file.clone())?;
        let encoder_name = encoder.name();

        let time_taken = self.encode_with(encoder, engine_controller)?;

        let _ = notify_rust::Notification::new()
            .appname("Shapemaker")
            .summary(&format!("{} is ready", &output_file.into().pretty()))
            .body(&format!(
                "Encoded with {encoder_name} in {}",
                time_taken.pretty()
            ))
            .show();

        Ok(time_taken)
    }

    pub fn encode_with(
        &mut self,
        mut encoder: Box<dyn Encoder + Send>,
        engine_controller: &EngineController<C>,
    ) -> Result<std::time::Duration> {
        debug_time!("encode_with");

        self.progress.remove(&self.progress_bars.loading);

        let pb = self.progress_bars.encoding.clone();

        pb.set_length(self.ms_to_frames(self.duration_ms()) as _);
        pb.set_message("");

        let (tx, rx) = std::sync::mpsc::sync_channel::<EngineOutput>(1_000);

        let encoder_thread =
            thread::spawn(move || -> Result<std::time::Duration> {
                for output in rx.iter() {
                    match output {
                        EngineOutput::Finished => break,
                        EngineOutput::Frame { .. } => {
                            pb.inc(1);
                            pb.set_message(encoder.progress_message(
                                pb.position(),
                                pb.length().unwrap(),
                            ));
                        }
                    }

                    encoder.encode_frame(output)?;
                }

                let time_taken = pb.elapsed();
                let finish_message = encoder.finish_message(time_taken);

                encoder.finish()?;

                pb.finish();
                pb.log("Encoded", &finish_message);

                Ok(time_taken)
            });

        self.render(tx, engine_controller)?;

        let time_taken = encoder_thread
            .join()
            .map_err(|e| anyhow!("Encoder thread panicked: {e:?}"))
            .flatten()?;

        let _ = self.progress.clear();

        Ok(time_taken)
    }

    #[allow(dead_code)]
    fn add_audio_track(&mut self, _output_file: String) -> Result<()> {
        todo!(
            "Look into https://github.com/zmwangx/rust-ffmpeg/blob/master/examples/transcode-x264.rs and maybe contribute to video-rs (see https://github.com/oddity-ai/video-rs/issues/44)"
        );
    }
}
