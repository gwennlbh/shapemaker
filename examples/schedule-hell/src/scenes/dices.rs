use itertools::Itertools;
use rand::{
    Rng, SeedableRng,
    rngs::{SmallRng, StdRng},
    seq::{IndexedRandom, IteratorRandom},
};
use shapemaker::*;

use crate::State;

pub const DICES_GRID: Grid = Grid(3, 3);

pub fn dices() -> Scene<State> {
    Scene::<State>::new("dices")
        .init(&|canvas, ctx| {
            ctx.extra.cranks = 0;
            setup(canvas);

            Ok(())
        })
        .each_n_frame(4, &|canvas, ctx| {
            match ctx.extra.cranks {
                0 => place_dice(canvas, (2, 1), 1),
                1 => place_dice(canvas, (2, 0), 2),
                2 => place_dice(canvas, (2, 2), 3),
                3 => place_dice(canvas, (1, 1), 4),
                4 => place_dice(canvas, (3, 1), 5),
                5 => place_dice(canvas, (3, 2), 6),
                _ => return Ok(()),
            }

            ctx.extra.cranks += 1;

            Ok(())
        })
        .on_note("brokenup", &|canvas, ctx| {
            if ctx.extra.cranks < 6 {
                return Ok(());
            }

            iterate(canvas, &mut ctx.extra.rng);

            Ok(())
        })
}

pub fn setup(canvas: &mut Canvas) {
    // canvas.set_grid_size(
    //     canvas.grid_size.0 * DICES_GRID.0,
    //     canvas.grid_size.1 * DICES_GRID.1,
    // );
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
}

pub fn place_dice(canvas: &mut Canvas, at: (usize, usize), value: usize) {
    canvas.add_dice(
        at,
        match value {
            1 => vec![(1, 1)],
            2 => vec![(0, 0), (2, 2)],
            3 => vec![(0, 0), (1, 1), (2, 2)],
            4 => vec![(0, 0), (0, 2), (2, 2), (2, 0)],
            5 => vec![(0, 0), (0, 2), (1, 1), (2, 2), (2, 0)],
            6 => vec![(0, 0), (0, 1), (0, 2), (2, 0), (2, 1), (2, 2)],
            _ => panic!("These dices are six-sided"),
        },
    );
}

