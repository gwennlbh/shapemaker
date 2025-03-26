extern crate ffmpeg_next as ffmpeg;
use super::{context::Context, engine::milliseconds_to_timestamp, Video};
use crate::rendering::stringify_svg;
use crate::{ui::Log, Canvas, SVGRenderable};
use anyhow::Result;
use indicatif::ProgressIterator;
use measure_time::debug_time;
use rayon::iter::ParallelIterator;
use rayon::{iter::IndexedParallelIterator, slice::ParallelSliceMut};
use std::sync::MutexGuard;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, Mutex},
};
use video_rs::Time;

impl Canvas {
    pub fn render_to_hwc_frame(
        &mut self,
        size: (usize, usize),
    ) -> anyhow::Result<video_rs::Frame> {
        let (width, height) = size;
        let pixmap = self.render_to_pixmap(width as u32, height as u32)?;
        self.pixmap_to_hwc_frame(size, &pixmap)
    }

    pub fn pixmap_to_hwc_frame(
        &self,
        size: (usize, usize),
        pixmap: &tiny_skia::Pixmap,
    ) -> anyhow::Result<video_rs::Frame> {
        debug_time!("pixmap_to_hwc_frame");
        let (width, height) = size;
        let mut data = vec![0u8; height * width * 3];

        data.par_chunks_exact_mut(3)
            .enumerate()
            .for_each(|(index, chunk)| {
                let x = index % width;
                let y = index / width;

                let pixel =
                    pixmap.pixel(x as u32, y as u32).unwrap_or_else(|| {
                        panic!("No pixel found at x, y = {x}, {y}")
                    });

                chunk[0] = pixel.red();
                chunk[1] = pixel.green();
                chunk[2] = pixel.blue();
            });

        Ok(video_rs::Frame::from_shape_vec([height, width, 3], data)?)
    }
}

impl<AdditionalContext: Default> Video<AdditionalContext> {
    fn setup_encoder(&mut self, output_path: &str) -> anyhow::Result<()> {
        debug_time!("setup_encoder");
        let (width, height) =
            self.initial_canvas.resolution_to_size_even(self.resolution);

        self.encoder = Some(Arc::new(Mutex::new(
            video_rs::Encoder::new(
                PathBuf::from_str(output_path)?,
                video_rs::encode::Settings::preset_h264_yuv420p(
                    width as usize,
                    height as usize,
                    false,
                ),
            )
            .expect("Failed to build encoder"),
        )));

        Ok(())
    }

    pub fn render_frames(&mut self) -> Result<usize> {
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

        let render_ms_range = 0..self.duration_ms() + self.start_rendering_at;

        self.progress_bar.set_length(render_ms_range.len() as u64);

        let mut frames_to_encode: Vec<(Time, String)> = vec![];

        for _ in render_ms_range
            .into_iter()
            .progress_with(self.progress_bar.clone())
        {
            context.ms += 1_usize;
            context.timestamp = milliseconds_to_timestamp(context.ms).to_string();
            context.beat_fractional =
                (context.bpm * context.ms) as f32 / (1000.0 * 60.0);
            context.beat = context.beat_fractional as usize;
            context.frame = self.fps * context.ms / 1000;

            self.progress_bar.set_message(context.timestamp.clone());

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
                frames_to_encode.push((
                    Time::from_secs_f64(context.ms as f64 * 1e-3),
                    stringify_svg(canvas.render_to_svg(
                        canvas.colormap.clone(),
                        canvas.cell_size,
                        canvas.object_sizes,
                        "",
                    )?),
                ));

                written_frames_count += 1;

                previous_rendered_beat = context.beat;
                previous_rendered_frame = context.frame;
            }
        }

        self.initial_canvas.load_fonts()?;

        self.progress_bar.set_position(0);
        self.progress_bar.set_length(frames_to_encode.len() as u64);
        self.progress_bar.set_message("Encoding");

        for (time, svg) in frames_to_encode
            .into_iter()
            .progress_with(self.progress_bar.clone())
        {
            encode_frame(
                self.encoder
                    .as_mut()
                    .expect("Encoder was not initalized")
                    .lock()
                    .unwrap(),
                self.resolution,
                time,
                &canvas,
                &svg,
            )?;
        }

        self.progress_bar.finish();

        Ok(written_frames_count)
    }

    pub fn render(&mut self, output_file: String) -> Result<()> {
        debug_time!("render");

        // create_dir_all(self.frames_output_directory)?;
        // remove_dir_all(self.frames_output_directory)?;
        // create_dir(self.frames_output_directory)?;
        create_dir_all(Path::new(&output_file).parent().unwrap())?;

        self.setup_encoder(&output_file)?;

        self.progress_bar.set_position(0);
        self.progress_bar.set_prefix("Rendering");
        self.progress_bar.set_message("");

        let frames_written = self.render_frames()?;

        self.encoder
            .as_mut()
            .expect("Encoder is missing somehow")
            .lock()
            .unwrap()
            .finish()?;

        self.progress_bar.log(
            "Rendered",
            &format!("{} frames to {}", frames_written, output_file),
        );

        self.progress_bar.set_position(0);
        self.progress_bar.set_prefix("Adding");
        self.progress_bar.set_message("audio track");

        Ok(())
    }

    #[allow(dead_code)]
    fn add_audio_track(&mut self, _output_file: String) -> Result<()> {
        todo!("Look into https://github.com/zmwangx/rust-ffmpeg/blob/master/examples/transcode-x264.rs and maybe contribute to video-rs (see https://github.com/oddity-ai/video-rs/issues/44)");
    }
}

fn encode_frame(
    mut encoder: MutexGuard<video_rs::Encoder>,
    resolution: u32,
    timestamp: Time,
    canvas: &Canvas,
    svg: &String,
) -> anyhow::Result<()> {
    debug_time!("encode_frame");
    // Make sure that width and height are divisible by 2, as the encoder requires it
    let (width, height) = canvas.resolution_to_size_even(resolution);

    let pixmap = canvas.svg_to_pixmap(width, height, svg)?;
    let frame =
        canvas.pixmap_to_hwc_frame((width as usize, height as usize), &pixmap)?;
    Ok(encoder.encode(&frame, timestamp)?)
}
