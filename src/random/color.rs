use crate::Color;
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

impl Distribution<Color> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        let candidates = Color::all();
        candidates[rng.random_range(0..candidates.len())]
    }
}

impl Color {
    pub fn random_except(rng: &mut impl Rng, except: Color) -> Self {
        let candidates = Color::all()
            .iter()
            .filter(|&&c| c != except)
            .cloned()
            .collect::<Vec<_>>();
        candidates[rng.random_range(0..candidates.len())]
    }
}
