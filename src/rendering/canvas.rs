use super::renderable::SVGRenderable;
use crate::{graphics::canvas::Canvas, rendering::svg};
use measure_time::debug_time;
use resvg::usvg;
use std::sync::Arc;

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
                .attr("x", -(self.canvas_outter_padding as i32))
                .attr("y", -(self.canvas_outter_padding as i32))
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
                    -(self.canvas_outter_padding as i32),
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
        let mut pixmap = self.create_pixmap(width, height);

        let parsed_svg = &svg_to_usvg_tree(contents, &self.fontdb)?;

        self.usvg_tree_to_pixmap(width, height, pixmap.as_mut(), parsed_svg);

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

    fn usvg_tree_to_pixmap(
        &self,
        width: u32,
        height: u32,
        mut pixmap_mut: tiny_skia::PixmapMut<'_>,
        parsed_svg: &resvg::usvg::Tree,
    ) {
        debug_time!("usvg_tree_to_pixmap");
        resvg::render(
            parsed_svg,
            tiny_skia::Transform::from_scale(
                width as f32 / self.width() as f32,
                height as f32 / self.height() as f32,
            ),
            &mut pixmap_mut,
        );
    }

    fn create_pixmap(&self, width: u32, height: u32) -> tiny_skia::Pixmap {
        debug_time!("create_pixmap");
        tiny_skia::Pixmap::new(width, height).expect("Failed to create pixmap")
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

fn svg_to_usvg_tree(
    svg: &str,
    fontdb: &Option<Arc<usvg::fontdb::Database>>,
) -> anyhow::Result<resvg::usvg::Tree> {
    debug_time!("svg_to_usvg_tree");
    Ok(resvg::usvg::Tree::from_str(
        svg,
        &match fontdb {
            Some(fontdb) => resvg::usvg::Options {
                fontdb: fontdb.clone(),
                ..Default::default()
            },
            None => resvg::usvg::Options::default(),
        },
    )?)
}

fn pixmap_to_png_data(pixmap: tiny_skia::Pixmap) -> anyhow::Result<Vec<u8>> {
    debug_time!("pixmap_to_png_data");
    Ok(pixmap.encode_png()?)
}

fn write_png_data(data: Vec<u8>, at: &str) -> anyhow::Result<()> {
    debug_time!("write_png_data");
    std::fs::write(at, data)?;
    Ok(())
}
