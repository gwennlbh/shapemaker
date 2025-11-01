mod scenes;

use anyhow::anyhow;
use itertools::Itertools;
use rand::{SeedableRng, rngs::SmallRng};
use shapemaker::{ui::Log, video::engine::EngineControl, *};
use std::{fs, path::PathBuf, time::Duration};

pub struct State {
    bass_pattern_at: Region,
    kick_color: Color,
    rng: SmallRng,
    cranks: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            bass_pattern_at: Region::from_topleft(Point(1, 1), (2, 2)).unwrap(),
            kick_color: Color::White,
            rng: SmallRng::seed_from_u64(0),
            cranks: 0,
        }
    }
}

#[tokio::main]
pub async fn main() {
    let canvas = Canvas::new(16, 9);

    let mut video = Video::<State>::new(canvas);
    let mut args = pico_args::Arguments::from_env();

    video.duration_override = args
        .value_from_str("--duration")
        .ok()
        .map(Duration::from_secs);

    if video.duration_override.is_some_and(|d| d.is_zero()) {
        video.duration_override = None;
    }

    video.start_rendering_at = args
        .value_from_str("--start")
        .ok()
        .map(Timestamp::from_seconds)
        .unwrap_or_default();

    video = video
        // Sync inputs //
        .sync_audio_with("schedule-hell.midi")
        .sync_audio_with("schedule-hell.wav");

    if let Ok(marker) = args.value_from_str::<_, String>("--marker") {
        let marker_start = video
            .syncdata
            .markers
            .iter()
            .find_map(|(&ms, m)| if m == &marker { Some(ms) } else { None })
            .expect("Marker not found");

        let marker_end = video
            .syncdata
            .markers
            .iter()
            .filter(|&(&ms, _)| ms > marker_start)
            .sorted_by_key(|&(&ms, _)| ms)
            .find_map(|(&ms, m)| if m != &marker { Some(ms) } else { None });

        video.start_rendering_at = Timestamp::from_ms(marker_start as _);
        video.duration_override =
            marker_end.map(|end| Duration::from_millis((end - marker_start) as _))
    }

    video.resolution = args.value_from_str("--resolution").ok().unwrap_or(480);
    video.fps = args.value_from_str("--fps").ok().unwrap_or(30);

    video.audiofile = PathBuf::from("schedule-hell.wav");
    video = video
        // Scenes //
        .with_scene(scenes::starry_sky())
        .with_init_scene(scenes::intro())
        .with_marked_scene(scenes::first_break())
        .with_scene(scenes::backbone())
        .assign_scene_to("end of first break", "intro")
        .assign_scene_to("second break", "starry sky")
        // "end first break" means "end of second break" lol
        .assign_scene_to("end first break", "backbone")
        // Credits //
        .when_remaining(10, &|canvas, _| {
            let world = canvas.world_region;
            canvas.root().set(
                "credits text",
                Object::Text(
                    world.start.translated(2, 2),
                    "Postamble / Schedule Hell".into(),
                    12.0,
                )
                .colored(Color::White),
            );
            Ok(())
        });

    let destination: String = args
        .free_from_str()
        .unwrap_or(String::from("schedule-hell.mp4"));

    if destination.starts_with("localhost:") {
        video.serve(&destination).await;
    } else {
        let result = if destination.ends_with(".svg") {
            let render_ahead = 10;

            let frame_no = destination
                .trim_end_matches(".svg")
                .parse::<usize>()
                .expect("Provide a integer when rendering a frame");

            video.progress_bars.loading.log(
                "Constrained",
                &format!(
                    "to frame #{frame_no}, with {render_ahead}-frame context"
                ),
            );

            video
                .render_frame(frame_no, render_ahead)
                .and_then(|svg| {
                    fs::write(destination, svg.to_string())
                        .map_err(|e| anyhow!("{e:?}"))
                })
                .map(|_| Duration::default())
        } else {
            video.encode(destination)
        };

        match result {
            Ok(_) => (),
            Err(e) => {
                let _ = video.progress.clear();
                ().log_error("Failed", &format!("{e:?}"));
            }
        };
    }
}
