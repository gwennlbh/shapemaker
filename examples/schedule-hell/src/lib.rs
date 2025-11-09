pub mod scenes;

use rand::{SeedableRng, rngs::SmallRng};
use shapemaker::*;

pub struct State {
    pub bass_pattern_at: Region,
    pub kick_color: Color,
    pub rng: SmallRng,
    pub cranks: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            bass_pattern_at: Region::from_topleft(CornerPoint(1, 1), (2, 2))
                .unwrap(),
            kick_color: Color::White,
            rng: SmallRng::seed_from_u64(0),
            cranks: 0,
        }
    }
}
