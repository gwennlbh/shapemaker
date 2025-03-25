use super::random_color;
use crate::{Angle, Color, Fill};
use rand::Rng;

impl Fill {
    pub fn random_solid(except: Option<Color>) -> Self {
        Fill::Solid(random_color(except))
    }

    pub fn random_hatches(except: Option<Color>) -> Self {
        let hatch_size = rand::thread_rng().gen_range(5..=100) as f32 * 1e-2;
        Fill::Hatches(
            random_color(except),
            Angle(rand::thread_rng().gen_range(0.0..360.0)),
            hatch_size,
            // under a certain hatch size, we can't see the hatching if the ratio is not ½
            if hatch_size < 8.0 {
                0.5
            } else {
                rand::thread_rng().gen_range(1..=4) as f32 / 4.0
            },
        )
    }
}
