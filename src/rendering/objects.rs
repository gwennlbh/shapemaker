use itertools::Itertools;
use measure_time::debug_time;

use crate::{Fill, Object, Shape};

use super::{
    CSSRenderable, SVGAttributesRenderable, renderable::SVGRenderable, svg,
};

impl SVGRenderable for Object {
    fn render_to_svg(
        &self,
        colormap: crate::ColorMapping,
        cell_size: usize,
        object_sizes: crate::graphics::objects::ObjectSizes,
        id: &str,
    ) -> anyhow::Result<svg::Node> {
        debug_time!("render_to_svg/colored_object");

        let plain_obj = match &self.shape {
            Shape::RawSVG { .. } => self.render_raw_svg(&colormap),
            _ => self.shape.render_to_svg(
                colormap.clone(),
                cell_size,
                object_sizes,
                id,
            )?,
        };

        let mut css = self
            .fill
            .render_to_css(&colormap.clone(), !self.shape.fillable());

        let object_svg = if !self.transformations.is_empty()
            || !self.filters.is_empty()
        {
            // transform-box is not supported by resvg yet
            // css += "transform-box: fill-box; transform-origin: 50% 50%;";

            let (center_x, center_y) =
                self.shape.region().center_coords(cell_size);

            css += &format!("transform-origin: {center_x}px {center_y}px;");

            css += self
                .filters
                .iter()
                .map(|f| f.render_to_css_filled(&colormap))
                .join(" ")
                .as_ref();

            svg::tag("g")
                .dataset("object", id)
                .with_attributes(self.transformations.render_to_svg_attributes(
                    colormap,
                    cell_size,
                    object_sizes,
                    id,
                )?)
                .wrapping(vec![plain_obj])
                .attr("style", &css)
                .into()
        } else {
            match plain_obj {
                svg::Node::Element(el) => el.attr("style", &css).into(),
                _ => plain_obj,
            }
        };

        if let Some(region) = &self.clip_to {
            Ok(svg::tag("g")
                .attr("clip-path", region.clip_path_id())
                .child(object_svg)
                .into())
        } else {
            Ok(object_svg)
        }
    }
}

impl Object {
    fn render_raw_svg(&self, colormap: &crate::ColorMapping) -> svg::Node {
        if let Shape::RawSVG { content, color } = &self.shape {
            let filled_svg = match &self.fill {
                Some(Fill::Solid(fill_color)) => {
                    content.replace(color, &fill_color.render(colormap))
                }
                _ => content.clone(),
            };

            svg::Node::SVG(filled_svg)
        } else {
            panic!("Called render_raw_svg on a non-RawSVG shape");
        }
    }
}
