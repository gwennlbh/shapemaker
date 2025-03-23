use itertools::Itertools;

use crate::{
    graphics::objects::{LineSegment, ObjectSizes},
    ColoredObject, Object,
};

use super::{
    renderable::SVGRenderable, CSSRenderable, SVGAttributesRenderable,
};

impl SVGRenderable for ColoredObject {
    fn render_to_svg(
        &self,
        colormap: crate::ColorMapping,
        cell_size: usize,
        object_sizes: crate::graphics::objects::ObjectSizes,
        id: &str,
    ) -> anyhow::Result<svg::node::element::Element> {
        let mut group = self.object.render_to_svg(
            colormap.clone(),
            cell_size,
            object_sizes,
            id,
        )?;

        let attributes = group.get_attributes_mut();

        for (key, value) in self.transformations.render_to_svg_attributes(
            colormap.clone(),
            cell_size,
            object_sizes,
            id,
        )? {
            attributes.insert(key, value.into());
        }

        let start = self.object.region().start.coords(cell_size);
        let (w, h) = (
            self.object.region().width() * cell_size,
            self.object.region().height() * cell_size,
        );

        attributes.insert(
            "transform-origin".to_string(),
            format!(
                "{} {}",
                start.0 + (w as f32 / 2.0),
                start.1 + (h as f32 / 2.0)
            )
            .into(),
        );

        let mut css = String::new();
        if !matches!(self.object, Object::RawSVG(..)) {
            css = self
                .fill
                .render_to_css(&colormap.clone(), !self.object.fillable());
        }

        css += "transform-box: fill-box;";

        css += self
            .filters
            .iter()
            .map(|f| f.render_to_css_filled(&colormap))
            .join(" ")
            .as_ref();

        attributes.insert("style".into(), css.into());

        Ok(group)
    }
}

impl SVGRenderable for Object {
    fn render_to_svg(
        &self,
        _colormap: crate::ColorMapping,
        cell_size: usize,
        object_sizes: crate::graphics::objects::ObjectSizes,
        id: &str,
    ) -> anyhow::Result<svg::node::element::Element> {
        let group = svg::node::element::Group::new();

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

        Ok(group.set("data-object", id).add(rendered).into())
    }
}

impl Object {
    fn render_image(&self, cell_size: usize) -> Box<dyn svg::node::Node> {
        if let Object::Image(region, path) = self {
            let (x, y) = region.start.coords(cell_size);
            return Box::new(
                svg::node::element::Image::new()
                    .set("x", x)
                    .set("y", y)
                    .set("width", region.width() * cell_size)
                    .set("height", region.height() * cell_size)
                    .set("href", path.clone()),
            );
        }

        panic!("Expected Image, got {:?}", self);
    }

    fn render_raw_svg(&self) -> Box<dyn svg::node::Node> {
        if let Object::RawSVG(svg) = self {
            return svg.clone();
        }

        panic!("Expected RawSVG, got {:?}", self);
    }

    fn render_text(&self, cell_size: usize) -> Box<dyn svg::node::Node> {
        if let Object::Text(position, content, font_size)
        | Object::CenteredText(position, content, font_size) = self
        {
            let centered = matches!(self, Object::CenteredText(..));

            let coords = if centered {
                position.center_coords(cell_size)
            } else {
                position.coords(cell_size)
            };

            let mut node = svg::node::element::Text::new(content.clone())
                .set("x", coords.0)
                .set("y", coords.1)
                .set("font-size", format!("{}pt", font_size))
                .set("font-family", "Inconsolata");

            if centered {
                node = node
                    .set("text-anchor", "middle")
                    // FIXME does not work with imagemagick
                    .set("dominant-baseline", "middle");
            } else {
                // FIXME does not work with imagemagick
                // see https://legacy.imagemagick.org/discourse-server/viewtopic.php?t=31540
                node = node.set("dominant-baseline", "hanging")
            }

            return Box::new(node);
        }

        panic!("Expected Text, got {:?}", self);
    }

