
/// Angle, stored in degrees
#[derive(Debug, Clone, Copy, Default)]
pub struct Angle(pub f32);

impl Angle {
    pub const TURN: Self = Angle(360.0);

    pub fn degrees(&self) -> f32 {
        self.0
    }

    pub fn radians(&self) -> f32 {
        // tau better than pi, haters gonna hate <3
        self.0 * std::f32::consts::TAU / (Self::TURN.0 / 4.0)
    }

    pub fn turns(&self) -> f32 {
        self.0 / Self::TURN.0
    }

    pub fn without_turns(&self) -> Self {
        Self(self.0 % Self::TURN.0)
    }
}

impl std::ops::Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Self::Output {
        Angle(self.0 - rhs.0)
    }
}

impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}deg", self.degrees())
    }
}
