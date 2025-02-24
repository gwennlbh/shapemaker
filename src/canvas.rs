use core::panic;
use rayon::prelude::*;
use std::{collections::HashMap, fs::OpenOptions, io::Write, ops::Range};

use itertools::Itertools as _;
use measure_time::info_time;
use rand::Rng;

use crate::{
    layer::Layer, objects::Object, random_color, Angle, Color, ColorMapping, ColoredObject,
    Containable, Fill, Filter, LineSegment, ObjectSizes, Point, Region,
};

#[derive(Debug, Clone)]
pub struct Canvas {
    pub grid_size: (usize, usize),
    pub cell_size: usize,
    pub objects_count_range: Range<usize>,
    pub polygon_vertices_range: Range<usize>,
    pub canvas_outter_padding: usize,
    pub object_sizes: ObjectSizes,
    pub colormap: ColorMapping,
    /// The layers are in order of top to bottom: the first layer will be rendered on top of the second, etc.
    pub layers: Vec<Layer>,
    pub background: Option<Color>,

    pub world_region: Region,

    /// Render cache for the SVG string. Prevents having to re-calculate a pixmap when the SVG hasn't changed.
    png_render_cache: Option<String>,
}

impl Canvas {
    /// Create a new canvas.
    /// The layers are in order of top to bottom: the first layer will be rendered on top of the second, etc.
    /// A layer named "root" will be added below all layers if you don't add it yourself.
    pub fn new(layer_names: Vec<&str>) -> Self {
        let mut layer_names = layer_names;
        if !layer_names.iter().any(|&name| name == "root") {
            layer_names.push("root");
        }
        Self {
            layers: layer_names
                .iter()
                .map(|name| Layer {
                    object_sizes: ObjectSizes::default(),
                    objects: HashMap::new(),
                    name: name.to_string(),
                    _render_cache: None,
                    hidden: false,
                })
                .collect(),
            ..Self::default_settings()
        }
    }

    pub fn set_grid_size(&mut self, new_width: usize, new_height: usize) {
        self.grid_size = (new_width, new_height);
        self.world_region = Region {
            start: Point(0, 0),
            end: Point::from(self.grid_size).translated(-1, -1),
        };
    }