pub fn iterate(canvas: &mut Canvas, rng: &mut impl Rng) {
    canvas.layer_or_empty("connections").clear();

    let connection_groups_count = (1..=5).choose(rng).unwrap();

    let dice_dots: Vec<_> = canvas
        .layer("dices")
        .unwrap()
        .objects_sorted_owned()
        .collect();

    let dice_dots_positions: Vec<_> = dice_dots
        .iter()
        .map(|(_, dot)| dot.region().center())
        .collect();

    let mut group_seeds = vec![*dice_dots_positions.choose(rng).unwrap()];

    for _ in 1..connection_groups_count {
        let seed = dice_dots_positions
            .iter()
            .filter(|p| {
                group_seeds
                    .iter()
                    .all(|seed| p.distance_to(seed).norm() >= 5.0)
            })
            .choose(rng)
            .cloned();

        if let Some(seed) = seed {
            group_seeds.push(seed);
        }
    }

    for &seed in &group_seeds {
        canvas
            .layer_or_empty("debug")
            .add_anon(SmallCircle(seed).colored(Red));
    }

    for &seed in &group_seeds {
        let mut point_before = seed;

        let group_points: Vec<_> = dice_dots
            .iter()
            .map(|(_, o)| o)
            .filter(|o| {
                let closest_seed = group_seeds
                    .iter()
                    .min_by_key(|&&s| {
                        (o.shape.position().as_centered().distance_to(&s).norm()
                            * 100.0) as usize
                    })
                    .unwrap();

                closest_seed == &seed
            })
            .filter(|o| o.shape.position().as_centered() != seed)
            .map(|o| o.shape.position().as_centered())
            .collect();

        let mut points_to_connect = group_points.clone();

        for &point in &points_to_connect {
            canvas
                .layer("debug")
                .unwrap()
                .add_anon(Line(point, seed, 0.5).colored(Gray));
        }

        let mut backtrack_attempts = 0;

        while !points_to_connect.is_empty() {
            let candidates: Vec<_> = points_to_connect
                .iter()
                .filter(|&&p| {
                    !canvas.layer_or_empty("connections").has_object_that(
                        |line| match line.shape {
                            ref line @ Line(..) => {
                                !line.meets_endpoint_of_line(point_before)
                                    && !line.meets_endpoint_of_line(p)
                                    && Line(point_before, p, 1.0)
                                        .intersects_with(line.clone())
                            }
                            _ => false,
                        },
                    )
                })
                .collect();

            let next_point = candidates
                .chunk_by(|a, b| {
                    point_before.distance_to(a).norm() as usize
                        == point_before.distance_to(b).norm() as usize
                })
                .next()
                .and_then(|closest_points| closest_points.choose(rng))
                .cloned()
                .cloned();

            if let Some(next_point) = next_point {
                // println!(
                //     "{seed} Connecting {point_before} to {next_point} from {points_to_connect:?}"
                // );
                canvas
                    .layer_unchecked("connections")
                    .add_anon(Line(point_before, next_point, 3.0).colored(Cyan));
                points_to_connect.retain(|&p| p != next_point);
                point_before = next_point;
            } else {
                canvas
                    .layer_unchecked("debug")
                    .add_anon(SmallCircle(point_before).colored(Orange));

                for &candidate in &points_to_connect {
                    let collision = canvas
                        .layer_unchecked("connections")
                        .find_object(|line| match line.shape {
                            ref line @ Line(..) => {
                                !line.meets_endpoint_of_line(point_before)
                                    && !line.meets_endpoint_of_line(candidate)
                                    && Line(point_before, candidate, 1.0)
                                        .intersects_with(line.clone())
                            }
                            _ => false,
                        })
                        .cloned();

                    if let Some(Object {
                        shape: Line(s, e, _),
                        ..
                    }) = collision
                    {
                        // println!(
                        //     "{seed} Could not connect {point_before} to {candidate}, collides with line {s} to {e}"
                        // );

                        canvas
                            .layer_unchecked("debug")
                            .add_anon(Line(s, e, 1.0).colored(Orange));
                    }
                }

                if backtrack_attempts > 10 {
                    // println!(
                    //     "{seed} Giving up on this group after {backtrack_attempts} backtrack attempts"
                    // );
                    break;
                }

                backtrack_attempts += 1;
                point_before = group_points.iter().choose(rng).cloned().unwrap();
                // println!("{seed} Trying again from {point_before}");
            }
        }
    }

    canvas.reorder_layers(vec!["debug", "dices", "connections", "dices_bg"]);

    canvas.remove_layer("debug");

    canvas
        .render_to_svg_file("result.svg")
        .expect("Could not write SVG");
}

trait MyCanvas {
    fn add_dice(
        &mut self,
        place_at: (usize, usize),
        dots_at: Vec<(usize, usize)>,
    );
}

impl MyCanvas for Canvas {
    fn add_dice(
        &mut self,
        place_at: (usize, usize),
        dots_at: Vec<(usize, usize)>,
    ) {
        // println!("Adding dice at {place_at:?} with dots at {dots_at:?}");
        for (x, y) in dots_at {
            let at = Point::Center(x, y)
                .translated_by(Point::from(place_at).coords_from(&DICES_GRID));

            self.layer_or_empty("dices")
                .add_anon(SmallCircle(at).colored(White));
        }

        let dicebox = Region::from_topleft(
            Point::from(place_at).coords_from(&DICES_GRID),
            DICES_GRID.size(),
        )
        .unwrap();

        self.layer_or_empty("dices_bg").add_many_anon(
            dicebox
                .corners()
                .iter()
                .circular_tuple_windows()
                .map(|(&s, &e)| Line(s, e, 0.5).colored(Gray)),
        );
    }
}

pub struct Grid(pub usize, pub usize);

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
