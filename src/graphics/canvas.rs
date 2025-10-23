use core::panic;
use resvg::usvg;
use std::{collections::HashMap, ops::Range, sync::Arc};

use itertools::Itertools as _;
use measure_time::debug_time;

use crate::{
    fonts::{load_fonts, FontOptions},
    Color, ColorMapping, Fill, Filter, Layer, Object, ObjectSizes, Point, Region,
};

use super::ColoredObject;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub grid_size: (usize, usize),
    pub cell_size: usize,
    pub objects_count_range: Range<usize>,
    pub polygon_vertices_range: Range<usize>,
    pub canvas_outter_padding: usize,
    pub object_sizes: ObjectSizes,
    pub font_options: FontOptions,
    pub colormap: ColorMapping,

    /// The layers are in order of top to bottom: the first layer will be rendered on top of the second, etc.
    pub layers: Vec<Layer>,
    pub background: Option<Color>,

    pub world_region: Region,

    /// Render cache for the SVG string. Prevents having to re-calculate a pixmap when the SVG hasn't changed.
    pub(crate) fontdb: Option<Arc<usvg::fontdb::Database>>,
}

impl Canvas {
    pub fn default_settings() -> Self {
        Self {
            grid_size: (3, 3),
            cell_size: 50,
            objects_count_range: 3..7,
            polygon_vertices_range: 2..7,
            canvas_outter_padding: 10,
            object_sizes: ObjectSizes::default(),
            font_options: FontOptions::default(),
            colormap: ColorMapping::default(),
            layers: vec![Layer::new("root")],
            world_region: Region::new(0, 0, 3, 3).unwrap(),
            background: None,
            fontdb: None,
        }
    }

    /// Create a new canvas.
    /// The layers are in order of top to bottom: the first layer will be rendered on top of the second, etc.
    /// A layer named "root" will be added below all layers if you don't add it yourself.
    pub fn new(layer_names: Vec<&str>) -> Self {
        let mut canvas = Self::default_settings();
        canvas.load_fonts().unwrap();
        canvas.init_layers(layer_names);
        canvas
    }

    pub fn with_colors(colormap: ColorMapping) -> Self {
        Self {
            colormap,
            ..Self::default_settings()
        }
    }

    pub fn init_layers(&mut self, names: Vec<&str>) {
        self.layers = names
            .iter()
            .map(|name| Layer {
                object_sizes: ObjectSizes::default(),
                objects: HashMap::new(),
                name: name.to_string(),
                _render_cache: None,
                hidden: false,
            })
            .collect();
        if !self.layer_exists("root") {
            self.layers.push(Layer::new("root"));
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

        let mut layer = Layer::new(name);
        layer.object_sizes = self.object_sizes;
        self.layers.push(layer);
        self.layers.last_mut().unwrap()
    }

    pub fn add_layer(&mut self, layer: Layer) -> &mut Layer {
        if self.layer_exists(&layer.name) {
            panic!("Layer {} already exists", layer.name);
        }

        self.layers.push(layer);
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
        let target_index =
            self.layers.iter().position(|l| l.name == name).unwrap();
        self.layers.swap(0, target_index)
    }

    /// puts this layer on bottom, and the others above, without changing their order
    pub fn put_layer_on_bottom(&mut self, name: &str) {
        self.ensure_layer_exists(name);
        let target_index =
            self.layers.iter().position(|l| l.name == name).unwrap();
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
            new_order.iter().position(|n| *n == o.name).unwrap_or(
                current_order.iter().position(|n| *n == o.name).unwrap(),
            )
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
                layer.set(name, ColoredObject::from((object, fill)));
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

    pub fn fonts_loaded(&self) -> bool {
        self.fontdb.is_some()
    }

    pub fn load_fonts(&mut self) -> anyhow::Result<()> {
        if self.fonts_loaded() {
            return Ok(());
        }

        debug_time!("load_fonts");
        let usvg = load_fonts(&self.font_options)?;
        self.fontdb = Some(usvg.fontdb);
        Ok(())
    }

    pub fn add_or_replace_layer(&mut self, layer: Layer) {
        if let Some(existing_layer) = self.layer_safe(&layer.name) {
            existing_layer.replace(layer);
        } else {
            self.layers.push(layer);
        }
    }

    pub fn region_is_whole_grid(&self, region: &Region) -> bool {
        region.start == (0, 0) && region.end == self.grid_size
    }
    pub fn clear(&mut self) {
        self.layers.clear();
        self.remove_background()
    }

    pub fn resolution_to_size_even(&self, resolution: u32) -> (u32, u32) {
        let (width, height) = self.resolution_to_size(resolution);
        (width + width % 2, height + height % 2)
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

    pub fn width(&self) -> usize {
        self.cell_size * self.world_region.width()
            + 2 * self.canvas_outter_padding
    }

    pub fn height(&self) -> usize {
        self.cell_size * self.world_region.height()
            + 2 * self.canvas_outter_padding
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
    pub fn unique_filters(&self) -> Vec<Filter> {
        let mut filters: Vec<Filter> = self
            .layers
            .iter()
            .flat_map(|layer| {
                layer.objects.iter().flat_map(|(_, o)| o.filters.clone())
            })
            .unique()
            .collect();
        filters.sort_by_key(|f| format!("{:?}", f));
        filters
    }

    pub fn unique_pattern_fills(&self) -> Vec<Fill> {
        let mut fills: Vec<Fill> = self
            .layers
            .iter()
            .flat_map(|layer| layer.objects.iter().flat_map(|(_, o)| o.fill))
            .filter(|fill| matches!(fill, Fill::Hatches(..) | Fill::Dotted(..)))
            .unique_by(|fill| fill.pattern_id())
            .collect();
        fills.sort_by_key(|f| f.pattern_id());
        fills
    }

    pub fn debug_region(&mut self, region: &Region, color: Color) {
        let layer = self.layer_or_empty("debug plane");

        layer.set(
            format!("{}_corner_ss", region).as_str(),
            Object::Dot(region.topleft()).colored(color),
        );
        layer.set(
            format!("{}_corner_se", region).as_str(),
            Object::Dot(region.topright().translated(1, 0)).colored(color),
        );
        layer.set(
            format!("{}_corner_ne", region).as_str(),
            Object::Dot(region.bottomright().translated(1, 1)).colored(color),
        );
        layer.set(
            format!("{}_corner_nw", region).as_str(),
            Object::Dot(region.bottomleft().translated(0, 1)).colored(color),
        );
        layer.set(
            format!("{}_region", region).as_str(),
            Object::Rectangle(region.start, region.end)
                .filled(Fill::Translucent(color, 0.25)),
        )
    }
}
