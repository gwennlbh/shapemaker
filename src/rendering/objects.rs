use itertools::Itertools;
use measure_time::debug_time;

use crate::{
    graphics::objects::{LineSegment, ObjectSizes},
    ColoredObject, Object,
};

use super::{
    renderable::SVGRenderable, svg, CSSRenderable, SVGAttributesRenderable,
};

impl SVGRenderable for ColoredObject {
    fn render_to_svg(
        &self,
        colormap: crate::ColorMapping,
        cell_size: usize,
        object_sizes: crate::graphics::objects::ObjectSizes,
        id: &str,
    ) -> anyhow::Result<svg::Node> {
        debug_time!("render_to_svg/colored_object");
        let plain_obj = self.object.render_to_svg(
            colormap.clone(),
            cell_size,
            object_sizes,
            id,
        )?;

        let mut css = self
            .fill
            .render_to_css(&colormap.clone(), !self.object.fillable());

        if !self.transformations.is_empty() || !self.filters.is_empty() {
            let start = self.object.region().start.coords(cell_size);
            let (w, h) = (
                self.object.region().width() * cell_size,
                self.object.region().height() * cell_size,
            );

            css += "transform-box: fill-box;";

            css += self
                .filters
                .iter()
                .map(|f| f.render_to_css_filled(&colormap))
                .join(" ")
                .as_ref();

            Ok(svg::tag("g")
                .dataset("object", id)
                .attr(
                    "transform-origin",
                    &format!(
                        "{} {}",
                        start.0 + (w as f32 / 2.0),
                        start.1 + (h as f32 / 2.0)
                    ),
                )
                .with_attributes(self.transformations.render_to_svg_attributes(
                    colormap,
                    cell_size,
                    object_sizes,
                    id,
                )?)
                .wrapping(vec![plain_obj])
                .attr("style", &css)
                .into())
        } else {
            Ok(match plain_obj {
                svg::Node::Element(el) => el.attr("style", &css).into(),
                _ => plain_obj,
            })
        }
    }
}

impl SVGRenderable for Object {
    fn render_to_svg(
        &self,
        _colormap: crate::ColorMapping,
        cell_size: usize,
        object_sizes: crate::graphics::objects::ObjectSizes,
        id: &str,
    ) -> anyhow::Result<svg::Node> {
        debug_time!("render_to_svg/object");
        let rendered = match self {
            Object::Text(..) | Object::CenteredText(..) => {
                self.render_text(cell_size)
            }
            Object::Rectangle(..) => self.render_rectangle(cell_size),
            Object::Polygon(..) => self.render_polygon(cell_size),
            Object::Line(..) => self.render_line(cell_size),
            Object::CurveInward(..) | Object::CurveOutward(..) => {
                self.render_curve(cell_size)
            }
            Object::SmallCircle(..) => {
                self.render_small_circle(cell_size, object_sizes)
            }
            Object::Dot(..) => self.render_dot(cell_size, object_sizes),
            Object::BigCircle(..) => self.render_big_circle(cell_size),
            Object::Image(..) => self.render_image(cell_size),
            Object::RawSVG(..) => self.render_raw_svg(),
        };

        Ok(match rendered {
            svg::Node::Element(el) => el.dataset("object", id).into(),
            svg::Node::SVG(svg) => {
                if svg.trim().starts_with("<") {
                    let (before, after) =
                        svg.split_once(' ').unwrap_or(svg.split_once(">").expect("Malformed SVG tag, {svg} starts with < but doesn't contain a space or >"));
                    svg::Node::SVG(format!(
                        r#"{before} data-object="{id}" {after}"#
                    ))
                } else {
                    eprintln!("Malformed raw SVG, {svg} doesn't start with <");
                    svg::Node::SVG(svg)
                }
            }
            _ => {
                panic!("Expected Element or SVG, got {:?}", rendered);
            }
        })
    }
}

impl Object {
    fn render_image(&self, cell_size: usize) -> svg::Node {
        if let Object::Image(region, path) = self {
            return svg::tag("image")
                .coords(region.start.coords(cell_size))
                .attr("width", region.width() * cell_size)
                .attr("height", region.height() * cell_size)
                .attr("href", path.clone())
                .into();
        }

        panic!("Expected Image, got {:?}", self);
    }

    fn render_raw_svg(&self) -> svg::Node {
        if let Object::RawSVG(svg) = self {
            return svg::Node::SVG(svg.clone());
        }

        panic!("Expected RawSVG, got {:?}", self);
    }

    fn render_text(&self, cell_size: usize) -> svg::Node {
        match self {
            Object::Text(position, content, font_size)
            | Object::CenteredText(position, content, font_size) => {
                let centered = matches!(self, Object::CenteredText(..));

                svg::tag("text")
                    .coords(if centered {
                        position.center_coords(cell_size)
                    } else {
                        position.coords(cell_size)
                    })
                    .attr("font-size", format!("{}pt", font_size))
                    .attr("font-family", "Inconsolata")
                    .attr(
                        "dominant-baseline",
                        if centered { "middle" } else { "hanging" },
                    )
                    .attr(
                        "text-anchor",
                        if centered { "middle" } else { "start" },
                    )
                    .wrapping(vec![svg::Node::Text(content.to_string())])
                    .into()
            }
            _ => panic!("Expected Text, got {:?}", self),
        }
    }

    // fn render_fitted_text(&self, cell_size: usize) -> svg::Node {
    //     if let Object::FittedText(region, content) = self {
    //         let (x, y) = region.start.coords(cell_size);
    //         let width = region.width() * cell_size as f32;
    //         let height = region.height() * cell_size as f32;