    pub fn layer_safe(&mut self, name: &str) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|layer| layer.name == name)
    }

    pub fn layer(&mut self, name: &str) -> &mut Layer {
        if !self.layer_exists(name) {
            panic!("Layer {} does not exist", name);
        }

        self.layer_safe(name).unwrap()
    }

    pub fn new_layer(&mut self, name: &str) -> &mut Layer {
        if self.layer_exists(name) {
            panic!("Layer {} already exists", name);
        }

        self.layers.push(Layer::new(name));
        self.layers.last_mut().unwrap()
    }

    pub fn layer_or_empty(&mut self, name: &str) -> &mut Layer {
        if self.layer_exists(name) {
            return self.layer(name);
        }

        self.new_layer(name)
    }

    pub fn layer_exists(&self, name: &str) -> bool {
        self.layers.iter().any(|layer| layer.name == name)
    }

    pub fn ensure_layer_exists(&self, name: &str) {
        if !self.layer_exists(name) {
            panic!("Layer {} does not exist", name);
        }
    }

    /// puts this layer on top, and the others below, without changing their order
    pub fn put_layer_on_top(&mut self, name: &str) {
        self.ensure_layer_exists(name);
        let target_index = self.layers.iter().position(|l| l.name == name).unwrap();
        self.layers.swap(0, target_index)
    }

    /// puts this layer on bottom, and the others above, without changing their order
    pub fn put_layer_on_bottom(&mut self, name: &str) {
        self.ensure_layer_exists(name);
        let target_index = self.layers.iter().position(|l| l.name == name).unwrap();
        let last_index = self.layers.len() - 1;
        self.layers.swap(last_index, target_index)
    }

    /// re-order layers. The first layer in the list will be on top, the last at the bottom
    pub fn reorder_layers(&mut self, new_order: Vec<&str>) {
        println!(
            "re-ordering {:?} to {:?}",
            self.layers
                .iter()
                .map(|l| l.name.clone())
                .collect::<Vec<_>>(),
            new_order
        );
        let current_order = self
            .layers
            .iter()
            .map(|l| l.name.clone())
            .collect::<Vec<_>>();

        // make sure the new order is well-formed
        // assert_eq!(self.layers.len(), new_order.len());
        assert!(new_order.iter().all(|name| self.layer_exists(name)));

        self.layers.sort_by_key(|o| {
            new_order
                .iter()
                .position(|&n| n == o.name)
                .unwrap_or(current_order.iter().position(|n| *n == o.name).unwrap())
        });
    }

    pub fn root(&mut self) -> &mut Layer {
        self.layer_safe("root")
            .expect("Layer 'root' should always exist in a canvas")
    }

    pub fn add_object(
        &mut self,
        layer: &str,
        name: &str,
        object: Object,
        fill: Option<Fill>,
    ) -> Result<(), String> {
        match self.layer_safe(layer) {
            None => Err(format!("Layer {} does not exist", layer)),
            Some(layer) => {
                layer.add_object(name, (object, fill).into());
                Ok(())
            }
        }
    }

    pub fn remove_object(&mut self, name: &str) {
        for layer in self.layers.iter_mut() {
            layer.remove_object(name);
        }
    }

    pub fn set_background(&mut self, color: Color) {
        self.background = Some(color);
    }

    pub fn remove_background(&mut self) {
        self.background = None;
    }

    pub fn default_settings() -> Self {
        Self {
            grid_size: (3, 3),
            cell_size: 50,
            objects_count_range: 3..7,
            polygon_vertices_range: 2..7,
            canvas_outter_padding: 10,
            object_sizes: ObjectSizes::default(),
            colormap: ColorMapping::default(),
            layers: vec![],
            world_region: Region::new(0, 0, 3, 3).unwrap(),
            background: None,
            png_render_cache: None,
        }
    }

    pub fn random_layer(&self, name: &str) -> Layer {
        self.random_layer_within(name, &self.world_region)
    }

    pub fn random_object(&self) -> Object {
        self.random_object_within(&self.world_region)
    }

    pub fn add_or_replace_layer(&mut self, layer: Layer) {
        if let Some(existing_layer) = self.layer_safe(&layer.name) {
            existing_layer.replace(layer);
        } else {
            self.layers.push(layer);
        }
    }

    pub fn random_layer_within(&self, name: &str, region: &Region) -> Layer {
        let mut objects: HashMap<String, ColoredObject> = HashMap::new();
        let number_of_objects = rand::thread_rng().gen_range(self.objects_count_range.clone());
        for i in 0..number_of_objects {
            let object = self.random_object_within(region);
            let hatchable = object.hatchable();
            objects.insert(
                format!("{}#{}", name, i),
                object.color(self.random_fill(hatchable)),
            );
        }
        Layer {
            object_sizes: self.object_sizes,
            name: name.to_string(),
            objects,
            _render_cache: None,
            hidden: false,
        }
    }

    pub fn random_linelikes(&self, layer_name: &str) -> Layer {
        self.random_linelikes_within(layer_name, &self.world_region)
    }

    pub fn n_random_linelikes_within(
        &self,
        layer_name: &str,
        region: &Region,
        count: usize,
    ) -> Layer {
        let mut objects: HashMap<String, ColoredObject> = HashMap::new();
        for i in 0..count {
            let object = self.random_linelike_within(region);
            let hatchable = object.fillable();
            objects.insert(
                format!("{}#{}", layer_name, i),
                ColoredObject::from((
                    object,
                    if rand::thread_rng().gen_bool(0.5) {
                        Some(self.random_fill(hatchable))
                    } else {
                        None
                    },
                )),
            );
        }
        Layer {
            object_sizes: self.object_sizes,
            name: layer_name.to_owned(),
            objects,
            _render_cache: None,
            hidden: false,
        }
    }

    pub fn random_linelikes_within(&self, layer_name: &str, region: &Region) -> Layer {
        let number_of_objects = rand::thread_rng().gen_range(self.objects_count_range.clone());
        self.n_random_linelikes_within(layer_name, region, number_of_objects)
    }

    pub fn random_object_within(&self, region: &Region) -> Object {
        let start = self.random_point(region);
        match rand::thread_rng().gen_range(1..=7) {
            1 => self.random_polygon(region),
            2 => Object::BigCircle(start),
            3 => Object::SmallCircle(start),
            4 => Object::Dot(start),
            5 => Object::CurveInward(
                start,
                self.random_end_anchor(start, region),
                self.object_sizes.default_line_width,
            ),
            6 => Object::CurveOutward(
                start,
                self.random_end_anchor(start, region),
                self.object_sizes.default_line_width,
            ),
            7 => Object::Line(
                self.random_point(region),
                self.random_point(region),
                self.object_sizes.default_line_width,
            ),
            _ => unreachable!(),
        }
    }

    pub fn random_linelike_within(&self, region: &Region) -> Object {
        let start = self.random_point(region);
        match rand::thread_rng().gen_range(1..=3) {
            1 => Object::CurveInward(
                start,
                self.random_end_anchor(start, region),
                self.object_sizes.default_line_width,
            ),
            2 => Object::CurveOutward(
                start,
                self.random_end_anchor(start, region),
                self.object_sizes.default_line_width,
            ),
            3 => Object::Line(
                self.random_point(region),
                self.random_point(region),
                self.object_sizes.default_line_width,
            ),
            _ => unreachable!(),
        }
    }

    pub fn random_end_anchor(&self, start: Point, region: &Region) -> Point {
        // End anchors are always a square diagonal from the start anchor (for now)
        // that means taking steps of the form n * (one of (1, 1), (1, -1), (-1, 1), (-1, -1))
        // Except that the end anchor needs to stay in the bounds of the shape.

        // Determine all possible end anchors that are in a square diagonal from the start anchor
        let mut possible_end_anchors = vec![];

        // shapes can end on the next cell, since that's where they end
        let actual_region = region.enlarged(1, 1);

        for x in actual_region.mirrored_width_range() {
            for y in actual_region.mirrored_height_range() {
                let end_anchor = start.translated(x, y);

                if end_anchor == start {
                    continue;
                }

                // Check that the end anchor is in a square diagonal from the start anchor and that the end anchor is in bounds
                if x.abs() == y.abs() && actual_region.contains(&end_anchor) {
                    possible_end_anchors.push(end_anchor);
                }
            }
        }

        // Pick a random end anchor from the possible end anchors
        possible_end_anchors[rand::thread_rng().gen_range(0..possible_end_anchors.len())]
    }

    pub fn random_polygon(&self, region: &Region) -> Object {
        let number_of_anchors = rand::thread_rng().gen_range(self.polygon_vertices_range.clone());
        let start = self.random_point(region);
        let mut lines: Vec<LineSegment> = vec![];
        for _ in 0..number_of_anchors {
            let next_anchor = self.random_point(region);
            lines.push(self.random_line(next_anchor));
        }
        Object::Polygon(start, lines)
    }

    pub fn random_line(&self, end: Point) -> LineSegment {
        match rand::thread_rng().gen_range(1..=3) {
            1 => LineSegment::Straight(end),
            2 => LineSegment::InwardCurve(end),
            3 => LineSegment::OutwardCurve(end),
            _ => unreachable!(),
        }
    }

    pub fn region_is_whole_grid(&self, region: &Region) -> bool {
        region.start == (0, 0) && region.end == self.grid_size
    }

    pub fn random_region(&self) -> Region {
        let start = self.random_point(&self.world_region);
        let end = self.random_end_anchor(start, &self.world_region);
        Region::from(if start.0 > end.0 {
            (end, start)
        } else {
            (start, end)
        })
    }

    pub fn random_point(&self, region: &Region) -> Point {
        region.ensure_nonempty().unwrap();
        Point(
            rand::thread_rng().gen_range(region.x_range()),
            rand::thread_rng().gen_range(region.y_range()),
        )
    }

    pub fn random_fill(&self, hatchable: bool) -> Fill {
        if hatchable {
            if rand::thread_rng().gen_bool(0.75) {
                Fill::Solid(random_color(self.background))
            } else {
                let hatch_size = rand::thread_rng().gen_range(5..=100) as f32 * 1e-2;
                Fill::Hatched(
                    random_color(self.background),
                    Angle(rand::thread_rng().gen_range(0.0..360.0)),
                    hatch_size,
                    // under a certain hatch size, we can't see the hatching if the ratio is not ½
                    if hatch_size < 8.0 {
                        0.5
                    } else {
                        rand::thread_rng().gen_range(1..=4) as f32 / 4.0
                    },
                )
            }
        } else {
            Fill::Solid(random_color(self.background))
        }
    }

    pub fn clear(&mut self) {
        self.layers.clear();
        self.remove_background()
    }

    pub fn resolution_to_size(&self, resolution: u32) -> (u32, u32) {
        let aspect_ratio = self.aspect_ratio();
        if aspect_ratio > 1.0 {
            // landscape: resolution is width
            (resolution, (resolution as f32 / aspect_ratio) as u32)
        } else {
            // portrait: resolution is height
            ((resolution as f32 * aspect_ratio) as u32, resolution)
        }
    }

    // previous_frame_at gives path to the previously rendered frame, which allows to copy on cache hits instead of having to re-write bytes again
    pub fn render_to_png(
        &mut self,
        at: &str,
        resolution: u32,
        previous_frame_at: Option<&str>,
    ) -> anyhow::Result<()> {
        info_time!("render_to_png");
        let (width, height) = self.resolution_to_size(resolution);
        if let Some(previous_frame_at) = previous_frame_at {
            match self.render_to_pixmap(width, height)? {
                None => {
                    std::fs::copy(previous_frame_at, at)?;
                }
                Some(pixmap) => {
                    pixmap_to_png_data(pixmap).and_then(|data| write_png_data(data, at))?
                }
            }
            return Ok(());
        }

        Ok(self
            .render_to_pixmap_no_cache(width, height)
            .and_then(|pixmap| {
                pixmap_to_png_data(pixmap).and_then(|data| write_png_data(data, at))
            })?)
    }
}

