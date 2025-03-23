use std::iter;

use crate::*;
use rand::Rng;

pub fn shapes_shed() -> Canvas {
    let mut canvas = Canvas::new(vec![]);

    canvas.set_grid_size(3, 3);
    canvas.set_background(Color::White);

    let root = canvas.layer("root");

    root.add_object("1", Object::BigCircle(Point(0, 0)).color(Color::Black));
    root.add_object(
        "2",
        Object::CurveOutward(Point(1, 1), Point(2, 0), 5.0).color(Color::Black),
    );
    root.add_object(
        "3",
        Object::CurveInward(Point(2, 1), Point(3, 0), 5.0).color(Color::Black),
    );
    root.add_object("4", Object::SmallCircle(Point(0, 1)).color(Color::Black));
    root.add_object(
        "5",
        Object::Line(Point(1, 1), Point(2, 2), 5.0).color(Color::Black),
    );
    root.add_object(
        "6",
        Object::Polygon(
            Point(2, 1),
            vec![
                LineSegment::Straight(Point(3, 1)),
                LineSegment::Straight(Point(3, 2)),
            ],
        )
        .color(Color::Black),
    );
    root.add_object(
        "7",
        Object::Rectangle(Point(0, 2), Point(0, 2)).color(Color::Black),
    );
    root.add_object("8", Object::Dot(Point(2, 3)).color(Color::Black));

    canvas
}

pub fn colors_shed() -> Canvas {
    let mut canvas = Canvas::new(vec!["circles"]);
    canvas.set_grid_size(3, 3);
    canvas.canvas_outter_padding = 0;

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

    let foregrounds = all_colors.iter();
    let backgrounds = all_colors.iter().cycle().skip(1);
    let colors = iter::zip(foregrounds, backgrounds);

    for ((color, bgcolor), point) in
        iter::zip(colors, canvas.world_region.iter())
    {
        println!("{}: {:?} {:?}", point, color, bgcolor);
        canvas
            .layer("circles")
            .add_object(color.name(), Object::BigCircle(point).color(*color));
        canvas.layer("root").add_object(
            format!("{}_bg", color.name()),
            Object::Rectangle(point, point).color(*bgcolor),
        );
    }

    canvas
}

pub fn grid() -> Canvas {
    let mut canvas = Canvas::new(vec![]);
    canvas.set_grid_size(3, 3);
    canvas.set_background(Color::White);
    for point in canvas.world_region.iter() {
        canvas.root().add_object(
            point.to_string(),
            Object::Dot(point).color(Color::Black),
        );
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

    let filaments_area =
        Region::from_bottomleft(draw_in.bottomleft().translated(2, -1), (3, 3))
            .unwrap();

    let red_circle_at =
        Region::from_topright(draw_in.topright().translated(-3, 0), (4, 3))
            .unwrap()
            .random_point();

    let mut hatches_layer = Layer::new("hatches");
    let mut red_dot_layer = Layer::new("red dot");

    for (i, point) in draw_in.iter().enumerate() {
        if filaments_area.contains(&point) {
            continue;
        }

        if point == red_circle_at {
            red_dot_layer.add_object(
                format!("red circle @ {}", point),
                Object::BigCircle(point)
                    .color(Color::Red)
                    .filter(Filter::glow(5.0)),
            );
        }

        hatches_layer.add_object(
            point,
            if rand::thread_rng().gen_bool(0.5) || point == red_circle_at {
                Object::BigCircle(point)
            } else {
                Object::Rectangle(point, point)
            }
            .paint(Fill::Hatched(
                Color::White,
                Angle(45.0),
                (i + 5) as f32 / 10.0,
                0.25,
            )),
        );
    }

    let mut filaments =
        canvas.n_random_curves_within(&filaments_area, 30, "splines");

    for (i, object) in filaments.objects.values_mut().enumerate() {
        object.recolor(if i % 2 == 0 { Color::Cyan } else { Color::Pink });
    }

    filaments.filter_all_objects(Filter::glow(4.0));

    canvas.layers.push(red_dot_layer);
    canvas.layers.push(hatches_layer);
    canvas.layers.push(filaments);
    canvas
}
