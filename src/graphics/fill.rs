use crate::{rendering::svg, Angle, Color, ColorMapping};

#[derive(Debug, Clone, Copy)]
pub enum Fill {
    Solid(Color),
    Translucent(Color, f32),
    /// Hatches(color, angle, thickness_ratio, spacing)
    Hatches(Color, Angle, f32, f32),
    /// Dotted(color, diameter, spacing)
    Dotted(Color, f32, f32),
}

impl Color {
    pub fn solid(self) -> Fill {
        Fill::Solid(self)
    }

    pub fn translucent(self, opacity: f32) -> Fill {
        Fill::Translucent(self, opacity)
    }

    pub fn hatches(self, angle: Angle, thickness: f32, spacing: f32) -> Fill {
        Fill::Hatches(self, angle, thickness, spacing)
    }

    pub fn dotted(self, diameter: f32, spacing: f32) -> Fill {
        Fill::Dotted(self, diameter, spacing)
    }
}

// Operations that can be applied on fills.
pub trait FillOperations {
    fn opacify(&self, opacity: f32) -> Self;
}

impl FillOperations for Fill {
    fn opacify(&self, opacity: f32) -> Self {
        match self {
            Fill::Solid(color) => Fill::Translucent(*color, opacity),
            Fill::Translucent(color, _) => Fill::Translucent(*color, opacity),
            _ => *self,
        }
    }
}

impl FillOperations for Option<Fill> {
    fn opacify(&self, opacity: f32) -> Self {
        self.map(|fill| fill.opacify(opacity))
    }
}

impl Fill {
    pub fn bottom_up_hatches(color: Color, thickness: f32, spacing: f32) -> Self {
        Fill::Hatches(color, Angle(45.0), thickness, spacing)
    }

    pub fn pattern_id(&self) -> String {
        if let Fill::Hatches(color, angle, thickness, spacing) = self {
            return format!(
                "pattern-hatched-{}-{}-{}-{}",
                angle,
                color.name(),
                thickness,
                spacing
            );
        }
        if let Fill::Dotted(color, diameter, spacing) = self {
            return format!(
                "pattern-dotted-{}-{}-{}",
                color.name(),
                diameter,
                spacing
            );
        }
        String::from("")
    }

    pub fn pattern_definition(
        &self,
        colormapping: &ColorMapping,
    ) -> Option<svg::Node> {
        match self {
            Fill::Hatches(color, angle, size, thickness_ratio) => {
                let thickness = size * (2.0 * thickness_ratio);

                let pattern = svg::tag("pattern")
                    .attr("id", self.pattern_id())
                    .attr("patternUnits", "userSpaceOnUse")
                    .attr("height", size * 2.0)
                    .attr("width", size * 2.0)
                    .attr("viewBox", format!("0,0,{},{}", size, size))
                    .attr(
                        "patternTransform",
                        format!("rotate({})", (*angle - Angle(45.0)).degrees()),
                    )
                    // https://stackoverflow.com/a/55104220/9943464
                    .wrapping(vec![
                        svg::tag("polygon").fill(*color, colormapping).attr(
                            "points",
                            format!(
                                "0,0 {},0 0,{}",
                                thickness / 2.0,
                                thickness / 2.0
                            ),
                        ),
                        svg::tag("polygon").fill(*color, colormapping).attr(
                            "points",
                            format!(
                                "0,{} {},0 {},{} {},{}",
                                size,
                                size,
                                size,
                                thickness / 2.0,
                                thickness / 2.0,
                                size,
                            ),
                        ),
                    ])
                    .node();

                Some(pattern)
            }
            Fill::Dotted(color, diameter, spacing) => {
                let box_size = diameter + 2.0 * spacing;
                let pattern = svg::tag("pattern")
                    .attr("id", self.pattern_id())
                    .attr("patternUnits", "userSpaceOnUse")
                    .attr("height", box_size)
                    .attr("width", box_size)
                    .attr("viewBox", format!("0,0,{},{}", box_size, box_size))
                    .wrapping(vec![svg::tag("circle")
                        .fill(*color, colormapping)
                        .attr("cx", box_size / 2.0)
                        .attr("cy", box_size / 2.0)
                        .attr("r", diameter / 2.0)])
                    .node();

                Some(pattern)
            }
            _ => None,
        }
    }
}
