use super::{hooks::milliseconds_to_timestamp, Video};
use crate::Canvas;
use anyhow::Result;
use measure_time::debug_time;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::{fs::create_dir_all, path::PathBuf};

impl<AdditionalContext: Default> Video<AdditionalContext> {
    fn setup_encoder(
        &mut self,
        output_path: impl Into<PathBuf>,
    ) -> anyhow::Result<std::process::Child> {
        debug_time!("setup_encoder");
        let output_path: PathBuf = output_path.into();

        let (width, height) =
            self.initial_canvas.resolution_to_size_even(self.resolution);

        Ok(std::process::Command::new("ffmpeg")
            .arg("-i")
            .arg(self.audiofile.to_str().unwrap())
            .arg("-f")
            .arg("rawvideo")
            .arg("-pixel_format")
            .arg("rgba")
            .arg("-video_size")
            .arg(format!("{width}x{height}"))
            .arg("-framerate")
            .arg(format!("{}", self.fps))
            .arg("-i")
            .arg("-")
            .arg("-map")
            .arg("0:a")
            .arg("-map")
            .arg("1:v")
            .arg("-shortest")
            .arg(output_path.to_str().unwrap())
            .arg("-loglevel")
            .arg(if log::log_enabled!(log::Level::Debug) {
                "debug"
            } else {
                "error"
            })
            .stdin(std::process::Stdio::piped())
            .stdout(File::create("ffmpeg_stdout.log")?)
            .stderr(File::create("ffmpeg_stderr.log")?)
            .spawn()?)
    }

    pub fn encode(&mut self, output_file: impl Into<PathBuf>) -> Result<()> {
        debug_time!("encode");

        let output_file: PathBuf = output_file.into();

        if output_file.exists() {
            std::fs::remove_file(&output_file)?;
        }

        create_dir_all(
            &output_file
                .parent()
                .expect("Given output file has no parent"),
        )?;

        let mut encoder = self.setup_encoder(&output_file)?;

        self.progress_bar.set_position(0);
        self.progress_bar.set_prefix("Rendering");
        self.progress_bar.set_message("");

        self.initial_canvas.load_fonts()?;
        let initial_canvas = self.initial_canvas.clone();
        let resolution = self.resolution;

        let (tx, rx) = std::sync::mpsc::sync_channel::<(Duration, String)>(1_000);

        let pb = self.progress_bar.clone();

        let encoder_thread = thread::spawn(move || {
            for (time, svg) in rx.iter() {
                if svg.is_empty() {
                    break;
                }

                encode_frame(
                    &mut encoder,
                    resolution,
                    time,
                    &initial_canvas,
                    &svg,
                )
                .unwrap();

                pb.set_position(time.as_millis() as _);
                pb.set_message(milliseconds_to_timestamp(time.as_millis() as _));
            }

            encoder.stdin.take().unwrap().flush().unwrap();
        });

        self.render_all_frames(tx)?;

        encoder_thread.join().expect("Encoder thread panicked");

        Ok(())
    }

    #[allow(dead_code)]
    fn add_audio_track(&mut self, _output_file: String) -> Result<()> {
        todo!("Look into https://github.com/zmwangx/rust-ffmpeg/blob/master/examples/transcode-x264.rs and maybe contribute to video-rs (see https://github.com/oddity-ai/video-rs/issues/44)");
    }
}

fn encode_frame(
    encoder: &mut std::process::Child,
    resolution: u32,
    _timestamp: Duration,
    canvas: &Canvas,
    svg: &String,
) -> anyhow::Result<()> {
    debug_time!("encode_frame");
    // Make sure that width and height are divisible by 2, as the encoder requires it
    let (width, height) = canvas.resolution_to_size_even(resolution);

    let pixmap = canvas.svg_to_pixmap(width, height, svg)?;
    // Send frame
    encoder.stdin.as_mut().unwrap().write_all(&pixmap.data())?;

    // let frame =
    //     canvas.pixmap_to_hwc_frame((width as usize, height as usize), &pixmap)?;
    // Ok(encoder.encode(&frame, timestamp)?)
    Ok(())
}
