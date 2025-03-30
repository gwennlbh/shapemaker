use rand::{
    Rng,
    distr::{Distribution, StandardUniform, Uniform},
};

use crate::Angle;

impl Distribution<Angle> for StandardUniform {
    /// Generate a random angle in degrees
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Angle {
        Angle(Uniform::new(0.0, 360.0).unwrap().sample(rng))
    }
}
