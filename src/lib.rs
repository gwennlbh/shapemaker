#![allow(uncommon_codepoints)]

pub mod cli;
pub use cli::ui;
pub mod examples;
pub mod geometry;
pub mod graphics;
pub mod random;
pub mod rendering;
pub mod synchronization;
pub mod video;
pub mod wasm;

pub use geometry::{Angle, Containable, Point, Region};
pub use graphics::{
    Canvas, Color, ColorMapping, ColoredObject, Fill, Filter, FilterType, Layer, LineSegment,
    Object, ObjectSizes, Transformation,
};
pub use rendering::{fonts, CSSRenderable, SVGAttributesRenderable, SVGRenderable};
pub use video::{animation, context, Animation, Video};

trait Toggleable {
    fn toggle(&mut self);
}

impl Toggleable for bool {
    fn toggle(&mut self) {
        *self = !*self;
    }
}

#[allow(unused)]
fn main() {}
