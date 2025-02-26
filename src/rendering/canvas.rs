use super::renderable::SVGRenderable;
use crate::graphics::canvas::Canvas;
use measure_time::{debug_time, info_time};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use resvg::usvg;
use std::sync::Arc;

impl SVGRenderable for Canvas {
    fn render_to_svg(
        &self,
        _colormap: crate::ColorMapping,
        _cell_size: usize,
        _object_sizes: crate::graphics::objects::ObjectSizes,
        _id: &str,
    ) -> anyhow::Result<svg::node::element::Element> {
        debug_time!("render_to_svg");
        let background_color = self.background.unwrap_or_default();
        let mut svg = svg::Document::new();
        svg = svg.add(
            svg::node::element::Rectangle::new()
                .set("x", -(self.canvas_outter_padding as i32))
                .set("y", -(self.canvas_outter_padding as i32))
                .set("width", self.width())
                .set("height", self.height())
                .set("fill", background_color.render(&self.colormap)),
        );

        for layer in self.layers.iter().filter(|layer| !layer.hidden).rev() {
            svg = svg.add(layer.render_to_svg(
                self.colormap.clone(),
                self.cell_size,
                layer.object_sizes,
                "",
            )?);
        }

        let mut defs = svg::node::element::Definitions::new();
        for filter in self.unique_filters() {
            defs = defs.add(filter.render_to_svg(
                self.colormap.clone(),
                self.cell_size,
                self.object_sizes,
                "",
            )?);
        }

        for pattern_fill in self.unique_pattern_fills() {
            if let Some(patterndef) = pattern_fill.pattern_definition(&self.colormap) {
                defs = defs.add(patterndef)
            }
        }

        Ok(svg
            .add(defs)
            .set(
                "viewBox",
                format!(
                    "{0} {0} {1} {2}",
                    -(self.canvas_outter_padding as i32),
                    self.width(),
                    self.height()
                ),
            )
            .set("width", self.width())
            .set("height", self.height())
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
        info_time!("svg_to_pixmap");

        let mut pixmap = self.create_pixmap(width, height);

        let parsed_svg = &svg_to_usvg_tree(contents, &self.fontdb)?;

        self.usvg_tree_to_pixmap(width, height, pixmap.as_mut(), parsed_svg);

        Ok(pixmap)
    }

    pub fn render_to_pixmap_no_cache(
        &mut self,
        width: u32,
        height: u32,
    ) -> anyhow::Result<tiny_skia::Pixmap> {
        let svg_contents = self
            .render_to_svg(self.colormap.clone(), self.cell_size, self.object_sizes, "")?
            .to_string();
        self.svg_to_pixmap(width, height, &svg_contents)
    }

    // Returns None if we had a render cache hit -- pixmap is in self.png_render_cache in that case
    pub fn render_to_pixmap(
        &mut self,
        width: u32,
        height: u32,
    ) -> anyhow::Result<Option<tiny_skia::Pixmap>> {
        info_time!("render_to_pixmap");

        self.load_fonts()?;

        let new_svg_contents = self
            .render_to_svg(self.colormap.clone(), self.cell_size, self.object_sizes, "")?
            .to_string();
        if let Some(cached_svg) = &self.png_render_cache {
            if *cached_svg == new_svg_contents {
                // TODO find a way to avoid .cloneing the pixmap
                return Ok(None);
            }
        }

        let pixmap = self.svg_to_pixmap(width, height, &new_svg_contents)?;

        self.png_render_cache = Some(new_svg_contents);

        Ok(Some(pixmap))
    }

    pub fn pixmap_to_hwc_frame(
        &self,
        resolution: u32,
        pixmap: &tiny_skia::Pixmap,
    ) -> anyhow::Result<video_rs::Frame> {
        info_time!("pixmap_to_hwc_frame");
        let (width, height) = self.resolution_to_size(resolution);
        let (width, height) = (width as usize, height as usize);
        let mut data = vec![0u8; height * width * 3];

        data.par_chunks_exact_mut(3)
            .enumerate()
            .for_each(|(index, chunk)| {
                let x = index % width;
                let y = index / width;

                let pixel = pixmap
                    .pixel(x as u32, y as u32)
                    .unwrap_or_else(|| panic!("No pixel found at x, y = {x}, {y}"));

                chunk[0] = pixel.red();
                chunk[1] = pixel.green();
                chunk[2] = pixel.blue();
            });

        Ok(video_rs::Frame::from_shape_vec([height, width, 3], data)?)
    }

    pub fn render_to_hwc_frame(&mut self, resolution: u32) -> anyhow::Result<video_rs::Frame> {
        let (width, height) = self.resolution_to_size(resolution);
        let pixmap = self.render_to_pixmap_no_cache(width, height)?;
        self.pixmap_to_hwc_frame(resolution, &pixmap)
    }

    fn usvg_tree_to_pixmap(
        &self,
        width: u32,
        height: u32,
        mut pixmap_mut: tiny_skia::PixmapMut<'_>,
        parsed_svg: &resvg::usvg::Tree,
    ) {
        info_time!("usvg_tree_to_pixmap");
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
        info_time!("create_pixmap");
        tiny_skia::Pixmap::new(width, height).expect("Failed to create pixmap")
    }
}

fn svg_to_usvg_tree(
    svg: &str,
    fontdb: &Option<Arc<usvg::fontdb::Database>>,
) -> anyhow::Result<resvg::usvg::Tree> {
    info_time!("svg_to_usvg_tree");
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
