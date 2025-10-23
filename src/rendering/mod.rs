pub mod canvas;
pub mod fill;
pub mod filter;
pub mod fonts;
pub mod layer;
pub mod objects;
pub mod renderable;
pub mod svg;
pub mod transform;

use measure_time::debug_time;
pub use renderable::{CSSRenderable, SVGAttributesRenderable, SVGRenderable};

pub fn stringify_svg(element: svg::Node) -> String {
    debug_time!("stringify_svg");

    return element.to_string();
}