    // fn render_fitted_text(&self, cell_size: usize) -> Box<dyn svg:node::Node> {
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

    fn render_rectangle(&self, cell_size: usize) -> Box<dyn svg::node::Node> {
        if let Object::Rectangle(start, end) = self {
            return Box::new(
                svg::node::element::Rectangle::new()
                    .set("x", start.coords(cell_size).0)
                    .set("y", start.coords(cell_size).1)
                    .set("width", start.distances(end).0 * cell_size)
                    .set("height", start.distances(end).1 * cell_size),
            );
        }

        panic!("Expected Rectangle, got {:?}", self);
    }

    fn render_polygon(&self, cell_size: usize) -> Box<dyn svg::node::Node> {
        if let Object::Polygon(start, lines) = self {
            let mut path = svg::node::element::path::Data::new();
            path = path.move_to(start.coords(cell_size));
            for line in lines {
                path = match line {
                    LineSegment::Straight(end)
                    | LineSegment::InwardCurve(end)
                    | LineSegment::OutwardCurve(end) => {
                        path.line_to(end.coords(cell_size))
                    }
                };
            }
            path = path.close();
            return Box::new(svg::node::element::Path::new().set("d", path));
        }

        panic!("Expected Polygon, got {:?}", self);
    }

    fn render_line(&self, cell_size: usize) -> Box<dyn svg::node::Node> {
        if let Object::Line(start, end, width) = self {
            return Box::new(
                svg::node::element::Line::new()
                    .set("x1", start.coords(cell_size).0)
                    .set("y1", start.coords(cell_size).1)
                    .set("x2", end.coords(cell_size).0)
                    .set("y2", end.coords(cell_size).1)
                    .set("stroke-width", *width),
            );
        }

        panic!("Expected Line, got {:?}", self);
    }

    fn render_curve(&self, cell_size: usize) -> Box<dyn svg::node::Node> {
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

            return Box::new(
                svg::node::element::Path::new()
                    .set(
                        "d",
                        svg::node::element::path::Data::new()
                            .move_to(start.coords(cell_size))
                            .quadratic_curve_to((
                                control,
                                end.coords(cell_size),
                            )),
                    )
                    .set("stroke-width", format!("{stroke_width}")),
            );
        }

        panic!("Expected Curve, got {:?}", self);
    }

    fn render_small_circle(
        &self,
        cell_size: usize,
        object_sizes: ObjectSizes,
    ) -> Box<dyn svg::node::Node> {
        if let Object::SmallCircle(center) = self {
            return Box::new(
                svg::node::element::Circle::new()
                    .set("cx", center.coords(cell_size).0)
                    .set("cy", center.coords(cell_size).1)
                    .set("r", object_sizes.small_circle_radius),
            );
        }

        panic!("Expected SmallCircle, got {:?}", self);
    }

    fn render_dot(
        &self,
        cell_size: usize,
        object_sizes: ObjectSizes,
    ) -> Box<dyn svg::node::Node> {
        if let Object::Dot(center) = self {
            return Box::new(
                svg::node::element::Circle::new()
                    .set("cx", center.coords(cell_size).0)
                    .set("cy", center.coords(cell_size).1)
                    .set("r", object_sizes.dot_radius),
            );
        }

        panic!("Expected Dot, got {:?}", self);
    }

    fn render_big_circle(&self, cell_size: usize) -> Box<dyn svg::node::Node> {
        if let Object::BigCircle(topleft) = self {
            let (cx, cy) = {
                let (x, y) = topleft.coords(cell_size);
                (x + cell_size as f32 / 2.0, y + cell_size as f32 / 2.0)
            };

            return Box::new(
                svg::node::element::Circle::new()
                    .set("cx", cx)
                    .set("cy", cy)
                    .set("r", cell_size / 2),
            );
        }

        panic!("Expected BigCircle, got {:?}", self);
    }
}
