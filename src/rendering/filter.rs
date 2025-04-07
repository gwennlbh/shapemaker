use measure_time::debug_time;

use crate::{ColorMapping, Filter, FilterType};

use super::{renderable::SVGRenderable, CSSRenderable};

impl SVGRenderable for Filter {
    fn render_to_svg(
        &self,
        _colormap: crate::ColorMapping,
        _cell_size: usize,
        _object_sizes: crate::graphics::objects::ObjectSizes,
        _id: &str,
    ) -> anyhow::Result<svg::node::element::Element> {
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
                    svg::node::element::Filter::new()
                        .add(
                            // TODO parameterize stdDeviation
                            svg::node::element::FilterEffectGaussianBlur::new()
                                .set("stdDeviation", self.parameter)
                                .set("result", "coloredBlur"),
                        )
                        .add(
                            svg::node::element::FilterEffectMerge::new()
                                .add(
                                    svg::node::element::FilterEffectMergeNode::new()
                                        .set("in", "coloredBlur"),
                                )
                                .add(
                                    svg::node::element::FilterEffectMergeNode::new()
                                        .set("in", "SourceGraphic"),
                                ),
                        )
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
                    svg::node::element::Filter::new()
                        .add(
                            svg::node::element::FilterEffectOffset::new()
                                .set("in", "SourceGraphic")
                                .set("dx", self.parameter)
                                .set("dy", self.parameter),
                        )
                        .add(
                            svg::node::element::FilterEffectGaussianBlur::new()
                                .set("stdDeviation", self.parameter * 4.0)
                                .set("result", "blur"),
                        )
                        .add(
                            svg::node::element::FilterEffectMerge::new()
                                .add(
                                    svg::node::element::FilterEffectMergeNode::new()
                                        .set("in", "blur"),
                                )
                                .add(
                                    svg::node::element::FilterEffectMergeNode::new()
                                        .set("in", "SourceGraphic"),
                                ),
                        )
                }
                FilterType::Saturation => {
                    /*
                    <filter id="saturation">
                        <feColorMatrix type="saturate" values="0.5"/>
                    </filter>
                    */
                    svg::node::element::Filter::new().add(
                        svg::node::element::FilterEffectColorMatrix::new()
                            .set("type", "saturate")
                            .set("values", self.parameter),
                    )
                }
            }
            .set("id", self.id())
            .set("filterUnit", "userSpaceOnUse")
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
