use measure_time::debug_time;

use crate::{ColorMapping, Filter, FilterType};

use super::{CSSRenderable, renderable::SVGRenderable, svg};

impl SVGRenderable for Filter {
    fn render_to_svg(
        &self,
        _colormap: crate::ColorMapping,
        _cell_size: usize,
        _object_sizes: crate::graphics::objects::ObjectSizes,
        _id: &str,
    ) -> anyhow::Result<svg::Node> {
        {
            debug_time!("render_to_svg/filter");
            Ok(match self.kind {
                FilterType::Glow => {
                    // format!(
                    //     r#"
                    //     <filter id="glow">
                    //         <feGaussianBlur stdDeviation="{}" result="coloredBlur"/>
                    //         <feMerge>
                    //             <feMergeNode in="coloredBlur"/>
                    //             <feMergeNode in="SourceGraphic"/>
                    //         </feMerge>
                    //     </filter>
                    // "#,
                    //     2.5
                    // ) // TODO parameterize stdDeviation
                    svg::tag("filter").wrapping(vec![
                        // TODO parameterize stdDeviation
                        svg::tag("feGaussianBlur")
                            .attr("stdDeviation", self.parameter)
                            .attr("result", "coloredBlur"),
                        svg::tag("feMerge").wrapping(vec![
                            svg::tag("feMergeNode").attr("in", "coloredBlur"),
                            svg::tag("feMergeNode").attr("in", "SourceGraphic"),
                        ]),
                    ])
                }
                FilterType::NaturalShadow => {
                    /*
                                  <filter id="natural-shadow-filter" x="0" y="0" width="2" height="2">
                      <feOffset in="SourceGraphic" dx="3" dy="3" />
                      <feGaussianBlur stdDeviation="12" result="blur" />
                      <feMerge>
                        <feMergeNode in="blur" />
                        <feMergeNode in="SourceGraphic" />
                      </feMerge>
                    </filter>
                                   */
                    svg::tag("filter").wrapping(vec![
                        svg::tag("feOffset")
                            .attr("in", "SourceGraphic")
                            .attr("dx", self.parameter)
                            .attr("dy", self.parameter),
                        svg::tag("feGaussianBlur")
                            .attr("stdDeviation", self.parameter * 4.0)
                            .attr("result", "blur"),
                        svg::tag("feMerge").wrapping(vec![
                            svg::tag("feMergeNode").attr("in", "blur"),
                            svg::tag("feMergeNode").attr("in", "SourceGraphic"),
                        ]),
                    ])
                }
                FilterType::Saturation => {
                    /*
                    <filter id="saturation">
                        <feColorMatrix type="saturate" values="0.5"/>
                    </filter>
                    */
                    svg::tag("filter").wrapping(vec![
                        svg::tag("feColorMatrix")
                            .attr("type", "saturate")
                            .attr("values", self.parameter),
                    ])
                }
            }
            .attr("id", self.id())
            .attr("filterUnit", "userSpaceOnUse")
            .into())
        }
    }
}

impl CSSRenderable for Filter {
    fn render_to_css_filled(&self, _colormap: &ColorMapping) -> String {
        format!("filter: url(#{}); overflow: visible;", self.id())
    }

    fn render_to_css_stroked(&self, colormap: &ColorMapping) -> String {
        self.render_to_css_filled(colormap)
    }
}
