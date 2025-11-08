use num::FromPrimitive;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use crate::{
    Point::{Center, Corner},
    Region,
};

use super::Angle;

#[cfg_attr(feature = "web", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Point {
    Corner(usize, usize),
    Center(usize, usize),
}

impl Default for Point {
    fn default() -> Self {
        Self::Corner(0, 0)
    }
}

impl Point {
    pub fn coords(&self, cell_size: usize) -> (f32, f32) {
        let (x, y) = self.xy::<f32>();
        let cell = cell_size as f32;

        match self {
            Point::Corner(..) => (x * cell, y * cell),
            Point::Center(..) => (x * cell + cell / 2.0, y * cell + cell / 2.0),
        }
    }

    pub fn as_centered(&self) -> Self {
        match self {
            Point::Corner(x, y) => Point::Center(*x, *y),
            Point::Center(_, _) => *self,
        }
    }

    pub fn as_corner(&self) -> Self {
        match self {
            Point::Center(x, y) => Point::Corner(*x, *y),
            Point::Corner(_, _) => *self,
        }
    }

    pub fn region(&self) -> Region {
        Region::from((self.clone(), self.clone()))
    }

    pub fn with(&self, x: usize, y: usize) -> Self {
        match self {
            Point::Corner(..) => Point::Corner(x, y),
            Point::Center(..) => Point::Center(x, y),
        }
    }

    pub fn set(&mut self, x: usize, y: usize) {
        *self = self.with(x, y);
    }

    pub fn set_x(&mut self, x: usize) {
        self.set(x, self.y());
    }

    pub fn with_x(&self, x: usize) -> Self {
        self.with(x, self.y())
    }

    pub fn increment_x(&mut self, by: isize) {
        self.set_x(self.x().saturating_add_signed(by));
    }

    pub fn set_y(&mut self, y: usize) {
        self.set(self.x(), y);
    }

    pub fn increment_y(&mut self, by: isize) {
        self.set_y(self.y().saturating_add_signed(by));
    }

    pub fn with_y(&self, y: usize) -> Self {
        self.with(self.x(), y)
    }

    pub fn xy<N: FromPrimitive>(&self) -> (N, N) {
        let (x, y) = match self {
            &Point::Corner(x, y) => (x, y),
            &Point::Center(x, y) => (x, y),
        };

        (N::from_usize(x).unwrap(), N::from_usize(y).unwrap())
    }

    pub fn x(&self) -> usize {
        self.xy().0
    }

    pub fn y(&self) -> usize {
        self.xy().1
    }

    pub fn translated(&self, dx: i32, dy: i32) -> Self {
        Self::from((
            (self.x() as i32 + dx) as usize,
            (self.y() as i32 + dy) as usize,
        ))
    }

    pub fn translated_by(&self, point: Point) -> Self {
        Self::from((self.x() + point.x(), self.y() + point.y()))
    }

    pub fn translate(&mut self, dx: i32, dy: i32) {
        *self = Self::from((
            (self.x() as i32 + dx) as usize,
            (self.y() as i32 + dy) as usize,
        ))
    }

    /// get SVG coordinates of the cell's center instead of its origin (top-left)
    #[deprecated = "Use a CenterPoint instead (WIP)"]
    pub fn center_coords(&self, cell_size: usize) -> (f32, f32) {
        let (x, y) = self.coords(cell_size);
        (x + cell_size as f32 / 2.0, y + cell_size as f32 / 2.0)
    }

    pub fn distance_to(&self, other: &Point) -> (usize, usize) {
        (
            self.x().abs_diff(other.x()) + 1,
            self.y().abs_diff(other.y()) + 1,
        )
    }

    pub fn rotated(&self, around: &Point, angle: Angle) -> Self {
        let (dx, dy) = (
            self.x() as f32 - around.x() as f32,
            self.y() as f32 - around.y() as f32,
        );

        let (cos, sin) = angle.cos_sin();
        let new_x = dx * cos - dy * sin;
        let new_y = dx * sin + dy * cos;

        Self::from((
            (new_x + around.x() as f32) as usize,
            (new_y + around.y() as f32) as usize,
        ))
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self::Corner(value.0, value.1)
    }
}

impl From<(&usize, &usize)> for Point {
    fn from(value: (&usize, &usize)) -> Self {
        Self::Corner(*value.0, *value.1)
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self::Corner(value.0 as usize, value.1 as usize)
    }
}

impl PartialEq<(usize, usize)> for Point {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.xy() == other.clone()
    }
}

impl Eq for Point {}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Point::Corner(x, y) => write!(f, "({x}, {y})"),
            Point::Center(x, y) => write!(f, "centered ({x}, {y})"),
        }
    }
}

impl std::ops::Sub for Point {
    type Output = (isize, isize);

    fn sub(self, rhs: Point) -> Self::Output {
        match (self, rhs) {
            (Corner(..), Corner(..)) => {}
            (Center(..), Center(..)) => {}
            _ => panic!("Cannot subtract CornerPoint and CenterPoint"),
        }

        let (x1, y1) = self.xy::<isize>();
        let (x2, y2) = rhs.xy::<isize>();

        (x1 - x2, y1 - y2)
    }
}

pub trait Norm {
    fn norm(&self) -> f32;
}

impl Norm for (usize, usize) {
    fn norm(&self) -> f32 {
        ((self.0 * self.0 + self.1 * self.1) as f32).sqrt()
    }
}
