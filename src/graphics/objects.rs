use crate::{Angle, Fill, Filter, Point, Region, Transformation};
use anyhow::anyhow;
use itertools::Itertools;
use std::fmt::Display;
#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use super::{Color, fill::FillOperations};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineSegment {
    Straight(Point),
    InwardCurve(Point),
    OutwardCurve(Point),
}

#[derive(Debug, Clone)]
pub enum Object {
    Polygon(Point, Vec<LineSegment>),
    Line(Point, Point, f32),
    CurveOutward(Point, Point, f32),
    CurveInward(Point, Point, f32),
    SmallCircle(Point),
    BigDot(Point),
    Dot(Point),
    BigCircle(Point),
    Text(Point, String, f32),
    CenteredText(Point, String, f32),
    // FittedText(Region, String),
    Rectangle(Point, Point),
    Image(Region, String),
    RawSVG(String),
    // Tiling(Region, Box<Object>),
}

impl Object {
    pub fn filled(self, fill: Fill) -> ColoredObject {
        ColoredObject::from((self, Some(fill)))
    }

    pub fn colored(self, color: Color) -> ColoredObject {
        ColoredObject::from((self, None)).colored(color)
    }

    pub fn filtered(self, filter: Filter) -> ColoredObject {
        ColoredObject::from((self, None)).filtered(filter)
    }

    pub fn transform(self, transformation: Transformation) -> ColoredObject {
        ColoredObject::from((self, None)).transformed(transformation)
    }
}

#[derive(Debug, Clone)]
pub struct ColoredObject {
    pub object: Object,
    pub fill: Option<Fill>,
    pub filters: Vec<Filter>,
    pub transformations: Vec<Transformation>,
    pub tags: Vec<String>,
    pub clip_to: Option<Region>,
}

impl ColoredObject {
    pub fn filtered(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn transformed(mut self, transformation: Transformation) -> Self {
        self.transformations.push(transformation);
        self
    }

    pub fn filled(mut self, fill: Fill) -> Self {
        self.fill = Some(fill);
        self
    }

    pub fn colored(mut self, color: Color) -> Self {
        self.fill = Some(Fill::Solid(color));
        self
    }

    pub fn opacified(mut self, opacity: f32) -> Self {
        if let Some(fill) = &mut self.fill {
            *fill = fill.opacify(opacity);
        }
        self
    }

    pub fn clipped_to(mut self, region: impl Into<Region>) -> Self {
        self.clip_to = Some(region.into());
        self
    }

    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    pub fn refill(&mut self, fill: Fill) {
        self.fill = Some(fill);
    }

    pub fn recolor(&mut self, color: Color) {
        self.fill = Some(Fill::Solid(color))
    }

    pub fn filter(&mut self, filter: Filter) {
        self.filters.push(filter)
    }

    pub fn rotate(&mut self, angle: Angle) {
        self.transformations
            .push(Transformation::Rotate(angle.degrees()))
    }

    pub fn set_rotation(&mut self, angle: Angle) {
        self.transformations
            .retain(|t| !matches!(t, Transformation::Rotate(_)));
        self.transformations
            .push(Transformation::Rotate(angle.degrees()))
    }

    pub fn region(&self) -> Region {
        self.object.region()
    }

    pub fn tag(&mut self, tag: impl Display) {
        self.tags.push(format!("{tag}"));
    }

    pub fn remove_tag(&mut self, tag: impl Display) {
        let tag_str = format!("{tag}");
        self.tags.retain(|t| t != &tag_str);
    }

    pub fn tagged(mut self, tag: impl Display) -> Self {
        self.tags.push(format!("{tag}"));
        self
    }

    pub fn has_tag(&self, tag: impl Display) -> bool {
        let tag_str = format!("{tag}");
        self.tags.iter().any(|t| t == &tag_str)
    }
}

impl std::fmt::Display for ColoredObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ColoredObject {
            object,
            fill,
            filters,
            transformations,
            tags,
            clip_to,
        } = self;

        if fill.is_some() {
            write!(f, "{:?} {:?}", fill.unwrap(), object)?;
        } else {
            write!(f, "transparent {:?}", object)?;
        }

        if !filters.is_empty() {
            write!(f, " with filters {:?}", filters)?;
        }

        if !transformations.is_empty() {
            write!(f, " with transformations {:?}", transformations)?;
        }

        if !tags.is_empty() {
            write!(f, "{}", tags.iter().map(|t| format!("#{t}")).join(" "))?;
        }

        if let Some(clip_to) = clip_to {
            write!(f, " (clipped to {:?})", clip_to)?;
        }

        Ok(())
    }
}

impl From<Object> for ColoredObject {
    fn from(value: Object) -> Self {
        ColoredObject {
            object: value,
            fill: None,
            filters: vec![],
            transformations: vec![],
            tags: vec![],
            clip_to: None,
        }
    }
}

impl From<(Object, Option<Fill>)> for ColoredObject {
    fn from((object, fill): (Object, Option<Fill>)) -> Self {
        ColoredObject {
            object,
            fill,
            filters: vec![],
            transformations: vec![],
            tags: vec![],
            clip_to: None,
        }
    }
}