fn pixmap_to_png_data(pixmap: tiny_skia::Pixmap) -> anyhow::Result<Vec<u8>> {
    info_time!("\tpixmap_to_png_data");
    Ok(pixmap.encode_png()?)
}

fn write_png_data(data: Vec<u8>, at: &str) -> anyhow::Result<()> {
    info_time!("\twrite_png_data");
    std::fs::write(at, data)?;
    Ok(())
}

impl Canvas {
    pub fn width(&self) -> usize {
        self.cell_size * self.world_region.width() + 2 * self.canvas_outter_padding
    }

    pub fn height(&self) -> usize {
        self.cell_size * self.world_region.height() + 2 * self.canvas_outter_padding
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width() as f32 / self.height() as f32
    }

    pub fn remove_all_objects_in(&mut self, region: &Region) {
        self.layers
            .iter_mut()
            .for_each(|layer| layer.remove_all_objects_in(region));
    }

    /// returns a list of all unique filters used throughout the canvas
    /// used to only generate one definition per filter
    ///
    fn unique_filters(&self) -> Vec<Filter> {
        self.layers
            .iter()
            .flat_map(|layer| layer.objects.iter().flat_map(|(_, o)| o.filters.clone()))
            .unique()
            .collect()
    }

    fn unique_pattern_fills(&self) -> Vec<Fill> {
        self.layers
            .iter()
            .flat_map(|layer| layer.objects.iter().flat_map(|(_, o)| o.fill))
            .filter(|fill| matches!(fill, Fill::Hatched(..) | Fill::Dotted(..)))
            .unique_by(|fill| fill.pattern_id())
            .collect()
    }

