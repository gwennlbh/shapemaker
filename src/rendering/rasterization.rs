use anyhow::Result;
use measure_time::debug_time;
use resvg::usvg;
use std::sync::Arc;

pub fn create_pixmap(width: u32, height: u32) -> tiny_skia::Pixmap {
    debug_time!("create_pixmap");
    tiny_skia::Pixmap::new(width, height).expect("Failed to create pixmap")
}

pub fn paint_svg_on_pixmap(
    pixmap: tiny_skia::PixmapMut<'_>,
    svg_contents: &str,
    canvas_dimensions: (usize, usize),
    fontdb: &Option<Arc<usvg::fontdb::Database>>,
) -> Result<()> {
    debug_time!("paint_svg_on_pixmap");
    let parsed_svg = &svg_to_usvg_tree(svg_contents, fontdb)?;

    usvg_tree_to_pixmap(canvas_dimensions, pixmap, parsed_svg);

    Ok(())
}

pub fn usvg_tree_to_pixmap(
    canvas_dimensions: (usize, usize),
    mut pixmap_mut: tiny_skia::PixmapMut<'_>,
    parsed_svg: &resvg::usvg::Tree,
) {
    debug_time!("usvg_tree_to_pixmap");

    let (canvas_width, canvas_height) = canvas_dimensions;
    let (target_width, target_height) = (pixmap_mut.width(), pixmap_mut.height());

    resvg::render(
        parsed_svg,
        tiny_skia::Transform::from_scale(
            target_width as f32 / canvas_width as f32,
            target_height as f32 / canvas_height as f32,
        ),
        &mut pixmap_mut,
    );
}

pub fn svg_to_usvg_tree(
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

pub fn pixmap_to_png_data(pixmap: tiny_skia::Pixmap) -> anyhow::Result<Vec<u8>> {
    debug_time!("pixmap_to_png_data");
    Ok(pixmap.encode_png()?)
}

pub fn write_png_data(data: Vec<u8>, at: &str) -> anyhow::Result<()> {
    debug_time!("write_png_data");
    std::fs::write(at, data)?;
    Ok(())
}
