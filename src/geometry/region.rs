use crate::{Object, Point};
use anyhow::{Error, Result, anyhow, format_err};
#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use super::Axis;

#[cfg_attr(feature = "web", wasm_bindgen)]
#[derive(Debug, Clone, Default, Copy)]
pub struct Region {
    pub start: Point,
    pub end: Point,
}

impl Region {
    /// iterates from left to right then top to bottom (in a "row-major" order)
    pub fn iter(&self) -> RegionIterator {
        self.into()
    }

    /// Iterates all points except the ones specified in the `except` region
    pub fn except<'a>(
        &self,
        except: &'a Region,
    ) -> impl Iterator<Item = Point> + use<'a> {
        self.iter().filter(move |p| !except.contains(p))
    }

    pub fn iter_lower_triangle(&self) -> impl Iterator<Item = Point> {
        self.iter().filter(|Point(x, y)| x < y)
    }

    pub fn iter_upper_strict_triangle(&self) -> impl Iterator<Item = Point> {
        self.iter().filter(|Point(x, y)| x >= y)
    }

    /// Iterates all points outlining the region, in clockwise order starting from top-left
    pub fn outline(&self) -> impl Iterator<Item = Point> {
        self.top_edge()
            .chain(self.right_edge().skip(1))
            .chain(self.bottom_edge().rev().skip(1))
            .chain(self.left_edge().rev().skip(1))
    }

    pub fn top_edge(&self) -> impl DoubleEndedIterator<Item = Point> {
        (self.start.0..=self.end.0).map(move |x| Point(x, self.start.1))
    }

    pub fn bottom_edge(&self) -> impl DoubleEndedIterator<Item = Point> {
        (self.start.0..=self.end.0).map(move |x| Point(x, self.end.1))
    }

    pub fn left_edge(&self) -> impl DoubleEndedIterator<Item = Point> {
        (self.start.1..=self.end.1).map(move |y| Point(self.start.0, y))
    }

    pub fn right_edge(&self) -> impl DoubleEndedIterator<Item = Point> {
        (self.start.1..=self.end.1).map(move |y| Point(self.end.0, y))
    }

    /// Corners of the region's outline
    /// Does _not_ match .bottomright() etc., since
    /// this method takes into account that the region is inclusive
    /// topleft, topright, bottomright, bottomleft
    pub fn corners(&self) -> [Point; 4] {
        [
            self.topleft(),
            self.topright().translated(1, 0),
            self.bottomright().translated(1, 1),
            self.bottomleft().translated(0, 1),
        ]
    }

    pub fn is_empty(&self) -> bool {
        self.width() == 0 || self.height() == 0
    }

    pub fn ensure_nonempty(&self) -> Result<()> {
        if self.is_empty() {
            return Err(format_err!("Region {} is empty", self));
        }

        Ok(())
    }

    pub fn rectangle(&self) -> Object {
        Object::Rectangle(self.start, self.end)
    }

    pub fn center_coords(&self, cell_size: usize) -> (f32, f32) {
        let (x, y) = self.center().coords(cell_size);
        let (h, w) = self.size(cell_size);

        (x + (w / 2.0), y + (h / 2.0))
    }
}

pub struct RegionIterator {
    region: Region,
    current: Point,
}

impl IntoIterator for Region {
    type Item = Point;
    type IntoIter = RegionIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Iterator for RegionIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.0 > self.region.end.0 {
            self.current.0 = self.region.start.0;
            self.current.1 += 1;
        }
        if self.current.1 > self.region.end.1 {
            return None;
        }
        let result = self.current;
        self.current.0 += 1;
        Some(result)
    }
}

impl From<&Region> for RegionIterator {
    fn from(region: &Region) -> Self {
        Self {
            region: *region,
            current: region.start,
        }
    }
}

impl From<(&Point, &Point)> for Region {
    fn from(value: (&Point, &Point)) -> Self {
        Self {
            start: *value.0,
            end: *value.1,
        }
    }
}

impl From<(Point, Point)> for Region {
    fn from(value: (Point, Point)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}

impl From<((usize, usize), (usize, usize))> for Region {
    fn from(value: ((usize, usize), (usize, usize))) -> Self {
        Region {
            start: value.0.into(),
            end: value.1.into(),
        }
    }
}

impl std::ops::Sub for Region {
    type Output = (i32, i32);

