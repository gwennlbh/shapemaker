use rand::{Rng, SeedableRng, rngs::SmallRng};
use shapemaker::*;
use std::{time::Duration, vec};

const SEED: u64 = 0;

fn main() {
    env_logger::init();
    let mut canvas = Canvas::with_layers(vec![
        "flickers_occlusions",
        "flickers",
        "occlusions",
        "root",
    ]);

    canvas.colormap = ColorMapping {
        black: "#000000".to_string(),
        white: "#FFFFFF".to_string(),
        purple: "#da40f5".to_string(),
        ..Default::default()
    };

    canvas.set_grid_size(16, 10);
    canvas.set_background(Black);
    canvas.object_sizes.dot_radius = 7.5;

    let mut video = Video::<Ctx>::new(canvas);
    video.audiofile = "../schedule-hell/schedule-hell.wav".into();
    video.fps = 60;
    video.resolution = 480;
    video.duration_override = Some(Duration::from_secs(30));
    video
        .sync_audio_with("../schedule-hell/schedule-hell.midi")
        .init(&|canvas, ctx| {
            backbone(&mut ctx.extra.rng, canvas);
            Ok(())
        })
        .each_frame(&|canvas, ctx| {
            backbone(&mut ctx.extra.rng, canvas);
            Ok(())
        })
        // .each_n_frame(10, &|canvas, ctx| {
        //     canvas.render_to_svg_file(&format!("framedump-{}.svg", ctx.frame))?;
        //     Ok(())
        // })
        .encode("schedule-hell-backbone.mp4")
        .unwrap();
}

fn backbone(rng: &mut SmallRng, canvas: &mut Canvas) {
    // let mut rng = canvas.rng.clone();
    let world = canvas.world_region.clone();

    let grid_thickness = 2.0;

    for point in
        Region::from((world.topleft(), world.topright().translated(1, 1)))
    {
        canvas.root().set(
            format!("grid-rows-{point}"),
            Object::Line(
                Point(point.0, world.topleft().1),
                Point(point.0, world.bottomleft().1 + 1),
                grid_thickness * 0.75,
            )
            .filled(White.translucent(0.05 + rng.random_range(0.0..0.3))),
        );
    }

    for point in
        Region::from((world.topleft(), world.bottomleft().translated(1, 1)))
    {
        canvas.root().set(
            format!("grid-cols-{point}"),
            Object::Line(
                Point(world.topleft().0, point.1),
                Point(world.bottomright().0 + 1, point.1),
                grid_thickness * 0.75,
            )
            .filled(White.translucent(0.005 + rng.random_range(0.0..0.3))),
        );
    }

    let occlusions = canvas.layer("occlusions");

    for point in world.enlarged(1, 1) {
        occlusions.set(
            format!("occlusion-{point}"),
            Object::Dot(point).colored(Color::Black),
        );
    }

    let flickers = canvas.layer("flickers");

    for point in world {
        flickers.set(
            format!("crosses-SWNE-{point}"),
            Object::Line(point, point.translated(1, 1), grid_thickness)
                .colored(Color::Purple)
                .opacified(0.25 + rng.random_range(0.5..1.0)),
        );
        flickers.set(
            format!("crosses-NWSE-{point}"),
            Object::Line(
                point.translated(0, 1),
                point.translated(1, 0),
                grid_thickness,
            )
            .colored(Color::Purple)
            .opacified(0.25 + rng.random_range(0.5..1.0)),
        );
    }

    let flickers_occlusions = canvas.layer("flickers_occlusions");
    flickers_occlusions.object_sizes.dot_radius = 10.0;

    for point in world.enlarged(1, 1) {
        flickers_occlusions.set(
            format!("crosses-occlusions-{point}"),
            Object::Dot(point).colored(Color::Black),
        )
    }
}

struct Ctx {
    rng: SmallRng,
}

impl Default for Ctx {
    fn default() -> Self {
        Self {
            rng: SmallRng::seed_from_u64(SEED),
        }
    }
}
