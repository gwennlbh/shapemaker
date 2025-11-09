use rand::{
    SeedableRng,
    rngs::StdRng,
    seq::{IndexedRandom, IteratorRandom},
};
use shapemaker::*;

fn main() {
    let seed = (0..100u64).choose(&mut rand::rng()).unwrap();
    // let seed = 84;
    let mut rng = StdRng::seed_from_u64(seed);
    let mut canvas = Canvas::new(16, 9);
    canvas.outer_padding = 30;

    schedule_hell::scenes::dices::setup(&mut canvas);

    schedule_hell::scenes::dices::place_dice(&mut canvas, (2, 0), 2);
    schedule_hell::scenes::dices::place_dice(&mut canvas, (2, 1), 1);
    schedule_hell::scenes::dices::place_dice(&mut canvas, (2, 2), 3);
    schedule_hell::scenes::dices::place_dice(&mut canvas, (1, 1), 4);
    schedule_hell::scenes::dices::place_dice(&mut canvas, (3, 1), 5);
    schedule_hell::scenes::dices::place_dice(&mut canvas, (3, 2), 6);

    schedule_hell::scenes::dices::iterate(&mut canvas, &mut rng);

    canvas.root().add(
        "seed",
        Text(CornerPoint(0, 0), format!("seed {seed}"), 10.0).colored(White),
    );

    canvas
        .render_to_svg_file("result.svg")
        .expect("Could not write SVG");
}
