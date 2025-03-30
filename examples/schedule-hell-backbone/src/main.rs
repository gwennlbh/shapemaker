use rand::{Rng, SeedableRng, rngs::SmallRng};
use shapemaker::*;
use std::vec;

const SEED: u64 = 0;

fn main() {
    let mut canvas = Canvas::new(vec![
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
    canvas.set_background(Color::Black);
    canvas.object_sizes.dot_radius = 7.5;

    let mut video = Video::<Ctx>::new(canvas);
    video.audiofile = "../schedule-hell/schedule-hell.flac".into();
    video.resolution = 720;
    video.duration_override = Some(3_000);
    video
        .sync_audio_with("../schedule-hell/schedule-hell.midi")
        .each_n_frame(10, &|canvas, ctx| {
            // canvas.set_background(ctx.extra.rng.random());
            backbone(&mut ctx.extra.rng, canvas);
            Ok(())
        })
        .render("schedule-hell-backbone.mp4".into())
        .unwrap();
}

fn backbone(rng: &mut SmallRng, canvas: &mut Canvas) {
    // let mut rng = canvas.rng.clone();
    let world = canvas.world_region.clone();

    let grid_thickness = 2.0;

    for point in
        Region::from((world.topleft(), world.topright().translated(1, 1)))
    {
        canvas.root().add(
            Object::Line(
                Point(point.0, world.topleft().1),
                Point(point.0, world.bottomleft().1 + 1),
                grid_thickness * 0.75,
            )
            .colored(Color::White),
        );
    }

    for point in
        Region::from((world.topleft(), world.bottomleft().translated(1, 1)))
    {
        canvas.root().add(
            Object::Line(
                Point(world.topleft().0, point.1),
                Point(world.bottomright().0 + 1, point.1),
                grid_thickness * 0.75,
            )
            .colored(Color::White)
            .opacified(0.25 + rng.random_range(0.0..0.3)),
        );
    }

    let occlusions = canvas.layer("occlusions");

    for point in world.enlarged(1, 1) {
        occlusions.add(Object::Dot(point).colored(Color::Black));
    }

    let flickers = canvas.layer("flickers");

    for point in world {
        flickers.add(
            Object::Line(point, point.translated(1, 1), grid_thickness)
                .colored(Color::Purple),
        );
        flickers.add(
            Object::Line(
                point.translated(0, 1),
                point.translated(1, 0),
                grid_thickness,
            )
            .colored(Color::Purple)
            .opacified(0.5 + rng.random_range(0.5..1.0)),
        );
    }

    let flickers_occlusions = canvas.layer("flickers_occlusions");
    flickers_occlusions.object_sizes.dot_radius = 10.0;

    for point in world.enlarged(1, 1) {
        flickers_occlusions.add(Object::Dot(point).colored(Color::Black))
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
