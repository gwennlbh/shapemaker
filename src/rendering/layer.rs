use itertools::Itertools;
use measure_time::debug_time;

use super::{renderable::SVGRenderable, svg};
use crate::Layer;

impl SVGRenderable for Layer {
    fn render_to_svg(
        &self,
        colormap: crate::ColorMapping,
        cell_size: usize,
        object_sizes: crate::graphics::objects::ObjectSizes,
        id: &str,
    ) -> anyhow::Result<svg::Node> {
        debug_time!("render_to_svg/layer");
        let mut group = svg::tag("g").class("layer").dataset("layer", &self.name);
        for (object_id, object) in
            self.objects.iter().sorted_by_key(|(oid, _)| (*oid).clone()) {
            group.add(object.render_to_svg(
                colormap.clone(),
                cell_size,
                object_sizes,
                &format!("{}--{}", id, object_id),
            )?);
        }
        Ok(group.into())
    }
}
