use rand::Rng;

use crate::{Point, Region};

impl Point {
    pub fn random(rng: &mut impl Rng, within: &Region) -> Self {
        within.ensure_nonempty().unwrap();
        Self(
            rng.random_range(within.x_range()),
            rng.random_range(within.y_range()),
        )
    }
}
