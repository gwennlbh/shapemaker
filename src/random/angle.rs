use crate::Angle;

impl Angle {
    /// Generate a random angle in degrees
    pub fn random() -> Self {
        let angle = rand::random::<f32>() * 360.0;
        Self(angle)
    }
}
