use measure_time::debug_time;

use super::SVGAttributesRenderable;
use crate::{ColorMapping, ObjectSizes, Transformation};
use std::collections::HashMap;

impl SVGAttributesRenderable for Transformation {
    const MULTIPLE_VALUES_JOIN_BY: &'static str = " ";

    fn render_to_svg_attributes(
        &self,
        _colormap: ColorMapping,
        _cell_size: usize,
        _object_sizes: ObjectSizes,
        _id: &str,
    ) -> anyhow::Result<HashMap<String, String>> {
        debug_time!("render_to_svg/transformation");
        Ok(HashMap::from([(
            "transform".to_string(),
            match self {
                Transformation::Scale(x, y) => format!("scale({}  {})", x, y),
                Transformation::Rotate(angle) => format!("rotate({})", angle),
                Transformation::Skew(x, y) => {
                    format!("skewX({}) skewY({})", x, y)
                }
                Transformation::Matrix(a, b, c, d, e, f) => {
                    format!("matrix({}, {}, {}, {}, {}, {})", a, b, c, d, e, f)
                }
            },
        )]))
    }
}
