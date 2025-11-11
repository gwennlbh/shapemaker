use crate::{
    Video,
    ui::{Log, Pretty},
    video::{encoders::vgv::VGVTranscodeMode, engine::EngineOutput},
};
use anyhow::Result;
use itertools::Chunk;
use rayon::iter::ParallelIterator;
use std::path::PathBuf;

pub mod ffmpeg;
pub mod vgv;

pub trait Encoder {
    fn name(&self) -> String;
    fn encode_frames(
        &mut self,
        outputs: Vec<EngineOutput>,
    ) -> Result<std::ops::ControlFlow<()>>;
    fn finish(&mut self) -> Result<()>;
    fn finish_message(&self, time_elapsed: std::time::Duration) -> String;
    fn progress_message(&self, current: u64, total: u64) -> String {
        format!("{}/{} frames", current, total,)
    }
}

impl<C: Default> Video<C> {
    pub(crate) fn setup_encoder(
        &mut self,
        output_path: impl Into<PathBuf>,
    ) -> Result<Box<dyn Encoder + Send>> {
        let (width, height) =
            self.initial_canvas.resolution_to_size_even(self.resolution);

        let destination = output_path.into();
        let pb = &self.progress_bars.encoding;

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
