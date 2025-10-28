use super::renderable::SVGRenderable;
use crate::{
    graphics::canvas::Canvas,
    rendering::{
        rasterization::{
            create_pixmap, pixmap_to_png_data, svg_to_usvg_tree,
            usvg_tree_to_pixmap, write_png_data,
        },
        svg,
    },
};
use measure_time::debug_time;

impl SVGRenderable for Canvas {
    fn render_to_svg(
        &self,
        colormap: crate::ColorMapping,
        cell_size: usize,
        object_sizes: crate::graphics::objects::ObjectSizes,
        _id: &str,
    ) -> anyhow::Result<svg::Node> {
        debug_time!("render_to_svg/canvas");
        let background_color = self.background.unwrap_or_default();
        let mut svg = svg::tag("svg").attr("xmlns", "http://www.w3.org/2000/svg");

        svg.add(
            svg::tag("rect")
                .attr("x", -(self.canvas_outer_padding as i32))
                .attr("y", -(self.canvas_outer_padding as i32))
                .attr("width", self.width())
                .attr("height", self.height())
                .attr("fill", background_color.render(&self.colormap)),
        );

        for layer in self.layers.iter().filter(|layer| !layer.hidden).rev() {
            svg.add(layer.render_to_svg(
                colormap.clone(),
                cell_size,
                layer.object_sizes,
                layer.name.as_str(),
            )?);
        }

        let mut defs = svg::tag("defs");
        for filter in self.unique_filters() {
            defs.add(filter.render_to_svg(
                colormap.clone(),
                cell_size,
                object_sizes,
                "",
            )?);
        }

        for pattern_fill in self.unique_pattern_fills() {
            if let Some(patterndef) =
                pattern_fill.pattern_definition(&self.colormap)
            {
                defs.add(patterndef);
            }
        }

        svg.add(defs);

        Ok(svg
            .attr(
                "viewBox",
                format!(
                    "{0} {0} {1} {2}",
                    -(self.canvas_outer_padding as i32),
                    self.width(),
                    self.height()
                ),
            )
            .attr("width", self.width())
            .attr("height", self.height())
            .into())
    }
}

impl Canvas {
    pub fn svg_to_pixmap(
        &self,
        width: u32,
        height: u32,
        contents: &str,
    ) -> anyhow::Result<tiny_skia::Pixmap> {
        let mut pixmap = create_pixmap(width, height);

        let parsed_svg = &svg_to_usvg_tree(contents, &self.fontdb)?;

        usvg_tree_to_pixmap(self.dimensions(), pixmap.as_mut(), parsed_svg);

        Ok(pixmap)
    }

    pub fn render_to_pixmap(
        &mut self,
        width: u32,
        height: u32,
    ) -> anyhow::Result<tiny_skia::Pixmap> {
        let svg_contents = self
            .render_to_svg(
                self.colormap.clone(),
                self.cell_size,
                self.object_sizes,
                "",
            )?
            .to_string();
        self.svg_to_pixmap(width, height, &svg_contents)
    }

    // previous_frame_at gives path to the previously rendered frame, which allows to copy on cache hits instead of having to re-write bytes again
    pub fn render_to_png(
        &mut self,
        at: &str,
        resolution: u32,
    ) -> anyhow::Result<()> {
        debug_time!("render_to_png");
        let (width, height) = self.resolution_to_size(resolution);

        self.render_to_pixmap(width, height).and_then(|pixmap| {
            pixmap_to_png_data(pixmap).and_then(|data| write_png_data(data, at))
        })
    }

    pub fn render_to_svg_string(&mut self) -> anyhow::Result<String> {
        debug_time!("render_to_svg_string");

        let rendered = self.render_to_svg(
            self.colormap.clone(),
            self.cell_size,
            self.object_sizes,
            "",
        )?;

        Ok(rendered.to_string())
    }

    pub fn render_to_svg_file(&mut self, at: &str) -> anyhow::Result<()> {
        debug_time!("render_to_svg_file");

        std::fs::write(at, self.render_to_svg_string()?)?;

        Ok(())
    }
}
