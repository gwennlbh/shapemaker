use shapemaker::CornerPoint as P;
use shapemaker::*;

pub fn shapes_shed() -> Canvas {
    let mut canvas = Canvas::with_layers(vec![]);

    canvas.set_grid_size(3, 3);
    canvas.set_background(Color::White);

    canvas.layer_unchecked("root").add_objects([
        BigCircle(P(0, 0)).colored(Black),
        CurveOutward(P(1, 1), P(2, 0), 5.0).colored(Black),
        CurveInward(P(2, 1), P(3, 0), 5.0).colored(Black),
        SmallCircle(P(0, 1)).colored(Black),
        Line(P(1, 1), P(2, 2), 5.0).colored(Black),
        Rectangle(P(0, 2), P(0, 2)).colored(Black),
        Dot(CenterPoint(1, 2)).colored(Black),
        Polygon(
            P(2, 1),
            vec![
                LineSegment::Straight(P(3, 1)),
                LineSegment::Straight(P(3, 2)),
            ],
        )
        .colored(Black),
        CenteredText(P(2, 2), "Test".into(), 5.0).colored(Black),
    ]);

    canvas
}

pub fn colors_shed() -> Canvas {
    let mut canvas = Canvas::with_layers(vec!["circles"]);
    canvas.set_grid_size(3, 3);
    canvas.canvas_outer_padding = 0;
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
        Shape::Rectangle(point, point)
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
        Shape::Dot(point)
            .colored(Color::Black)
            .add_to(canvas.root());
    }

    canvas
}

fn main() {
    grid()
        .render_to_svg_file("grid.svg")
        .expect("Failed to render grid");
    println!("Rendered grid.svg");
    colors_shed()
        .render_to_svg_file("colorshed.svg")
        .expect("Failed to render colors_shed");
    println!("Rendered colorshed.svg");
    shapes_shed()
        .render_to_svg_file("shapeshed.svg")
        .expect("Failed to render shapes_shed");
    println!("Rendered shapeshed.svg");
    shapes_shed()
        .render_to_png("shapeshed.png", 1000)
        .expect("Failed to render shapes_shed as PNG");
    println!("Rendered shapeshed.png");
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
