use itertools::Itertools;
use rand::{Rng, seq::IteratorRandom};
use shapemaker::*;

const DICES_GRID: Grid = Grid(3, 3);

fn main() {
    let mut rng = rand::rng();
    let mut canvas = Canvas::new(9, 9);
    canvas.object_sizes.small_circle_radius = 7.0;
    canvas.colormap = ColorMapping {
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
    };

    dice(&mut canvas, (1, 1), vec![(1, 1)]);
    dice(&mut canvas, (1, 0), vec![(0, 0), (2, 2)]);
    dice(&mut canvas, (0, 1), vec![(0, 0), (0, 2), (2, 2), (2, 0)]);
    dice(
        &mut canvas,
        (2, 1),
        vec![(0, 0), (0, 2), (1, 1), (2, 2), (2, 0)],
    );
    dice(&mut canvas, (1, 2), vec![(0, 0), (1, 1), (2, 2)]);
    dice(
        &mut canvas,
        (2, 2),
        vec![(0, 0), (0, 1), (0, 2), (2, 2), (2, 1), (2, 0)],
    );

    let dice_dots: Vec<_> = canvas
        .layer("dices")
        .unwrap()
        .objects
        .iter()
        .map(|(_, dot)| dot.region().center())
        .collect();

    let connections = canvas.layer_or_empty("connections");

    for &point in &dice_dots {
        if rng.random_bool(0.5) {
            continue;
        }

        // Find another point that's not connected yet
        let other_point = dice_dots
            .iter()
            .filter(|&&p| p != point)
            .filter(|&&p| p.distance_to(&point).norm() <= 5.0)
            .filter(|&&p| {
                !connections.has_object_that(|obj| match obj.object {
                    Line(..) => obj.object.intersects_with(Line(point, p, 0.)),
                    _ => false,
                })
            })
            .choose(&mut rng);

        if let Some(&other_point) = other_point {
            connections.add_anon(
                Line(point, other_point, 3.0).colored(Cyan), // .opacified(0.5)
                                                             // .filtered(Filter::glow(10.0)),
            );
        }
    }

    // let world = canvas.world_region.clone();
    // let grid = canvas.layer_or_empty("grid");
    // for (p, _, _) in world.top_edge().tuples() {
    //     grid.add_anon(Line(p.with_y(0), p.with_y(9), 1.0).colored(Gray));
    // }
    // for (p, _, _) in world.left_edge().tuples() {
    //     grid.add_anon(Line(p.with_x(0), p.with_x(9), 1.0).colored(Gray));
    // }
    // grid.add_anon(Line(P(0, 9), P(9, 9), 1.0).colored(Gray));
    // grid.add_anon(Line(P(9, 0), P(9, 9), 1.0).colored(Gray));

    canvas.reorder_layers(vec!["connections", "dices", "dices_bg"]);

    canvas
        .render_to_svg_file("result.svg")
        .expect("Could not write SVG");
}

fn dice(
    canvas: &mut Canvas,
    place_at: (usize, usize),
    dots_at: Vec<(usize, usize)>,
) {
    for (x, y) in dots_at {
        let at = Point::Center(x, y)
            .translated_by(Point::from(place_at).coords_from(&DICES_GRID));

        canvas
            .layer_or_empty("dices")
            .add_anon(SmallCircle(at).colored(White));
    }

    let dicebox = Region::from_topleft(
        Point::from(place_at).coords_from(&DICES_GRID),
        DICES_GRID.size(),
    )
    .unwrap();

    canvas.layer_or_empty("dices_bg").add_many_anon(
        dicebox
            .corners()
            .iter()
            .circular_tuple_windows()
            .map(|(&s, &e)| Line(s, e, 0.5).colored(Gray)),
    );
}

struct Grid(usize, usize);

trait GridSnappable {
    // fn snapped_to(&self, grid: &Grid) -> Self;
    // fn snap_to(&mut self, grid: &Grid);
    fn coords_from(&self, grid: &Grid) -> Self;
}

impl GridSnappable for Point {
    fn coords_from(&self, Grid(sx, sy): &Grid) -> Self {
        self.with(self.x() * sx, self.y() * sy)
    }
}

impl Grid {
    fn size(&self) -> (usize, usize) {
        (self.0, self.1)
    }
}
