use super::Video;
use crate::ui::{self, Log};
use crate::video::encoders::Encoder;
use crate::video::encoders::vgv::VGVTranscodeMode;
use crate::video::engine::EngineOutput;
use anyhow::Result;
use measure_time::debug_time;
use std::path::PathBuf;
use std::thread;

impl<AdditionalContext: Default> Video<AdditionalContext> {
    pub fn encode(&mut self, output_file: impl Into<PathBuf>) -> Result<()> {
        debug_time!("encode");

        let encoder = self.setup_encoder(output_file)?;

        self.encode_with(encoder)
    }

    fn setup_encoder(
        &mut self,
        output_path: impl Into<PathBuf>,
    ) -> Result<Box<dyn Encoder + Send>> {
        let (width, height) =
            self.initial_canvas.resolution_to_size_even(self.resolution);

        let destination = output_path.into();

        if destination.exists() {
            std::fs::remove_file(&destination)?;
        }

        std::fs::create_dir_all(
            &destination
                .parent()
                .expect("Given output file has no parent"),
        )?;

        Ok(match destination.full_extension() {
            ".vgv.html" => {
                self.progress_bars.encoding.log(
                    "Selecting",
                    &format!(
                        "VGV encoder with HTML transcoding as {} ends with .vgv.html",
                        ui::format_filepath(&destination),
                    ),
                );

                Box::new(self.setup_vgv_encoder(
                    VGVTranscodeMode::ToHTML,
                    width as _,
                    height as _,
                    &self.initial_canvas,
                    destination,
                )?)
            }
            ".vgv" => {
                self.progress_bars.encoding.log(
                    "Selecting",
                    &format!(
                        "VGV encoder as {} ends with .vgv (use .vgv.html for HTML transcoding)",
                        ui::format_filepath(&destination),
                    ),
                );

                Box::new(self.setup_vgv_encoder(
                    VGVTranscodeMode::None,
                    width as _,
                    height as _,
                    &self.initial_canvas,
                    destination,
                )?)
            }
            _ => {
                self.progress_bars
                    .encoding
                    .log("Selecting", &format!(
                        "FFMpeg encoder, as {} doesn't end with .vgv or .vgv.html",
                        ui::format_filepath(&destination),
                    ));
                self.initial_canvas.load_fonts()?;
                Box::new(self.setup_ffmpeg_encoder(width, height, destination)?)
            }
        })
    }

    pub fn encode_with(
        &mut self,
        mut encoder: Box<dyn Encoder + Send>,
    ) -> Result<()> {
        debug_time!("encode_with");

        self.progress.remove(&self.progress_bars.loading);

        let pb = self.progress_bars.encoding.clone();

        pb.set_length(self.ms_to_frames(self.duration_ms()) as _);
        pb.set_message("");

        let (tx, rx) = std::sync::mpsc::sync_channel::<EngineOutput>(1_000);

        let encoder_thread = thread::spawn(move || {
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

                encoder.encode_frame(output).expect("Couldn't encode frame");
            }

            let finish_message = encoder.finish_message(pb.elapsed());

            encoder.finish().expect("Couldn't finish encoding");

            pb.finish();
            pb.log("Encoded", &finish_message);
        });

        self.render_with_overrides(tx)?;

        encoder_thread.join().expect("Encoder thread panicked");

        self.progress.clear().unwrap();

        Ok(())
    }

    #[allow(dead_code)]
    fn add_audio_track(&mut self, _output_file: String) -> Result<()> {
        todo!(
            "Look into https://github.com/zmwangx/rust-ffmpeg/blob/master/examples/transcode-x264.rs and maybe contribute to video-rs (see https://github.com/oddity-ai/video-rs/issues/44)"
        );
    }
}

// Because .extension() sucks

trait FullExtension {
    fn full_extension(&self) -> &str;
}

impl FullExtension for PathBuf {
    fn full_extension(&self) -> &str {
        let filename = self
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or_default();
        let parts: Vec<&str> = filename.split('.').collect();
        if parts.len() <= 1 {
            ""
        } else {
            &filename[filename.find('.').unwrap()..]
        }
    }
}
