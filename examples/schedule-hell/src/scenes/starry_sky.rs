use shapemaker::{synchronization::audio::MusicalDurationUnit, *};

use crate::State;

pub fn starry_sky() -> Scene<State> {
    Scene::new("starry sky")
        .init(&|canvas, _| {
            canvas.clear();
            sky(Angle::default(), canvas);
            Ok(())
        })
        .every(1.0, MusicalDurationUnit::Eighths, &|canvas, ctx| {
            canvas.clear();
            sky(
                Angle::from_degrees(ctx.scene_started_at_ms.unwrap() as _),
                canvas,
            );
            Ok(())
        })
}

fn sky(theta: Angle, canvas: &mut Canvas) -> () {
    canvas.colormap = ColorMapping {
        black: "#000000".to_string(),
        white: "#FFFFFF".to_string(),
        ..Default::default()
    };

    canvas.set_background(Color::Black);
    canvas.set_grid_size(16 * 4, 9 * 4);

    let draw_in = canvas.world_region.clone().resized(-8, -14);
    let (leftside, rightside) = draw_in.split(Axis::Vertical);
    let (lefttopside, leftbottomside) = leftside.split(Axis::Horizontal);

    canvas.add_layer(cluster(
        draw_in,
        theta,
        lefttopside.translated(4, 0).center(),
    ));
    canvas.add_layer(cluster(draw_in, theta, leftbottomside.center()));
    canvas.add_layer(cluster(draw_in, theta, draw_in.center().translated(2, -2)));
    canvas.add_layer(cluster(draw_in, theta, draw_in.center().translated(-2, 2)));
    canvas.add_layer(cluster(
        draw_in,
        theta,
        rightside.translated(0, 4).center(),
    ));

    let background_stars_in = canvas.world_region.clone().enlarged(1, 1);

    for point in background_stars_in {
        canvas
            .root()
            .add_anon(Object::Dot(point).filled(Fill::Translucent(
                Color::White,
                rand::random_range(if rand::random_bool(0.01) {
                    0.8..=1.0
                } else {
                    0.0..=0.3
                }),
            )));
    }
}

fn cluster(world: Region, rotation: Angle, at: Point) -> Layer {
    let mut layer = Layer::new(format!("cluster{}", rand::random::<u32>()));

    for _ in 1..=rand::random_range(2..=5) {
        layer.add_anon(
            Object::random(
                &mut rand::rng(),
                &Region::from_center_and_size(
                    at.rotated(&world.center(), rotation),
                    (2, 2),
                )
                .unwrap(),
                5.0,
                3..6,
            )
            .colored(Color::White),
        );
    }

    layer
}