#[cfg_attr(feature = "web", wasm_bindgen)]
#[derive(Debug, Clone, Copy)]
pub struct ObjectSizes {
    pub empty_shape_stroke_width: f32,
    pub small_circle_radius: f32,
    pub dot_radius: f32,
    pub default_line_width: f32,
}

impl Default for ObjectSizes {
    fn default() -> Self {
        Self {
            empty_shape_stroke_width: 0.5,
            small_circle_radius: 5.0,
            dot_radius: 2.0,
            default_line_width: 2.0,
        }
    }
}

impl Object {
    pub fn translate(&mut self, dx: i32, dy: i32) {
        match self {
            Object::Polygon(start, lines) => {
                start.translate(dx, dy);
                for line in lines {
                    match line {
                        LineSegment::InwardCurve(anchor)
                        | LineSegment::OutwardCurve(anchor)
                        | LineSegment::Straight(anchor) => {
                            anchor.translate(dx, dy)
                        }
                    }
                }
            }
            Object::Line(start, end, _)
            | Object::CurveInward(start, end, _)
            | Object::CurveOutward(start, end, _)
            | Object::Rectangle(start, end) => {
                start.translate(dx, dy);
                end.translate(dx, dy);
            }
            Object::Text(anchor, _, _)
            | Object::CenteredText(anchor, ..)
            | Object::Dot(anchor)
            | Object::BigDot(anchor) => anchor.translate(dx, dy),
            Object::BigCircle(center) | Object::SmallCircle(center) => {
                center.translate(dx, dy)
            }
            Object::Image(region, ..) => region.translate(dx, dy),
            Object::RawSVG(_) => {
                unimplemented!()
            }
        }
    }

    pub fn translate_with(&mut self, delta: (i32, i32)) {
        self.translate(delta.0, delta.1)
    }

    pub fn teleport(&mut self, x: i32, y: i32) {
        let (current_x, current_y) = self.region().start.xy::<i32>();
        let delta_x = x - current_x;
        let delta_y = y - current_y;
        self.translate(delta_x, delta_y);
    }

    pub fn teleport_with(&mut self, position: (i32, i32)) {
        self.teleport(position.0, position.1)
    }

    pub fn region(&self) -> Region {
        match self {
            Object::Polygon(start, lines) => {
                let mut region: Region = (start, start).into();
                for line in lines {
                    match line {
                        LineSegment::InwardCurve(anchor)
                        | LineSegment::OutwardCurve(anchor)
                        | LineSegment::Straight(anchor) => {
                            // println!(
                            //     "extending region {} with {}",
                            //     region,
                            //     Region::from((start, anchor))
                            // );
                            region = *region.max(&(start, anchor).into())
                        }
                    }
                }
                // println!("region for {:?} -> {}", self, region);
                region
            }
            Object::Line(s, e, _)
            | Object::CurveInward(s, e, _)
            | Object::CurveOutward(s, e, _) => {
                let (x1, y1, x2, y2) = (s.x(), s.y(), e.y(), e.x());

                let region = Region::new(
                    (x1.min(x2), y1.min(y2)),
                    (x1.max(x2), y1.max(y2)),
                )
                .map_err(|e| {
                    anyhow!("Could not construct region of {self:?}: {e:?}")
                })
                .unwrap();

                region.enlarged(
                    if region.width() > 1 { -1 } else { 0 },
                    if region.height() > 1 { -1 } else { 0 },
                )
            }
            Object::Rectangle(start, end) => {
                Region::new(*start, *end).unwrap().enlarged(-1, -1)
            }
            Object::Text(anchor, _, _)
            | Object::CenteredText(anchor, ..)
            | Object::Dot(anchor)
            | Object::BigDot(anchor)
            | Object::SmallCircle(anchor) => anchor.region(),
            Object::BigCircle(center) => center.region(),
            Object::Image(region, ..) => *region,
            Object::RawSVG(_) => {
                unimplemented!()
            }
        }
    }

    pub fn fillable(&self) -> bool {
        !matches!(
            self,
            Object::Line(..) | Object::CurveInward(..) | Object::CurveOutward(..)
        )
    }

    pub fn hatchable(&self) -> bool {
        self.fillable() && !matches!(self, Object::Dot(..))
    }

    pub fn intersects_with(&self, line: Object) -> bool {
        match (self, &line) {
            (Object::Line(s1, e1, _), Object::Line(s2, e2, _)) => {
                let x1 = s1.x() as f32;
                let y1 = s1.y() as f32;
                let x2 = s2.x() as f32;
                let y2 = s2.y() as f32;
                let x3 = e1.x() as f32;
                let y3 = e1.y() as f32;
                let x4 = e2.x() as f32;
                let y4 = e2.y() as f32;

                let d1 = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
                let d2 = (x4 - x3) * (y2 - y3) - (y4 - y3) * (x2 - x3);
                let d3 = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1);
                let d4 = (x2 - x1) * (y4 - y1) - (y2 - y1) * (x4 - x1);

                return ((d1 > 0. && d2 < 0.) || (d1 < 0. && d2 > 0.))
                    && ((d3 > 0. && d4 < 0.) || (d3 < 0. && d4 > 0.));
            }
            _ => {
                unimplemented!(
                    "Intersection not implemented for {self:?} and {:?}",
                    line.clone()
                )
            }
        }
    }
}
