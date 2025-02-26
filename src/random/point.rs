use rand::Rng;

use crate::{Point, Region};

impl Point {
    pub fn random(within: &Region) -> Self {
        within.ensure_nonempty().unwrap();
        Self(
            rand::thread_rng().gen_range(within.x_range()),
            rand::thread_rng().gen_range(within.y_range()),
        )
    }
}
