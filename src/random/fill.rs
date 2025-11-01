use crate::{Color, Fill};

impl Fill {
    pub fn random_solid<R: rand::Rng>(
        rng: &mut R,
        except: Option<Color>,
    ) -> Self {
        Fill::Solid(match except {
            Some(color) => Color::random_except(rng, color),
            None => rng.random(),
        })
    }

    pub fn random_hatches<R: rand::Rng>(
        rng: &mut R,
        except: Option<Color>,
    ) -> Self {
        let hatch_size = rng.random_range(5..=100) as f32 * 1e-2;
        Fill::Hatches(
            match except {
                Some(color) => Color::random_except(rng, color),
                None => rng.random(),
            },
            rng.random(),
            hatch_size,
            // under a certain hatch size, we can't see the hatching if the ratio is not ½
            if hatch_size < 8.0 {
                0.5
            } else {
                rng.random_range(1..=4) as f32 / 4.0
            },
        )
    }
}

