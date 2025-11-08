#![allow(uncommon_codepoints)]

pub fn enabled_features() -> Vec<&'static str> {
    let mut features = vec![];
    #[cfg(feature = "vst")]
    features.push("vst");
    #[cfg(feature = "video")]
    features.push("video");
    #[cfg(feature = "web")]
    features.push("web");
    #[cfg(feature = "video-server")]
    features.push("video-server");
    features
}

pub mod geometry;
pub mod graphics;
pub mod random;
pub mod rendering;
pub mod synchronization;
pub mod ui;

#[cfg(feature = "web")]
pub mod wasm;

#[cfg(feature = "vst")]
pub mod vst;

pub use geometry::{
    Angle, Axis, CenterPoint, Containable, CornerPoint, Norm, Point, Region,
};
pub use graphics::{
    Canvas, Color, Color::*, ColorMapping, ColoredObject, Fill, FillOperations,
    Filter, FilterType, Layer, LineSegment, Object, Object::*, ObjectSizes,
    Transformation,
};
pub use rendering::{
    CSSRenderable, SVGAttributesRenderable, SVGRenderable, fonts,
};
pub use synchronization::audio::MusicalDurationUnit::*;

#[cfg(feature = "video")]
pub mod video;
#[cfg(feature = "video")]
pub use video::{
    Animation, AttachHooks, Scene, Timestamp, Video, animation, context,
};

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
