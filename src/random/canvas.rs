use crate::{Canvas, Fill, Layer, Object, Region, Shape};
use rand::{Rng, distr::uniform::SampleRange};
use std::collections::HashMap;

impl Canvas {
    pub fn random_layer(&mut self, rng: &mut impl Rng, name: &str) -> Layer {
        self.random_layer_within(rng, name, &self.world_region.clone())
    }

    pub fn random_object(&mut self, rng: &mut impl Rng) -> Shape {
        self.random_object_within(rng, &self.world_region.clone())
    }

    pub fn random_object_within(
        &mut self,
        rng: &mut impl Rng,
        region: &Region,
    ) -> Shape {
        Shape::random(
            rng,
            region,
            self.object_sizes.default_line_width,
            self.polygon_vertices_range.clone(),
        )
    }

    pub fn n_random_curves_within(
        &mut self,
        rng: &mut impl Rng,
        region: &Region,
        count: usize,
        layer_name: &str,
    ) -> Layer {
        let mut objects: HashMap<String, Object> = HashMap::new();
        for i in 0..count {
            let object = Shape::random_curve_within(
                rng,
                region,
                self.object_sizes.default_line_width,
            );
            objects.insert(
                format!("{}#{}", layer_name, i),
                Object::from((
                    object,
                    if rng.random_bool(0.5) {
                        Some(Fill::random_solid(rng, self.background))
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
            hidden: false,
        }
    }

    pub fn random_curves_within(
        &mut self,
        rng: &mut impl Rng,
        layer_name: &str,
        region: &Region,
        object_counts: impl SampleRange<usize>,
    ) -> Layer {
        let number_of_objects = rng.random_range(object_counts);
        self.n_random_curves_within(rng, region, number_of_objects, layer_name)
    }

    pub fn random_layer_within(
        &mut self,
        rng: &mut impl Rng,
        name: &str,
        region: &Region,
    ) -> Layer {
        let mut objects: HashMap<String, Object> = HashMap::new();
        let number_of_objects =
            rng.random_range(self.objects_count_range.clone());
        for i in 0..number_of_objects {
            let object = Shape::random(
                rng,
                region,
                self.object_sizes.default_line_width,
                self.polygon_vertices_range.clone(),
            );
            let hatchable = object.hatchable();
            objects.insert(
                format!("{}#{}", name, i),
                object.filled(if hatchable {
                    Fill::random_hatches(rng, self.background)
                } else {
                    Fill::random_solid(rng, self.background)
                }),
            );
        }

        Layer {
            object_sizes: self.object_sizes,
            name: name.to_string(),
            objects,
            hidden: false,
        }
    }

    pub fn random_linelikes(
        &mut self,
        rng: &mut impl Rng,
        layer_name: &str,
    ) -> Layer {
        self.random_curves_within(
            rng,
            layer_name,
            &self.world_region.clone(),
            self.objects_count_range.clone(),
        )
    }

    pub fn random_region(&mut self, rng: &mut impl Rng) -> Region {
        Region::random(rng, &self.world_region.clone())
    }
}
