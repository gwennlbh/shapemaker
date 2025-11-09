use super::shapes::Shape;
use crate::{Angle, Fill, Filter, Point, Region, Transformation};
use itertools::Itertools;
use std::fmt::Display;
#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use super::{Color, fill::FillOperations};

impl Shape {
    pub fn filled(self, fill: Fill) -> Object {
        Object::from((self, Some(fill)))
    }

    pub fn colored(self, color: Color) -> Object {
        Object::from((self, None)).colored(color)
    }

    pub fn filtered(self, filter: Filter) -> Object {
        Object::from((self, None)).filtered(filter)
    }

    pub fn transform(self, transformation: Transformation) -> Object {
        Object::from((self, None)).transformed(transformation)
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub shape: Shape,
    pub fill: Option<Fill>,
    pub filters: Vec<Filter>,
    pub transformations: Vec<Transformation>,
    pub tags: Vec<String>,
    pub clip_to: Option<Region>,
}

impl Object {
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
        self.shape.region()
    }

    pub fn position(&self) -> Point {
        self.shape.position()
    }

    pub fn tag(&mut self, tag: impl Display) -> String {
        let tag = tag.to_string();

        self.tags.push(tag.clone());
        tag
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

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Object {
            shape: object,
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

impl From<Shape> for Object {
    fn from(value: Shape) -> Self {
        Object {
            shape: value,
            fill: None,
            filters: vec![],
            transformations: vec![],
            tags: vec![],
            clip_to: None,
        }
    }
}

impl From<(Shape, Option<Fill>)> for Object {
    fn from((object, fill): (Shape, Option<Fill>)) -> Self {
        Object {
            shape: object,
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