    fn sub(self, rhs: Self) -> Self::Output {
        (
            (self.start.0 as i32 - rhs.start.0 as i32),
            (self.start.1 as i32 - rhs.start.1 as i32),
        )
    }
}

#[test]
fn test_sub_and_transate_coherence() {
    let a = Region::from_origin(Point(3, 3)).unwrap();
    let mut b = a;
    b.translate(2, 3);

    assert_eq!(b - a, (2, 3));
}

impl Region {
    pub fn new(
        start: impl Into<Point>,
        end: impl Into<Point>,
    ) -> Result<Self, Error> {
        let region = Self {
            start: start.into(),
            end: end.into(),
        };
        region.ensure_valid()
    }

    pub fn bottomleft(&self) -> Point {
        Point(self.start.0, self.end.1)
    }

    pub fn bottomright(&self) -> Point {
        Point(self.end.0, self.end.1)
    }

    pub fn topleft(&self) -> Point {
        Point(self.start.0, self.start.1)
    }

    pub fn topright(&self) -> Point {
        Point(self.end.0, self.start.1)
    }

    pub fn center(&self) -> Point {
        Point(
            (self.start.0 + self.end.0) / 2,
            (self.start.1 + self.end.1) / 2,
        )
    }

    pub fn max<'a>(&'a self, other: &'a Region) -> &'a Region {
        if self.within(other) { other } else { self }
    }

    pub fn merge<'a>(&'a self, other: &'a Region) -> Region {
        Region {
            start: Point(
                self.start.0.min(other.start.0),
                self.start.1.min(other.start.1),
            ),
            end: Point(self.end.0.max(other.end.0), self.end.1.max(other.end.1)),
        }
    }

    pub fn from_origin(end: Point) -> Result<Self> {
        Self::new((0, 0), end)
    }

    pub fn from_topleft(origin: Point, size: (usize, usize)) -> Result<Self> {
        Self::new(
            origin,
            origin.translated_by(Point::from(size).translated(-1, -1)),
        )
    }

    pub fn starting_from_topleft(&self, size: (usize, usize)) -> Result<Self> {
        Self::from_topleft(self.start, size)
    }

    pub fn from_bottomleft(origin: Point, size: (usize, usize)) -> Result<Self> {
        Self::from_topleft(origin.translated(0, -(size.1 as i32 - 1)), size)
    }

    pub fn starting_from_bottomleft(&self, size: (usize, usize)) -> Result<Self> {
        Self::from_bottomleft(self.bottomleft(), size)
    }

    pub fn from_bottomright(origin: Point, size: (usize, usize)) -> Result<Self> {
        Self::new(
            origin.translated(-(size.0 as i32 - 1), -(size.1 as i32 - 1)),
            origin,
        )
    }

    pub fn starting_from_bottomright(
        &self,
        size: (usize, usize),
    ) -> Result<Self> {
        Self::from_bottomright(self.bottomright(), size)
    }

    pub fn from_topright(origin: Point, size: (usize, usize)) -> Result<Self> {
        Self::from_topleft(origin.translated(-(size.0 as i32 - 1), 0), size)
    }

    pub fn starting_from_topright(&self, size: (usize, usize)) -> Result<Self> {
        Self::from_topright(self.topright(), size)
    }

    pub fn from_center_and_size(
        center: Point,
        size: (usize, usize),
    ) -> Result<Self> {
        let half_size = (size.0 / 2, size.1 / 2);
        Self::new(
            (center.0 - half_size.0, center.1 - half_size.1),
            (center.0 + half_size.0, center.1 + half_size.1),
        )
    }

    // panics if the region is invalid
    pub fn ensure_valid(self) -> Result<Self> {
        if self.start.0 > self.end.0 || self.start.1 > self.end.1 {
            return Err(format_err!(
                "Invalid region: start ({:?}) > end ({:?})",
                self.start,
                self.end
            ));
        }

        // check that no point's coordinate is too close to usize::MAX
        if vec![self.start.0, self.start.1, self.end.0, self.end.1]
            .iter()
            .any(|&coord| coord >= usize::MAX - 10)
        {
            return Err(format_err!(
                "Invalid region: coordinate very close to usize::MAX in region {:?}",
                self
            ));
        }

        Ok(self)
    }

    pub fn translate(&mut self, dx: i32, dy: i32) {
        *self = self.translated(dx, dy);
    }

