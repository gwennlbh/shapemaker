use super::{context::Context, engine::milliseconds_to_timestamp, Video};
use crate::rendering::stringify_svg;
use crate::{Canvas, SVGRenderable};
use anyhow::Result;
use measure_time::debug_time;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::SyncSender;
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

    pub fn render_frames(
        &self,
        output: SyncSender<(Duration, String)>,
    ) -> Result<usize> {
        debug_time!("render_frames");
        let mut written_frames_count: usize = 0;
        let mut context = Context {
            frame: 0,
            beat: 0,
            beat_fractional: 0.0,
            timestamp: "00:00:00.000".to_string(),
            ms: 0,
            bpm: self.syncdata.bpm,
            syncdata: &self.syncdata,
            extra: AdditionalContext::default(),
            later_hooks: vec![],
            audiofile: self.audiofile.clone(),
            duration_override: self.duration_override,
        };

        let mut canvas = self.initial_canvas.clone();

        let mut previous_rendered_beat = 0;
        let mut previous_rendered_frame = 0;

        let render_ms_range = self.start_rendering_at + 0..self.duration_ms();

        self.progress_bar.set_length(render_ms_range.len() as u64);

        for _ in render_ms_range {
            context.ms += 1_usize;
            context.timestamp = milliseconds_to_timestamp(context.ms).to_string();
            context.beat_fractional =
                (context.bpm * context.ms) as f32 / (1000.0 * 60.0);
            context.beat = context.beat_fractional as usize;
            context.frame = self.fps * context.ms / 1000;

            if context.marker() != "" {
                self.progress_bar.println(format!(
                    "{}: marker {}",
                    context.timestamp,
                    context.marker()
                ));
            }

            if context.marker().starts_with(':') {
                let marker_text = context.marker();
                let commandline = marker_text.trim_start_matches(':').to_string();

                for command in &self.commands {
                    if commandline.starts_with(&command.name) {
                        let args = commandline
                            .trim_start_matches(&command.name)
                            .trim()
                            .to_string();
                        (command.action)(args, &mut canvas, &mut context)?;
                    }
                }
            }

            // Render later hooks first, so that for example animations that aren't finished yet get overwritten by next frame's hook, if the next frames touches the same object
            // This is way better to cancel early animations such as fading out an object that appears on every note of a stem, if the next note is too close for the fade-out to finish.

            let mut later_hooks_to_delete: Vec<usize> = vec![];

            for (i, hook) in context.later_hooks.iter().enumerate() {
                if (hook.when)(&canvas, &context, previous_rendered_beat) {
                    (hook.render_function)(&mut canvas, context.ms)?;
                    if hook.once {
                        later_hooks_to_delete.push(i);
                    }
                } else if !hook.once {
                    later_hooks_to_delete.push(i);
                }
            }

            for i in later_hooks_to_delete {
                if i < context.later_hooks.len() {
                    context.later_hooks.remove(i);
                }
            }

            for hook in &self.hooks {
                if (hook.when)(
                    &canvas,
                    &context,
                    previous_rendered_beat,
                    previous_rendered_frame,
                ) {
                    (hook.render_function)(&mut canvas, &mut context)?;
                }
            }

            if context.frame != previous_rendered_frame {
                output.send((
                    Duration::from_millis(context.ms as _),
                    stringify_svg(canvas.render_to_svg(
                        canvas.colormap.clone(),
                        canvas.cell_size,
                        canvas.object_sizes,
                        "",
                    )?),
                ))?;

                written_frames_count += 1;

                previous_rendered_beat = context.beat;
                previous_rendered_frame = context.frame;
            }
        }

        output.send((Duration::from_millis(context.ms as _), "".to_string()))?;

        Ok(written_frames_count)
    }

    pub fn render(&mut self, output_file: impl Into<PathBuf>) -> Result<()> {
        debug_time!("render");

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

        self.render_frames(tx)?;

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