    pub fn debug_region(&mut self, region: &Region, color: Color) {
        let layer = self.layer_or_empty("debug plane");

        layer.add_object(
            format!("{}_corner_ss", region).as_str(),
            Object::Dot(region.topleft()).color(Fill::Solid(color)),
        );
        layer.add_object(
            format!("{}_corner_se", region).as_str(),
            Object::Dot(region.topright().translated(1, 0)).color(Fill::Solid(color)),
        );
        layer.add_object(
            format!("{}_corner_ne", region).as_str(),
            Object::Dot(region.bottomright().translated(1, 1)).color(Fill::Solid(color)),
        );
        layer.add_object(
            format!("{}_corner_nw", region).as_str(),
            Object::Dot(region.bottomleft().translated(0, 1)).color(Fill::Solid(color)),
        );
        layer.add_object(
            format!("{}_region", region).as_str(),
            Object::Rectangle(region.start, region.end).color(Fill::Translucent(color, 0.25)),
        )
    }

    pub fn render_to_svg(&mut self) -> anyhow::Result<String> {
        info_time!("render_to_svg");
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

        for layer in self.layers.iter_mut().filter(|layer| !layer.hidden).rev() {
            svg = svg.add(layer.render(self.colormap.clone(), self.cell_size, layer.object_sizes));
        }

        let mut defs = svg::node::element::Definitions::new();
        for filter in self.unique_filters() {
            defs = defs.add(filter.definition())
        }

        for pattern_fill in self.unique_pattern_fills() {
            if let Some(patterndef) = pattern_fill.pattern_definition(&self.colormap) {
                defs = defs.add(patterndef)
            }
        }

        let rendered = svg
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
            .to_string();

        Ok(rendered)
    }

