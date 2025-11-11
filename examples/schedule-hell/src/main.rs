mod scenes;

use anyhow::anyhow;
use schedule_hell::State;
use shapemaker::{ui::Log, *};
use std::{fs, path::PathBuf, time::Duration};

#[tokio::main]
pub async fn main() {
    let canvas = Canvas::new(16, 9);

    let mut video = Video::<State>::new(canvas);
    let mut args = pico_args::Arguments::from_env();

    video = video
        // Sync inputs //
        .sync_audio_with("schedule-hell.midi")
        .expect("Failed to sync from MIDI file")
        .sync_audio_with("schedule-hell.wav")
        .expect("Failed to sync from WAV file");

    if let Ok(marker) = args.value_from_str::<_, String>("--marker") {
        let range = video
            .syncdata
            .marker_ms_range(marker)
            .expect("Cannot find marker {marker:?} in sync data");

        video.start_rendering_at = Timestamp::from_ms(range.start);
        video.duration_override = Some(Duration::from_millis(range.len() as _));
    }

    if let Ok(duration) = args.value_from_str("--duration") {
        video.duration_override = Some(Duration::from_secs(duration));
    }

    if let Ok(start) = args.value_from_str("--start") {
        video.start_rendering_at = Timestamp::from_seconds(start);
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
        .with_scene(scenes::dices())
        .assign_scene_to("end of first break", "intro")
        .assign_scene_to("second break", "starry sky")
        // "end first break" means "end of second break" lol
        .assign_scene_to("end first break", "dices")
        // Credits //
        .when_remaining(10, &|canvas, _| {
            let world = canvas.world_region;
            canvas.root().set(
                "credits text",
                Shape::Text(
                    world.start,
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
            let render_ahead = 1_000;

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
