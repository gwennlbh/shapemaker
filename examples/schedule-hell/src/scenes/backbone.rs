use anyhow::Result;
use rand::{Rng, rngs::SmallRng, seq::IteratorRandom};
use shapemaker::CornerPoint as P;
use shapemaker::*;

use crate::State;

pub fn backbone() -> Scene<State> {
    Scene::<State>::new("backbone")
        .init(&|canvas, ctx| {
            canvas.clear();

            canvas.colormap = ColorMapping {
                black: "#000000".to_string(),
                white: "#FFFFFF".to_string(),
                purple: "#da40f5".to_string(),
                ..Default::default()
            };

            canvas.set_grid_size(16, 10);
            canvas.set_background(Black);
            canvas.object_sizes.dot_radius = 7.5;

            iterate(&mut ctx.extra.rng, canvas)?;
            Ok(())
        })
        // .each_n_frame(3, &|canvas, ctx| {
        //     canvas.clear();
        //     iterate(&mut ctx.extra.rng, canvas)?;
        //     Ok(())
        // })
        .on_note("anchor kick", &|canvas, ctx| {
            canvas.clear();
            iterate(&mut ctx.extra.rng, canvas)?;

            let world = canvas.world_region.clone();
            let flickers = canvas.layer("flickers")?;

            let point = world.iter().choose(&mut ctx.extra.rng).unwrap();

            flickers.tag_objects("rotate", |id, _| {
                id == &format!("crosses-SWNE-{point}")
                    || id == &format!("crosses-NWSE-{point}")
            });

            ctx.animate(200, &move |t, canvas, _| {
                canvas
                    .layer("flickers")?
                    .objects_with_tag("rotate")
                    .for_each(|(_, obj)| {
                        obj.recolor(Cyan);
                        obj.set_rotation(Angle::from_degrees(t * 90.0));
                    });

                Ok(())
            });

            Ok(())
        })
        .dump_frame_when(&|_, ctx, _, _| match ctx.since_scene_start() {
            Some(t) => t.as_millis() == 500,
            None => false,
        })
}

fn iterate(rng: &mut SmallRng, canvas: &mut Canvas) -> Result<()> {
    // let mut rng = canvas.rng.clone();
    let world = canvas.world_region.clone();

    let grid_thickness = 2.0;

    for point in
        Region::from((world.topleft(), world.topright().translated(1, 1)))
    {
        canvas.root().set(
            format!("grid-rows-{point}"),
            Shape::Line(
                P(point.x(), world.topleft().y()),
                P(point.x(), world.bottomleft().y() + 1),
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
            Shape::Line(
                P(world.topleft().x(), point.y()),
                P(world.bottomright().x() + 1, point.y()),
                grid_thickness * 0.75,
            )
            .filled(White.translucent(0.005 + rng.random_range(0.0..0.3))),
        );
    }

    let occlusions = canvas.layer_or_empty("occlusions");

    for point in world.enlarged(1, 1) {
        occlusions.set(
            format!("occlusion-{point}"),
            Shape::Dot(point).colored(Color::Black),
        );
    }

    let flickers = canvas.layer_or_empty("flickers");

    for point in world {
        flickers.set(
            format!("crosses-SWNE-{point}"),
            Shape::Line(point, point.translated(1, 1), grid_thickness)
                .colored(Color::Purple)
                .flickering(rng, 0.25)
                .clipped_to((point, point)),
        );
        flickers.set(
            format!("crosses-NWSE-{point}"),
            Shape::Line(
                point.translated(0, 1),
                point.translated(1, 0),
                grid_thickness,
            )
            .colored(Color::Purple)
            .flickering(rng, 0.25)
            .clipped_to((point, point)),
        );
    }

    let flickers_occlusions = canvas.layer_or_empty("flickers_occlusions");
    flickers_occlusions.object_sizes.dot_radius = 10.0;

    for point in world.enlarged(1, 1) {
        flickers_occlusions.set(
            format!("crosses-occlusions-{point}"),
            Shape::Dot(point).colored(Color::Black),
        )
    }

    canvas.reorder_layers(vec![
        "flickers_occlusions",
        "flickers",
        "occlusions",
        "root",
    ]);

    Ok(())
}
