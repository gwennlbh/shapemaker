use rand;
use shapemaker::*;

fn artwork() -> Canvas {
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
    canvas.set_background(Black);

    let draw_in = canvas.world_region.resized(-2, -2);

    // Strands

    let strands_in =
        Region::from_bottomleft(draw_in.bottomleft().translated(2, -1), (3, 3))
            .unwrap();

    canvas.n_random_curves_within(&mut rand::rng(), &strands_in, 30, "strands");

    for (i, obj) in canvas.layer("strands").objects.values_mut().enumerate() {
        obj.recolor(if i % 2 == 0 { Cyan } else { Pink });
        obj.filter(Filter::glow(4.0));
    }

    // Red dot

    let red_dot = Object::BigCircle(
        Region::from_topright(draw_in.topright().translated(-3, 1), (4, 3))
            .unwrap()
            .random_point(&mut rand::rng()),
    )
    .colored(Red)
    .filtered(Filter::glow(5.0));

    canvas.new_layer("red dot").add_anon(red_dot.clone());

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
        .filled(White.hatches(
            Angle::from_degrees(45.0),
            (i + 5) as f32 / 10.0,
            0.25,
        ))
        .add_to(hatches);
    }

    canvas
}

pub fn main() {
    artwork()
        .render_to_png("dna-analysis-machine.png", 480)
        .unwrap();
}

#[test]
fn test_artwork() {
    use insta;
    insta::assert_snapshot! { artwork().render_to_svg_string().unwrap() }
}
