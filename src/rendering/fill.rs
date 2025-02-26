use super::CSSRenderable;
use crate::{ColorMapping, Fill};

impl CSSRenderable for Fill {
    fn render_to_css_filled(&self, colormap: &ColorMapping) -> String {
        match self {
            Fill::Solid(color) => {
                format!("fill: {};", color.render(colormap))
            }
            Fill::Translucent(color, opacity) => {
                format!("fill: {}; opacity: {};", color.render(colormap), opacity)
            }
            Fill::Dotted(..) | Fill::Hatched(..) => {
                format!("fill: url(#{});", self.pattern_id())
            }
        }
    }

    fn render_to_css_stroked(&self, colormap: &ColorMapping) -> String {
        match self {
            Fill::Solid(color) => {
                format!("stroke: {}; fill: transparent;", color.render(colormap))
            }
            Fill::Translucent(color, opacity) => {
                format!(
                    "stroke: {}; opacity: {}; fill: transparent;",
                    color.render(colormap),
                    opacity
                )
            }
            Fill::Dotted(..) => unimplemented!(),
            Fill::Hatched(..) => unimplemented!(),
        }
    }
}
