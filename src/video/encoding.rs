use super::Video;
use crate::rendering::svg;
use crate::ui::format_duration;
use crate::video::engine::EngineOutput;
use crate::{ui::Log, Canvas};
use anyhow::Result;
use itertools::Itertools;
use measure_time::debug_time;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::{fs::create_dir_all, path::PathBuf};

impl<AdditionalContext: Default> Video<AdditionalContext> {
    pub fn encode_to_vgv(
        &mut self,
        output_file: impl Into<PathBuf>,
    ) -> Result<()> {
        debug_time!("encode_to_vgv");
        let output_file: PathBuf = output_file.into();

        if output_file.exists() {
            std::fs::remove_file(&output_file)?;
        }

        create_dir_all(
            &output_file
                .parent()
                .expect("Given output file has no parent"),
        )?;

        let mut file = File::create(&output_file)?;

        let pb = self.progress_bars.encoding.clone();
        pb.set_position(0);
        pb.set_length(self.ms_to_frames(self.duration_ms()) as _);
        pb.set_prefix("Encoding");
        pb.set_message(format!(
            "with VGV, to {}{}",
            if output_file.is_relative() { "./" } else { "" },
            output_file.to_string_lossy(),
        ));

        self.initial_canvas.load_fonts()?;
        let initial_canvas = self.initial_canvas.clone();
        let resolution = self.resolution;
        let (width, height) = initial_canvas.resolution_to_size_even(resolution);

        let (tx, rx) = std::sync::mpsc::sync_channel::<EngineOutput>(10_000);

        let mut vgv_encoder = vgv::Encoder::new(vgv::Frame::Initialization {
            w: width as _,
            h: height as _,
            d: (1000.0 / self.fps as f64) as _,
            bg: initial_canvas
                .background
                .unwrap_or_default()
                .render(&initial_canvas.colormap),
            svg: format!(
                r#"width={w} height={h} viewBox="-{pad} -{pad} {w} {h}""#,
                w = initial_canvas.width(),
                h = initial_canvas.height(),
                pad = initial_canvas.canvas_outer_padding
            ),
        });

        vgv_encoder.full_diff_ratio = 500;

        let vgv_thread = thread::spawn(move || {
            for output in rx.iter() {
                match output {
                    EngineOutput::Finished => break,
                    EngineOutput::Frame(ref svg) => {
                        pb.inc(1);
                        vgv_encoder.encode_svg(match svg {
                            svg::Node::Text(text) => text.to_string(),
                            svg::Node::SVG(svg) => svg.to_string(),
                            svg::Node::Element(element) => element
                                .children
                                .iter()
                                .map(|child| child.to_string())
                                .join(""),
                        });
                    }
                }
            }

            vgv_encoder.dump(&mut file);
        });

        self.render_with_overrides(tx)?;

        vgv_thread.join().expect("VGV thread panicked");

        self.progress_bars.encoding.finish();

        Ok(())
    }

    fn setup_encoder(
        &mut self,
        output_path: impl Into<PathBuf>,
    ) -> anyhow::Result<std::process::Child> {
        debug_time!("setup_encoder");
        let output_path: PathBuf = output_path.into();

        let (width, height) =
            self.initial_canvas.resolution_to_size_even(self.resolution);

        Ok(std::process::Command::new("ffmpeg")
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
            .stderr(File::create("ffmpeg_stderr.log")?)
            //
            // Spawn it!
            .spawn()?)
    }

    pub fn encode(&mut self, output_file: impl Into<PathBuf>) -> Result<()> {
        debug_time!("encode");

        self.progress.remove(&self.progress_bars.loading);

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

        let pb = self.progress_bars.encoding.clone();

        pb.set_length(self.ms_to_frames(self.duration_ms()) as _);
        pb.set_message("");

        self.initial_canvas.load_fonts()?;
        let initial_canvas = self.initial_canvas.clone();
        let resolution = self.resolution;

        let (tx, rx) = std::sync::mpsc::sync_channel::<EngineOutput>(1_000);

        let encoder_thread = thread::spawn(move || {
            for output in rx.iter() {
                match output {
                    EngineOutput::Finished => break,
                    EngineOutput::Frame(svg) => {
                        pb.inc(1);
                        pb.set_message(format!(
                            "{}/{} frames",
                            pb.position(),
                            pb.length().unwrap()
                        ));
                        encode_frame(
                            &mut encoder,
                            resolution,
                            &initial_canvas,
                            svg,
                        )
                        .unwrap();
                    }
                }
            }

            encoder.stdin.take().unwrap().flush().unwrap();
        });

        self.render_with_overrides(tx)?;

        encoder_thread.join().expect("Encoder thread panicked");

        self.progress_bars.encoding.finish();
        self.progress_bars.encoding.log(
            "Encoded",
            &format!(
                "video to {}{} in {}",
                if output_file.is_relative() { "./" } else { "" },
                output_file.to_string_lossy(),
                format_duration(self.progress_bars.encoding.elapsed())
            ),
        );

        self.progress.clear().unwrap();

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
    canvas: &Canvas,
    svg: svg::Node,
) -> anyhow::Result<()> {
    debug_time!("encode_frame");
    // Make sure that width and height are divisible by 2, as the encoder requires it
    let (width, height) = canvas.resolution_to_size_even(resolution);

    let pixmap = canvas.svg_to_pixmap(width, height, &svg.to_string())?;
    // Send frame
    encoder.stdin.as_mut().unwrap().write_all(&pixmap.data())?;

    // let frame =
    //     canvas.pixmap_to_hwc_frame((width as usize, height as usize), &pixmap)?;
    // Ok(encoder.encode(&frame, timestamp)?)
    Ok(())
}
