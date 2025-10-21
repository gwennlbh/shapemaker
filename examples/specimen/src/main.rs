use shapemaker::*;

pub fn shapes_shed() -> Canvas {
    let mut canvas = Canvas::with_layers(vec![]);

    canvas.set_grid_size(3, 3);
    canvas.set_background(Color::White);

    canvas.layer("root").add_objects([
        Object::BigCircle(Point(0, 0)).colored(Color::Black),
        Object::CurveOutward(Point(1, 1), Point(2, 0), 5.0).colored(Color::Black),
        Object::CurveInward(Point(2, 1), Point(3, 0), 5.0).colored(Color::Black),
        Object::SmallCircle(Point(0, 1)).colored(Color::Black),
        Object::Line(Point(1, 1), Point(2, 2), 5.0).colored(Color::Black),
        Object::Rectangle(Point(0, 2), Point(0, 2)).colored(Color::Black),
        Object::Dot(Point(2, 3)).colored(Color::Black),
        Object::Polygon(
            Point(2, 1),
            vec![
                LineSegment::Straight(Point(3, 1)),
                LineSegment::Straight(Point(3, 2)),
            ],
        )
        .colored(Color::Black),
        Object::Text(Point(0, 0), "test".into(), 5.0).colored(Color::Black),
    ]);

    canvas
}

pub fn colors_shed() -> Canvas {
    let mut canvas = Canvas::with_layers(vec!["circles"]);
    canvas.set_grid_size(3, 3);
    canvas.canvas_outter_padding = 0;
    canvas.set_background(Color::White);

    let all_colors = vec![
        Color::Blue,
        Color::Cyan,
        Color::Yellow,
        Color::Orange,
        Color::Red,
        Color::Brown,
        Color::Purple,
        Color::Pink,
        Color::Green,
    ];

    for (color, point) in std::iter::zip(all_colors, canvas.world_region) {
        Object::Rectangle(point, point)
            .colored(color)
            .add_to(canvas.root());
    }

    canvas
}

pub fn grid() -> Canvas {
    let mut canvas = Canvas::with_layers(vec![]);
    canvas.set_grid_size(3, 3);
    canvas.set_background(Color::White);

    for point in canvas.world_region {
        Object::Dot(point)
            .colored(Color::Black)
            .add_to(canvas.root());
    }

    canvas
}

fn main() {
    grid()
        .render_to_svg_file("grid.svg")
        .expect("Failed to render grid");
    colors_shed()
        .render_to_svg_file("colorshed.svg")
        .expect("Failed to render colors_shed");
    shapes_shed()
        .render_to_svg_file("shapeshed.svg")
        .expect("Failed to render shapes_shed");
    shapes_shed()
        .render_to_png("shapeshed.png", 1000)
        .expect("Failed to render shapes_shed as PNG");
}

#[test]
fn test_grid() {
    use insta;
    insta::assert_snapshot! { grid().render_to_svg_string().unwrap() }
}

#[test]
fn test_colors_shed() {
    use insta;
    insta::assert_snapshot! { colors_shed().render_to_svg_string().unwrap() }
}

#[test]
fn test_shapes_shed() {
    use insta;
    insta::assert_snapshot! { shapes_shed().render_to_svg_string().unwrap() }
}
