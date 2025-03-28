use shapemaker::*;

fn main() {
    let mut canvas = Canvas::with_colors(ColorMapping {
        black: "#000000".to_string(),
        white: "#FFFFFF".to_string(),
        purple: "#da40f5".to_string(),
        ..Default::default()
    });
    canvas.set_grid_size(16, 9);
    canvas.set_background(Color::Black);
    canvas.object_sizes.dot_radius = 7.5;
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
            .colored(Color::White),
        );
    }

    let occlusions = canvas.new_layer("occlusions");

    for point in world.enlarged(1, 1) {
        occlusions.add(Object::Dot(point).colored(Color::Black));
    }

    let flickers = canvas.new_layer("flickers");

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
            .colored(Color::Purple),
        );
    }

    let mut flickers_occlusions = canvas.new_layer("flickers_occlusions");
    flickers_occlusions.object_sizes.dot_radius = 10.0;

    for point in world.enlarged(1, 1) {
        flickers_occlusions.add(Object::Dot(point).colored(Color::Black))
    }

    // TODO figure out why it just makes the lines disappear
    // canvas.root().filter_all_objects(Filter::glow(4.5));

    canvas.reorder_layers(vec![
        "flickers_occlusions",
        "flickers",
        "occlusions",
        "root",
    ]);
    std::fs::write(
        "./backbone.svg",
        canvas
            .render_to_svg(
                canvas.colormap.clone(),
                canvas.cell_size,
                canvas.object_sizes,
                "",
            )
            .unwrap()
            .to_string(),
    )
    .unwrap();
    // canvas.render_to_png("./backbone.png", 480).unwrap();
}
