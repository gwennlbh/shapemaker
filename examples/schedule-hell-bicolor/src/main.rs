use rand;
use shapemaker::*;

fn main() {
    let mut canvas = Canvas::with_colors(ColorMapping {
        black: "#676E95".to_string(),
        white: "#ffffff".to_string(),
        red: "#ff5572".to_string(),
        green: "#a9c77d".to_string(),
        blue: "#82AAFF".to_string(),
        yellow: "#FFCB6B".to_string(),
        orange: "#FFCB6B".to_string(),
        purple: "#C792EA".to_string(),
        brown: "#ff5572".to_string(),
        pink: "#C792EA".to_string(),
        gray: "#ffffff".to_string(),
        cyan: "#89DDFF".to_string(),
    });

    canvas.set_grid_size(16, 9);
    canvas.canvas_outter_padding = 0;

    let world = canvas.world_region.clone();
    let mut tiling = Layer::new("tiling");
    let mut shapes = Layer::new("shapes");

    for point in world {
        let bgcolor: Color = rand::random();
        let thickness = 7.0;

        let shape = match rand::random_range(1..=5) {
            1 => Object::BigCircle(point),
            2 => Object::CurveInward(point, point.translated(1, 1), thickness),
            3 => Object::CurveOutward(point, point.translated(1, 1), thickness),
            4 => Object::Line(point, point.translated(1, 1), thickness),
            5 => Object::Line(
                point.translated(0, 1),
                point.translated(1, 0),
                thickness,
            ),
            _ => panic!("souhldn't happend, update rand:: call"),
        };

        tiling.add_anon(Object::Rectangle(point, point).colored(bgcolor));
        shapes
            .add_anon(shape.colored(Color::random_except(&mut rand::rng(), bgcolor)));
    }

    canvas.add_layer(shapes);
    canvas.add_layer(tiling);

    canvas.reorder_layers(vec!["shapes", "tiling", "root"]);
    std::fs::write(
        "./bicolor.svg",
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
}