    pub fn translated(&self, dx: i32, dy: i32) -> Self {
        Self {
            start: (
                (self.start.0 as i32 + dx).max(0) as usize,
                (self.start.1 as i32 + dy).max(0) as usize,
            )
                .into(),
            end: (
                (self.end.0 as i32 + dx).max(0) as usize,
                (self.end.1 as i32 + dy).max(0) as usize,
            )
                .into(),
        }
    }

    /// adds dx and dy to the end of the region (dx and dy are _not_ multiplicative but **additive** factors)
    pub fn enlarged(&self, add_x: i32, add_y: i32) -> Self {
        let resulting = Self {
            start: self.start,
            end: (
                (self.end.0.saturating_add_signed(add_x as _)),
                (self.end.1.saturating_add_signed(add_y as _)),
            )
                .into(),
        };

        resulting
            .ensure_valid()
            .map_err(|e| {
                anyhow!(
                    "Invalid enlargement of ({add_x}, {add_y}) on {self:?}: {e:?}"
                )
            })
            .unwrap()
    }

    /// resized is like enlarged, but transforms from the center, by first translating the region by (-dx, -dy)
    pub fn resized(&self, add_x: i32, add_y: i32) -> Self {
        self.translated(-add_x / 2, -add_y / 2)
            .enlarged(add_x, add_y)
    }

    pub fn x_range(&self) -> std::ops::RangeInclusive<usize> {
        self.start.0..=self.end.0
    }
    pub fn y_range(&self) -> std::ops::RangeInclusive<usize> {
        self.start.1..=self.end.1
    }

    pub fn x_range_without_last(&self) -> std::ops::Range<usize> {
        self.start.0..self.end.0
    }

    pub fn y_range_without_last(&self) -> std::ops::Range<usize> {
        self.start.1..self.end.1
    }

    pub fn within(&self, other: &Region) -> bool {
        self.start.0 >= other.start.0
            && self.start.1 >= other.start.1
            && self.end.0 <= other.end.0
            && self.end.1 <= other.end.1
    }

    pub fn clamped(&self, within: &Region) -> Region {
        Region {
            start: (
                self.start.0.max(within.start.0),
                self.start.1.max(within.start.1),
            )
                .into(),
            end: (self.end.0.min(within.end.0), self.end.1.min(within.end.1))
                .into(),
        }
    }

    pub fn width(&self) -> usize {
        if self.end.0 < self.start.0 {
            return 0;
        }

        self.end.0 - self.start.0 + 1
    }

    pub fn height(&self) -> usize {
        let (Point(_, sy), Point(_, ey)) = (self.start, self.end);

        if ey < sy {
            return 0;
        }

        ey.checked_sub(sy)
            .expect(&format!("{self:?} overflows when computing height"))
            .checked_add(1)
            .expect(&format!(
                "{self:?} overflows when adjusting height computation"
            ))
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    pub fn size(&self, cell_size: usize) -> (f32, f32) {
        (
            (self.width() * cell_size) as f32,
            (self.height() * cell_size) as f32,
        )
    }

    // goes from -width to width (inclusive on both ends)
    pub fn mirrored_width_range(&self) -> std::ops::RangeInclusive<i32> {
        let w = self.width() as i32;
        -w..=w
    }

    pub fn mirrored_height_range(&self) -> std::ops::RangeInclusive<i32> {
        let h = self.height() as i32;
        -h..=h
    }

    pub fn split(&self, along: Axis) -> (Region, Region) {
        match along {
            Axis::Horizontal => (
                Region {
                    start: self.start,
                    end: Point(self.end.0, self.end.1 / 2),
                },
                Region {
                    start: Point(self.start.0, self.end.1 / 2),
                    end: self.end,
                },
            ),
            Axis::Vertical => (
                Region {
                    start: self.start,
                    end: Point(self.end.0 / 2, self.end.1),
                },
                Region {
                    start: Point(self.end.0 / 2, self.start.1),
                    end: self.end,
                },
            ),
        }
    }
}

pub trait Containable<T> {
    fn contains(&self, value: &T) -> bool;
}

impl Containable<Point> for Region {
    fn contains(&self, value: &Point) -> bool {
        self.x_range().contains(&value.0) && self.y_range().contains(&value.1)
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.start, self.end)
    }
}
