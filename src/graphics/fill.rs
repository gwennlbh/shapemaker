use crate::{Angle, Color, ColorMapping};

#[derive(Debug, Clone, Copy)]
pub enum Fill {
    Solid(Color),
    Translucent(Color, f32),
    Hatches(Color, Angle, f32, f32),
    Dotted(Color, f32, f32),
}

// Operations that can be applied on fills.
// Applying them on Option<Fill> is also possible, and will return an Option<Fill>.
pub trait FillOperations {
    fn opacify(&self, opacity: f32) -> Self;
    fn bottom_up_hatches(color: Color, thickness: f32, spacing: f32) -> Self;
}

impl FillOperations for Fill {
    fn opacify(&self, opacity: f32) -> Self {
        match self {
            Fill::Solid(color) => Fill::Translucent(*color, opacity),
            Fill::Translucent(color, _) => Fill::Translucent(*color, opacity),
            _ => *self,
        }
    }

    fn bottom_up_hatches(color: Color, thickness: f32, spacing: f32) -> Self {
        Fill::Hatches(color, Angle(45.0), thickness, spacing)
    }
}

impl FillOperations for Option<Fill> {
    fn opacify(&self, opacity: f32) -> Self {
        self.as_ref().map(|fill| fill.opacify(opacity))
    }

    fn bottom_up_hatches(color: Color, thickness: f32, spacing: f32) -> Self {
        Some(Fill::bottom_up_hatches(color, thickness, spacing))
    }
}

impl Fill {
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
            return format!("pattern-dotted-{}-{}-{}", color.name(), diameter, spacing);
        }
        String::from("")
    }

    pub fn pattern_definition(
        &self,
        colormapping: &ColorMapping,
    ) -> Option<svg::node::element::Pattern> {
        match self {
            Fill::Hatches(color, angle, size, thickness_ratio) => {
                let thickness = size * (2.0 * thickness_ratio);

                let pattern = svg::node::element::Pattern::new()
                    .set("id", self.pattern_id())
                    .set("patternUnits", "userSpaceOnUse")
                    .set("height", size * 2.0)
                    .set("width", size * 2.0)
                    .set("viewBox", format!("0,0,{},{}", size, size))
                    .set(
                        "patternTransform",
                        format!("rotate({})", (*angle - Angle(45.0)).degrees()),
                    )
                    // https://stackoverflow.com/a/55104220/9943464
                    .add(
                        svg::node::element::Polygon::new()
                            .set(
                                "points",
                                format!("0,0 {},0 0,{}", thickness / 2.0, thickness / 2.0),
                            )
                            .set("fill", color.render(colormapping)),
                    )
                    .add(
                        svg::node::element::Polygon::new()
                            .set(
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
                            )
                            .set("fill", color.render(colormapping)),
                    );

                Some(pattern)
            }
            Fill::Dotted(color, diameter, spacing) => {
                let box_size = diameter + 2.0 * spacing;
                let pattern = svg::node::element::Pattern::new()
                    .set("id", self.pattern_id())
                    .set("patternUnits", "userSpaceOnUse")
                    .set("height", box_size)
                    .set("width", box_size)
                    .set("viewBox", format!("0,0,{},{}", box_size, box_size))
                    .add(
                        svg::node::element::Circle::new()
                            .set("cx", box_size / 2.0)
                            .set("cy", box_size / 2.0)
                            .set("r", diameter / 2.0)
                            .set("fill", color.render(colormapping)),
                    );

                Some(pattern)
            }
            _ => None,
        }
    }
}
