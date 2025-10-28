mod scenes;

use anyhow::Result;
use rand::{SeedableRng, rngs::SmallRng};
use shapemaker::*;
use std::{path::PathBuf, time::Duration};

pub struct State {
    bass_pattern_at: Region,
    kick_color: Color,
    rng: SmallRng,
    kick_counter: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            bass_pattern_at: Region::from_topleft(Point(1, 1), (2, 2)).unwrap(),
            kick_color: Color::White,
            rng: SmallRng::seed_from_u64(0),
            kick_counter: 0,
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut canvas = Canvas::with_layers(vec![]);

    canvas.set_grid_size(16, 9);
    canvas.colormap = ColorMapping {
        black: "#000000".into(),
        white: "#ffffff".into(),
        red: "#cf0a2b".into(),
        green: "#22e753".into(),
        blue: "#2734e6".into(),
        yellow: "#f8e21e".into(),
        orange: "#f05811".into(),
        purple: "#6a24ec".into(),
        brown: "#a05634".into(),
        pink: "#e92e76".into(),
        gray: "#81a0a8".into(),
        cyan: "#4fecec".into(),
    };

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

    video.resolution = args.value_from_str("--resolution").ok().unwrap_or(480);
    video.fps = args.value_from_str("--fps").ok().unwrap_or(30);

    video.audiofile = PathBuf::from("schedule-hell.wav");
    video = video
        // Sync inputs //
        .sync_audio_with("schedule-hell.midi")
        .sync_audio_with("schedule-hell.wav")
        // Scenes //
        .with_scene(scenes::starry_sky())
        .with_init_scene(scenes::intro())
        .with_marked_scene(scenes::first_break())
        .assign_scene_to("end of first break", "starry sky")
        .assign_scene_to("second break", "intro")
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
        video.serve("localhost:8000").await;
    } else {
        video.encode(destination)?;
    }

    Ok(())
}