    //         return Box::new(
    //             svg::node::element::Text::new(content.clone())
    //                 .set("x", x)
    //                 .set("y", y)
    //                 .set("")
    //                 .set("font-size", format!("{}pt", 10.0))
    //                 .set("font-family", "sans-serif"),
    //         );
    //     }

    //     panic!("Expected FittedText, got {:?}", self);
    // }

    fn render_rectangle(&self, cell_size: usize) -> svg::Node {
        if let Object::Rectangle(start, end) = self {
            return svg::tag("rect").region((start, end), cell_size).into();
        }

        panic!("Expected Rectangle, got {:?}", self);
    }

    fn render_polygon(&self, cell_size: usize) -> svg::Node {
        if let Object::Polygon(start, lines) = self {
            let mut path = svg::Path::new();
            path.move_to(*start, cell_size);
            for line in lines {
                match line {
                    LineSegment::Straight(end)
                    | LineSegment::InwardCurve(end)
                    | LineSegment::OutwardCurve(end) => {
                        path.line_to(*end, cell_size);
                    }
                };
            }
            path.close();
            return path.node();
        }

        panic!("Expected Polygon, got {:?}", self);
    }

    fn render_line(&self, cell_size: usize) -> svg::Node {
        if let Object::Line(start, end, width) = self {
            return svg::tag("line")
                .position_pair(*start, *end, cell_size)
                .attr("stroke-width", *width)
                .into();
        }

        panic!("Expected Line, got {:?}", self);
    }

    fn render_curve(&self, cell_size: usize) -> svg::Node {
        if let Object::CurveOutward(start, end, stroke_width)
        | Object::CurveInward(start, end, stroke_width) = self
        {
            let inward = matches!(self, Object::CurveInward(..));

            let (start_x, start_y) = start.coords(cell_size);
            let (end_x, end_y) = end.coords(cell_size);

            let midpoint = ((start_x + end_x) / 2.0, (start_y + end_y) / 2.0);
            let start_from_midpoint =
                (start_x - midpoint.0, start_y - midpoint.1);
            let end_from_midpoint = (end_x - midpoint.0, end_y - midpoint.1);

            let control = {
                let relative = (end_x - start_x, end_y - start_y);
                if start_from_midpoint.0 * start_from_midpoint.1 > 0.0
                    && end_from_midpoint.0 * end_from_midpoint.1 > 0.0
                {
                    if inward {
                        (
                            midpoint.0 + relative.0.abs() / 2.0,
                            midpoint.1 - relative.1.abs() / 2.0,
                        )
                    } else {
                        (
                            midpoint.0 - relative.0.abs() / 2.0,
                            midpoint.1 + relative.1.abs() / 2.0,
                        )
                    }
                // diagonal line is going like this: /
                } else if start_from_midpoint.0 * start_from_midpoint.1 < 0.0
                    && end_from_midpoint.0 * end_from_midpoint.1 < 0.0
                {
                    if inward {
                        (
                            midpoint.0 - relative.0.abs() / 2.0,
                            midpoint.1 - relative.1.abs() / 2.0,
                        )
                    } else {
                        (
                            midpoint.0 + relative.0.abs() / 2.0,
                            midpoint.1 + relative.1.abs() / 2.0,
                        )
                    }
                // line is horizontal
                } else if start_y == end_y {
                    (
                        midpoint.0,
                        midpoint.1
                            + (if inward { -1.0 } else { 1.0 })
                                * relative.0.abs()
                                / 2.0,
                    )
                // line is vertical
                } else if start_x == end_x {
                    (
                        midpoint.0
                            + (if inward { -1.0 } else { 1.0 })
                                * relative.1.abs()
                                / 2.0,
                        midpoint.1,
                    )
                } else {
                    unreachable!()
                }
            };

            let mut path = svg::Path::new();
            path.move_to(*start, cell_size);
            path.quadratic_curve_to(control, *end, cell_size);
            return path
                .element()
                .attr("stroke-width", format!("{stroke_width}"))
                .into();
        }

        panic!("Expected Curve, got {:?}", self);
    }

    fn render_small_circle(
        &self,
        cell_size: usize,
        object_sizes: ObjectSizes,
    ) -> svg::Node {
        if let Object::SmallCircle(center) = self {
            return svg::tag("circle")
                .center_position(*center, cell_size)
                .attr("r", object_sizes.small_circle_radius)
                .into();
        }

        panic!("Expected SmallCircle, got {:?}", self);
    }

    fn render_dot(
        &self,
        cell_size: usize,
        object_sizes: ObjectSizes,
    ) -> svg::Node {
        if let Object::Dot(center) = self {
            return svg::tag("circle")
                .center_position(*center, cell_size)
                .attr("r", object_sizes.dot_radius)
                .into();
        }

        panic!("Expected Dot, got {:?}", self);
    }

    fn render_big_circle(&self, cell_size: usize) -> svg::Node {
        if let Object::BigCircle(topleft) = self {
            let (cx, cy) = {
                let (x, y) = topleft.coords(cell_size);
                (x + cell_size as f32 / 2.0, y + cell_size as f32 / 2.0)
            };

            return svg::tag("circle")
                .attr("cx", cx)
                .attr("cy", cy)
                .attr("r", cell_size / 2)
                .into();
        }

        panic!("Expected BigCircle, got {:?}", self);
    }
}
