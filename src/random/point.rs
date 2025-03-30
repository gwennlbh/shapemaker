use rand::Rng;

use crate::{Point, Region};

impl Point {
    pub fn random(within: &Region) -> Self {
        within.ensure_nonempty().unwrap();
        Self(
            rand::rng().random_range(within.x_range()),
            rand::rng().random_range(within.y_range()),
        )
    }
}
