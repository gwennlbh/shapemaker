#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use crate::Region;

use super::Angle;

#[cfg_attr(feature = "web", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn coords(&self, cell_size: usize) -> (f32, f32) {
        (
            self.0 as f32 * cell_size as f32,
            self.1 as f32 * cell_size as f32,
        )
    }

    pub fn region(&self) -> Region {
        Region::from((self.clone(), self.clone()))
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.0 = x;
        self.1 = y;
    }

    pub fn x(&self) -> usize {
        self.0
    }

    pub fn y(&self) -> usize {
        self.1
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
        Self(value.0, value.1)
    }
}

impl From<(&usize, &usize)> for Point {
    fn from(value: (&usize, &usize)) -> Self {
        Self(*value.0, *value.1)
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0 as usize, value.1 as usize)
    }
}

impl PartialEq<(usize, usize)> for Point {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
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
