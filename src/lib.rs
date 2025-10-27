#![allow(uncommon_codepoints)]

pub fn enabled_features() -> Vec<&'static str> {
    let mut features = vec![];
    #[cfg(feature = "vst")]
    features.push("vst");
    #[cfg(feature = "video")]
    features.push("video");
    #[cfg(feature = "cli")]
    features.push("cli");
    #[cfg(feature = "web")]
    features.push("web");
    #[cfg(feature = "video-server")]
    features.push("video-server");
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
    Canvas, Color, Color::*, ColorMapping, ColoredObject, Fill, FillOperations,
    Filter, FilterType, Layer, LineSegment, Object, Object::*, ObjectSizes,
    Transformation,
};
pub use rendering::{
    fonts, CSSRenderable, SVGAttributesRenderable, SVGRenderable,
};
pub use video::{animation, context, Animation, AttachHooks, Scene, Video, Timestamp};

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
