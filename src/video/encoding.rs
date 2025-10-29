use super::Video;
use crate::ui::{Log, Pretty};
use crate::video::encoders::Encoder;
use crate::video::encoders::vgv::VGVTranscodeMode;
use crate::video::engine::EngineOutput;
use anyhow::{Result, anyhow};
use measure_time::debug_time;
use std::path::PathBuf;
use std::thread;

impl<AdditionalContext: Default> Video<AdditionalContext> {
    pub fn encode(
        &mut self,
        output_file: impl Into<PathBuf> + Clone,
    ) -> Result<std::time::Duration> {
        debug_time!("encode");

        let destination = output_file.into();

        let encoder = self.setup_encoder(destination.clone())?;

        let encoder_name = encoder.name();

        let encoding_result = self.encode_with(encoder);

        let file_to_open = match encoding_result {
            Err(_) => None,
            Ok(_) => Some(destination.clone()),
        };

        Self::send_notification(
            match encoding_result {
                Err(_) => format!("{} failed", destination.pretty()),
                Ok(_) => format!("{} is ready", destination.pretty()),
            },
            match encoding_result {
                Err(ref e) => format!("{e:?}"),
                Ok(time_taken) => format!(
                    "Encoded with {encoder_name} in {}",
                    time_taken.pretty()
                ),
            },
            move || {
                if let Some(ref file_to_open) = file_to_open {
                    let _ = ::open::that_detached(file_to_open);
                }
            },
        );

        let _ = self.progress.clear();

        encoding_result
    }

    fn setup_encoder(
        &mut self,
        output_path: impl Into<PathBuf>,
    ) -> Result<Box<dyn Encoder + Send>> {
        let (width, height) =
            self.initial_canvas.resolution_to_size_even(self.resolution);

        let destination = output_path.into();
        let pb = &self.progress_bars.encoding;

        if destination.exists() {
            // FIXME does not block at all lol (on Windows at least)
            // pb.log(
            //     "Blocking",
            //     &format!("on the output file {}", destination.pretty()),
            // );

            // std::fs::OpenOptions::new()
            //     .write(true)
            //     .open(&destination)?
            //     .lock()?;

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
                        destination.pretty(),
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
                        destination.pretty(),
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
                pb.log(
                    "Selecting",
                    &format!(
                        "FFMpeg encoder as {} ends with {}",
                        destination.pretty(),
                        destination.full_extension()
                    ),
                );

                self.initial_canvas.load_fonts()?;
                Box::new(self.setup_ffmpeg_encoder(width, height, destination)?)
            }
        })
    }

    pub fn encode_with(
        &mut self,
        mut encoder: Box<dyn Encoder + Send>,
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

        self.render_with_overrides(tx)?;

        let time_taken = encoder_thread
            .join()
            .map_err(|e| anyhow!("Encoder thread panicked: {e:?}"))
            .flatten()?;

        Ok(time_taken)
    }

    #[allow(dead_code)]
    fn add_audio_track(&mut self, _output_file: String) -> Result<()> {
        todo!(
            "Look into https://github.com/zmwangx/rust-ffmpeg/blob/master/examples/transcode-x264.rs and maybe contribute to video-rs (see https://github.com/oddity-ai/video-rs/issues/44)"
        );
    }

    // TODO contribute to notify-rust instead

    #[cfg(target_os = "windows")]
    fn send_notification(
        title: String,
        subtitle: String,
        on_click: impl Fn() + Send + 'static,
    ) {
        // TODO
        // let aum = winrt_toast::register(aum_id, display_name, icon_path)
        let mut toast = winrt_toast::Toast::new();
        toast.text1(title).text2(subtitle).text3("Shapemaker");

        let manager = winrt_toast::ToastManager::new(
            r#"{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe"#,
        );
        manager
            .show_with_callbacks(
                &toast,
                Some(Box::new(move |_| on_click())),
                None,
                Some(Box::new(move |e| eprintln!("Failed to show toast {e:?}"))),
            )
            .expect("Failed to prepare toast");
    }

    #[cfg(not(target_os = "windows"))]
    fn send_notification(
        title: String,
        subtitle: String,
        _on_click: impl FnOnce() + Send + 'static,
    ) {
        // TODO on_click
        let _ = notify_rust::Notification::new()
            .appname("Shapemaker")
            .summary(&title)
            .body(&subtitle)
            .show();
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
