use crate::*;
use std::iter;

pub fn shapes_shed() -> Canvas {
    let mut canvas = Canvas::new(vec![]);

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
    ]);

    canvas
}

pub fn colors_shed() -> Canvas {
    let mut canvas = Canvas::new(vec!["circles"]);
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

    for (color, point) in iter::zip(all_colors, canvas.world_region) {
        canvas
            .root()
            .add(Object::Rectangle(point, point).colored(color));
    }

    canvas
}

pub fn grid() -> Canvas {
    let mut canvas = Canvas::new(vec![]);
    canvas.set_grid_size(3, 3);
    canvas.set_background(Color::White);

    for point in canvas.world_region {
        canvas.root().add(Object::Dot(point).colored(Color::Black));
    }

    canvas
}

pub fn dna_analysis_machine() -> Canvas {
    let mut canvas = Canvas::with_colors(ColorMapping {
        black: "#000000".into(),
        white: "#ffffff".into(),
        red: "#cf0a2b".into(),
        green: "#22e753".into(),
        blue: "#2734e6".into(),
        yellow: "#f8e21e".into(),
        orange: "#f05811".into(),
        purple: "#6a24ec".into(),
        brown: "#a05634".into(),
        pink: "#e92e76".into(),
        gray: "#81a0a8".into(),
        cyan: "#4fecec".into(),
    });

    canvas.set_grid_size(16, 9);
    canvas.set_background(Color::Black);

    let draw_in = canvas.world_region.resized(-2, -2);

    // Strands

    let strands_in =
        Region::from_bottomleft(draw_in.bottomleft().translated(2, -1), (3, 3))
            .unwrap();

    canvas.add_layer(canvas.n_random_curves_within(&strands_in, 30, "strands"));

    for (i, obj) in canvas.layer("strands").objects.values_mut().enumerate() {
        obj.recolor(if i % 2 == 0 { Color::Cyan } else { Color::Pink });
        obj.filter(Filter::glow(4.0));
    }

    // Red dot

    let red_dot = Object::BigCircle(
        Region::from_topright(draw_in.topright().translated(-3, 0), (4, 3))
            .unwrap()
            .random_point(),
    )
    .colored(Color::Red)
    .filtered(Filter::glow(5.0));

    canvas.new_layer("red dot").add(red_dot.clone());

    // Hatched circles & squares

    let hatches = canvas.new_layer("hatches");

    for (i, point) in draw_in.except(&strands_in).enumerate() {
        if red_dot.region().contains(&point) {
            continue;
        }
        if rand::random() {
            Object::BigCircle(point)
        } else {
            Object::Rectangle(point, point)
        }
        .filled(Fill::Hatches(
            Color::White,
            Angle(45.0),
            (i + 5) as f32 / 10.0,
            0.25,
        ))
        .add_to(hatches);
    }

    canvas
}
