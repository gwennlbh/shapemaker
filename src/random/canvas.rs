use crate::{Canvas, ColoredObject, Fill, Layer, Object, Region};
use rand::{distributions::uniform::SampleRange, Rng};
use std::collections::HashMap;

impl Canvas {
    pub fn random_layer(&self, name: &str) -> Layer {
        self.random_layer_within(name, &self.world_region)
    }

    pub fn random_object(&self) -> Object {
        Object::random(
            &self.world_region,
            self.object_sizes.default_line_width,
            self.polygon_vertices_range.clone(),
        )
    }

    pub fn n_random_curves_within(
        &self,
        region: &Region,
        count: usize,
        layer_name: &str,
    ) -> Layer {
        let mut objects: HashMap<String, ColoredObject> = HashMap::new();
        for i in 0..count {
            let object =
                Object::random_curve_within(region, self.object_sizes.default_line_width);
            objects.insert(
                format!("{}#{}", layer_name, i),
                ColoredObject::from((
                    object,
                    if rand::thread_rng().gen_bool(0.5) {
                        Some(Fill::random_solid(self.background))
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

    pub fn random_curves_within(
        &self,
        layer_name: &str,
        region: &Region,
        object_counts: impl SampleRange<usize>,
    ) -> Layer {
        let number_of_objects = rand::thread_rng().gen_range(object_counts);
        self.n_random_curves_within(layer_name, region, number_of_objects)
    }

    pub fn random_layer_within(&self, name: &str, region: &Region) -> Layer {
        let mut objects: HashMap<String, ColoredObject> = HashMap::new();
        let number_of_objects = rand::thread_rng().gen_range(self.objects_count_range.clone());
        for i in 0..number_of_objects {
            let object = Object::random(
                region,
                self.object_sizes.default_line_width,
                self.polygon_vertices_range.clone(),
            );
            let hatchable = object.hatchable();
            objects.insert(
                format!("{}#{}", name, i),
                object.paint(if hatchable {
                    Fill::random_hatches(self.background)
                } else {
                    Fill::random_solid(self.background)
                }),
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
        self.random_curves_within(
            layer_name,
            &self.world_region,
            self.objects_count_range.clone(),
        )
    }

    pub fn random_region(&self) -> Region {
        Region::random(&self.world_region)
    }
}