    pub fn render_to_pixmap_no_cache(
        &mut self,
        width: u32,
        height: u32,
    ) -> anyhow::Result<tiny_skia::Pixmap> {
        info_time!("render_to_pixmap_no_cache");
        let mut pixmap = self.create_pixmap(width, height);

        let parsed_svg = &svg_to_usvg_tree(&self.render_to_svg()?)?;

        self.usvg_tree_to_pixmap(width, height, pixmap.as_mut(), parsed_svg);

        Ok(pixmap)
    }

    // Returns None if we had a render cache hit -- pixmap is in self.png_render_cache in that case
    pub fn render_to_pixmap(
        &mut self,
        width: u32,
        height: u32,
    ) -> anyhow::Result<Option<tiny_skia::Pixmap>> {
        info_time!("render_to_pixmap");

        let new_svg_contents = self.render_to_svg()?;
        if let Some(cached_svg) = &self.png_render_cache {
            if *cached_svg == new_svg_contents {
                // TODO find a way to avoid .cloneing the pixmap
                return Ok(None);
            }
        }

        let mut pixmap = self.create_pixmap(width, height);

        let parsed_svg = &svg_to_usvg_tree(&new_svg_contents)?;

        self.usvg_tree_to_pixmap(width, height, pixmap.as_mut(), parsed_svg);

        self.png_render_cache = Some(new_svg_contents);

        Ok(Some(pixmap))
    }

    pub fn render_to_hwc_frame(&mut self, resolution: u32) -> anyhow::Result<video_rs::Frame> {
        info_time!("render_to_hwc_frame");
        let (width, height) = self.resolution_to_size(resolution);
        let pixmap = self.render_to_pixmap_no_cache(width, height)?;

        let (width, height) = (width as usize, height as usize);
        let mut data = vec![0u8; height * width * 3];

        data.par_chunks_exact_mut(3)
            .enumerate()
            .for_each(|(index, chunk)| {
                let x = index % width;
                let y = index / width;

                let pixel = pixmap
                    .pixel(x as u32, y as u32)
                    .expect(&format!("No pixel found at x, y = {x}, {y}"));

                chunk[0] = pixel.red();
                chunk[1] = pixel.green();
                chunk[2] = pixel.blue();
            });

        Ok(video_rs::Frame::from_shape_vec([height, width, 3], data)?)
    }

    fn usvg_tree_to_pixmap(
        &mut self,
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

fn svg_to_usvg_tree(svg: &str) -> anyhow::Result<resvg::usvg::Tree> {
    info_time!("svg_to_usvg_tree");
    Ok(resvg::usvg::Tree::from_str(
        svg,
        &resvg::usvg::Options::default(),
    )?)
}
