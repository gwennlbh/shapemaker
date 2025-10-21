use itertools::Itertools;
use measure_time::debug_time;

use super::renderable::SVGRenderable;
use crate::Layer;

impl SVGRenderable for Layer {
    fn render_to_svg(
        &self,
        colormap: crate::ColorMapping,
        cell_size: usize,
        object_sizes: crate::graphics::objects::ObjectSizes,
        id: &str,
    ) -> anyhow::Result<svg::node::element::Element> {
        debug_time!("render_to_svg/layer");
        let mut layer_group = svg::node::element::Group::new()
            .set("class", "layer")
            .set("data-layer", self.name.clone());

        for (object_id, obj) in
            self.objects.iter().sorted_by_key(|(oid, _)| (*oid).clone())
        {
            layer_group = layer_group.add(obj.render_to_svg(
                colormap.clone(),
                cell_size,
                object_sizes,
                &[id, object_id].join("--"),
            )?);
        }

        Ok(layer_group.into())
    }
}
