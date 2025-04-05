#![allow(uncommon_codepoints)]

pub fn enabled_features() -> Vec<&'static str> {
    let mut features = vec![];
    #[cfg(feature = "vst")]
    features.push("vst");
    #[cfg(feature = "mp4")]
    features.push("mp4");
    #[cfg(feature = "cli")]
    features.push("cli");
    #[cfg(feature = "web")]
    features.push("web");
    features
}

#[cfg(feature = "cli")]
pub mod cli;
pub mod geometry;
pub mod graphics;
pub mod random;
pub mod rendering;
pub mod synchronization;
pub mod ui;
pub mod video;

#[cfg(feature = "web")]
pub mod wasm;

#[cfg(feature = "vst")]
pub mod vst;

pub use geometry::{Angle, Axis, Containable, Point, Region};
pub use graphics::{
    Canvas, Color, Color::*, ColorMapping, ColoredObject, Fill, Filter,
    FilterType, Layer, LineSegment, Object, ObjectSizes, Transformation,
};
pub use rendering::{
    fonts, CSSRenderable, SVGAttributesRenderable, SVGRenderable,
};
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
