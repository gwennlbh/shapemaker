use crate::State;
use shapemaker::*;

pub fn intro() -> Scene<State> {
    Scene::<State>::new("intro")
        .init(&|canvas, _| {
            canvas.clear();
            canvas.set_grid_size(16, 9);
            canvas.set_background(Black);

            let mut kicks = Layer::new("anchor kick");

            let kicks_in = canvas.world_region.resized(-2, -2);

            for (i, &corner) in kicks_in.corners().iter().enumerate() {
                kicks.set(format!("corner {i}"), SmallCircle(corner))
            }

            canvas.add_or_replace_layer(kicks);

            Ok(())
        })
        .on_note("anchor kick", &|canvas, ctx| {
            canvas
                .layer("anchor kick")?
                .paint_all_objects(Fill::Translucent(ctx.extra.kick_color, 1.0));

            ctx.animate_layer("anchor kick", 200, &|t, layer, _| {
                layer.objects.values_mut().for_each(
                    |ColoredObject { fill, .. }| {
                        *fill = fill.opacify(1.0 - t);
                    },
                );
                Ok(())
            });

            Ok(())
        })
        .on_note("bass", &|canvas, ctx| {
            let pitch = ctx
                .notes_of_stem("bass")
                .find(|note| note.is_on())
                .map(|note| note.pitch);

            let area = (2, 2);
            let bounds = canvas.world_region.resized(-2, -2);
            ctx.extra.bass_pattern_at = match pitch {
                Some(32 | 33 | 34) => bounds.starting_from_topleft(area),
                Some(39) => bounds.starting_from_topright(area),
                Some(35) => bounds.starting_from_bottomleft(area),
                Some(42 | 41) => bounds.starting_from_bottomright(area),
                _ => bounds.starting_from_bottomleft(area),
            }
            .unwrap();

            let mut bass = canvas.random_layer_within(
                &mut ctx.extra.rng,
                "bass",
                &ctx.extra.bass_pattern_at,
            );

            bass.paint_all_objects(Fill::Solid(Color::White));
            canvas.add_or_replace_layer(bass);

            Ok(())
        })
        .on_note("powerful clap hit, clap, perclap", &|canvas, ctx| {
            let mut claps = canvas.random_layer_within(
                &mut ctx.extra.rng,
                "claps",
                &Region::from_center_and_size(
                    canvas.world_region.center(),
                    (2, 2),
                )?,
            );

            claps.paint_all_objects(Fill::Solid(Color::Red));
            canvas.add_or_replace_layer(claps);
            Ok(())
        })
        .on_note(
            "rimshot, glitchy percs, hitting percs, glitchy percs",
            &|canvas, ctx| {
                let mut foley = canvas.random_layer_within(
                    &mut ctx.extra.rng,
                    "percs",
                    &Region::from_center_and_size(
                        canvas.world_region.center(),
                        (2, 2),
                    )?,
                );
                foley.paint_all_objects(Fill::Translucent(Color::Red, 0.5));
                canvas.add_or_replace_layer(foley);
                Ok(())
            },
        )
        .on_note("qanda", &|canvas, ctx| {
            let canvas_line_width = canvas.object_sizes.default_line_width;
            let mut qanda = canvas.random_curves_within(
                &mut ctx.extra.rng,
                "qanda",
                &Region::from_center_and_size(
                    canvas.world_region.center(),
                    (4, 4),
                )?,
                3..=5,
            );
            qanda.paint_all_objects(Fill::Solid(Color::Orange));
            qanda.object_sizes.default_line_width =
                canvas_line_width * 4.0 * ctx.stem("qanda").velocity_relative();

            canvas.add_or_replace_layer(qanda);
            Ok(())
        })
        .on_note("brokenup", &|canvas, ctx| {
            let canvas_line_width = canvas.object_sizes.default_line_width;
            let mut brokenup = canvas.random_curves_within(
                &mut ctx.extra.rng,
                "brokenup",
                &ctx.extra.bass_pattern_at.translated(0, -2),
                3..=5,
            );
            brokenup.paint_all_objects(Fill::Solid(Color::Yellow));
            brokenup.object_sizes.default_line_width = canvas_line_width
                * 4.0
                * ctx.stem("brokenup").velocity_relative();

            canvas.add_or_replace_layer(brokenup);
            Ok(())
        })
        .on_note("goup", &|canvas, ctx| {
            let canvas_line_width = canvas.object_sizes.default_line_width;

            let area = Region::from_center_and_size(
                canvas.world_region.center(),
                (6, 6),
            )?;
            let hole = area.resized(-4, -4);

            let mut goup = canvas.random_curves_within(
                &mut ctx.extra.rng,
                "goup",
                &area,
                3..=5,
            );

            goup.remove_all_objects_in(&hole);
            goup.paint_all_objects(Fill::Solid(Color::Green));
            goup.object_sizes.default_line_width =
                canvas_line_width * 4.0 * ctx.stem("goup").velocity_relative();

            canvas.add_or_replace_layer(goup);
            Ok(())
        })
        .on_note("ch", &|canvas, ctx| {
            let kicks_in = canvas.world_region.resized(-2, -2).enlarged(1, 1);

            let ch = canvas.layer_or_empty("ch");

            let ch_position = kicks_in
                .outline()
                .nth(ctx.stem("ch").playcount % kicks_in.outline().count())
                .unwrap_or(kicks_in.start);

            ch.set("hihat", Dot(ch_position).colored(Red));

            canvas.put_layer_on_top("ch");
            Ok(())
        })
}
