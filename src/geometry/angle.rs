/// Angle, stored in degrees
#[derive(Debug, Clone, Copy, Default)]
pub struct Angle(pub(crate) f32);

impl Angle {
    pub const TURN: Self = Angle(360.0);

    pub fn from_degrees(degrees: f32) -> Self {
        Self(degrees)
    }

    pub fn from_radians(radians: f32) -> Self {
        Self::from_ratio(radians, std::f32::consts::TAU)
    }

    /// Creates an angle given an amount, and what a full turn is equal to
    /// ```
    /// use shapemaker::geometry::Angle;
    ///
    /// assert_eq!(Angle::from_ratio(0.5, 1.0).degrees() as usize, 180);
    /// assert_eq!(Angle::from_radians(std::f32::consts::TAU).degrees() as usize, 360);
    /// ```
    pub fn from_ratio(amount: f32, of: f32) -> Self {
        Self(amount * Self::TURN.0 / of)
    }

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

    pub fn cos_sin(&self) -> (f32, f32) {
        let rad = self.radians();
        (rad.cos(), rad.sin())
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
