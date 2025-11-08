use rand::Rng;

use crate::{Point, Region};

impl Point {
    pub fn random(rng: &mut impl Rng, within: &Region) -> Self {
        within.ensure_nonempty().unwrap();
        Self::Corner(
            rng.random_range(within.x_range()),
            rng.random_range(within.y_range()),
        )
    }

    pub fn random_center(rng: &mut impl Rng, within: &Region) -> Self {
        within.ensure_nonempty().unwrap();
        Self::Center(
            rng.random_range(within.x_range()),
            rng.random_range(within.y_range()),
        )
    }
}
