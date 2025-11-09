use crate::{
    Video,
    rendering::rasterization::{create_pixmap, paint_svg_on_pixmap},
    ui::Pretty,
    video::{encoders::Encoder, engine::EngineOutput},
};
use anyhow::{Result, anyhow};
use measure_time::debug_time;
use std::{fs::File, io::Write, path::PathBuf, sync::Arc};

pub struct FFMpegEncoder {
    pixmap: tiny_skia::Pixmap,
    process: std::process::Child,
    fontdb: Option<Arc<resvg::usvg::fontdb::Database>>,
    destination: PathBuf,
}

impl<C: Default> Video<C> {
    pub fn setup_ffmpeg_encoder(
        &self,
        width: u32,
        height: u32,
        output_path: PathBuf,
    ) -> Result<FFMpegEncoder> {
        debug_time!("setup_encoder");
        let output_path: PathBuf = output_path.into();

        let mut command = std::process::Command::new("ffmpeg");
        command
            // Audio //
            // Take non-0 starting point into account
            .args(["-ss", &self.start_rendering_at.seconds_string()])
            // File
            .args(["-i", self.audiofile.to_str().unwrap()])
            //
            // Video //
            // Raw video input
            .args(["-f", "rawvideo"])
            // RGBA Pixels
            .args(["-pixel_format", "rgba"])
            // Dimensions
            .args(["-video_size", &format!("{width}x{height}")])
            // FPS
            .args(["-framerate", &self.fps.to_string()])
            // Input from pipe
            .args(["-i", "-"])
            .stdin(std::process::Stdio::piped())
            //
            // Mapping //
            // Audio from first input
            .args(["-map", "0:a"])
            // Video from second input
            .args(["-map", "1:v"])
            // Use shortest stream for final duration
            .arg("-shortest")
            //
            // Output //
            // Use 4:2:0 (4:4:4 is not widely supported)
            .args(["-pix_fmt", "yuv420p"])
            // Write to file
            .arg(output_path.to_str().unwrap())
            // Debug ffmpeg too if shapemaker is debugging
            .args([
                "-loglevel",
                (if log::log_enabled!(log::Level::Debug) {
                    "debug"
                } else {
                    "error"
                }),
            ])
            // Put stdout/stderr here so that it doesn't mess with progress bars
            .stdout(File::create("ffmpeg_stdout.log")?)
            .stderr(File::create("ffmpeg_stderr.log")?);

        let commandline = format!("{:?}", &command);

        Ok(FFMpegEncoder {
            destination: output_path.clone(),
            fontdb: self.initial_canvas.fontdb.clone(),
            pixmap: create_pixmap(width, height),
            process: command
                .spawn()
                .map_err(|e| anyhow!("Could not run {commandline}: {e:?}",))?,
        })
    }
}

impl Encoder for FFMpegEncoder {
    fn name(&self) -> String {
        "FFMpeg".into()
    }

    fn encode_frame(&mut self, output: EngineOutput) -> Result<()> {
        if let EngineOutput::Frame { svg, dimensions } = output {
            // TODO prendre width et height sur la node svg au lieu de devoir donnner un canvas initial (la grid size peut changer depuis l'initial canvas)
            debug_time!("encode_frame");
            // Make sure that width and height are divisible by 2, as the encoder requires it

            // let pixmap = svg_to_pixmap(width, height, &svg.to_string())?;
            paint_svg_on_pixmap(
                self.pixmap.as_mut(),
                &svg.to_string(),
                dimensions,
                &self.fontdb,
            )?;

            // Send frame
            self.process
                .stdin
                .as_mut()
                .unwrap()
                .write_all(&self.pixmap.data())?;

            // let frame =
            //     canvas.pixmap_to_hwc_frame((width as usize, height as usize), &pixmap)?;
            // Ok(encoder.encode(&frame, timestamp)?)
            Ok(())
        } else {
            Ok(())
        }
    }

    fn finish(&mut self) -> Result<()> {
        self.process.stdin.take().unwrap().flush().unwrap();
        Ok(())
    }

    fn finish_message(&self, time_elapsed: std::time::Duration) -> String {
        format!(
            "video to {} in {}",
            self.destination.pretty(),
            time_elapsed.pretty()
        )
    }
}
