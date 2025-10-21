use nanoid::nanoid;

use crate::{ColoredObject, Fill, Filter, ObjectSizes, Region, Toggleable};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Default)]
// #[wasm_bindgen(getter_with_clone)]
pub struct Layer {
    pub object_sizes: ObjectSizes,
    pub objects: HashMap<String, ColoredObject>,
    pub name: String,
    pub hidden: bool,
    pub _render_cache: Option<svg::node::element::Group>,
}

impl Layer {
    pub fn new(name: impl Display) -> Self {
        Layer {
            object_sizes: ObjectSizes::default(),
            objects: HashMap::new(),
            name: format!("{}", name),
            _render_cache: None,
            hidden: false,
        }
    }

    pub fn hide(&mut self) {
        self.hidden = true;
    }

    pub fn show(&mut self) {
        self.hidden = false;
    }

    pub fn toggle(&mut self) {
        self.hidden.toggle();
    }

    pub fn object(&mut self, name: &str) -> &mut ColoredObject {
        self.safe_object(name).unwrap()
    }

    pub fn safe_object(&mut self, name: &str) -> Option<&mut ColoredObject> {
        self.objects.get_mut(name)
    }

    // Flush the render cache.
    pub fn flush(&mut self) {
        self._render_cache = None;
    }

    // Remove all objects.
    pub fn clear(&mut self) {
        self.objects.clear();
        self.flush();
    }

    pub fn replace(&mut self, with: Layer) {
        self.objects.clone_from(&with.objects);
        self.flush();
    }

    pub fn remove_all_objects_in(&mut self, region: &Region) {
        self.objects.retain(|_, ColoredObject { object, .. }| {
            !object.region().within(region)
        })
    }

    pub fn paint_all_objects(&mut self, fill: Fill) {
        for obj in self.objects.values_mut() {
            obj.fill = Some(fill);
        }
        self.flush();
    }

    pub fn filter_all_objects(&mut self, filter: Filter) {
        for obj in self.objects.values_mut() {
            obj.filters.push(filter)
        }
        self.flush();
    }

    pub fn move_all_objects(&mut self, dx: i32, dy: i32) {
        self.objects
            .iter_mut()
            .for_each(|(_, ColoredObject { object, .. })| {
                object.translate(dx, dy)
            });
        self.flush();
    }

    pub fn add(&mut self, name: impl Display, object: impl Into<ColoredObject>) {
        let name_str = format!("{}", name);

        if self.objects.contains_key(&name_str) {
            panic!("object {} already exists in layer {}", name_str, self.name);
        }

        self.set(name_str, object);
    }

    pub fn add_anon(&mut self, object: impl Into<ColoredObject>) {
        self.add(nanoid!(), object);
    }

    pub fn set(&mut self, name: impl Display, object: impl Into<ColoredObject>) {
        let name_str = format!("{}", name);

        self.objects.insert(name_str, object.into());
        self.flush();
    }

    pub fn filter_object(
        &mut self,
        name: &str,
        filter: Filter,
    ) -> Result<(), String> {
        self.objects
            .get_mut(name)
            .ok_or(format!("Object '{}' not found", name))?
            .filters
            .push(filter);

        self.flush();
        Ok(())
    }

    pub fn remove_object(&mut self, name: &str) {
        self.objects.remove(name);
        self.flush();
    }

    pub fn replace_object(&mut self, name: &str, object: ColoredObject) {
        self.remove_object(name);
        self.add(name, object);
    }

    pub fn add_objects(
        &mut self,
        objects: impl IntoIterator<Item = ColoredObject>,
    ) {
        for obj in objects {
            self.add_anon(obj);
        }
    }

    /// Returns the effective region the layer occupies, by merging all its objects' regions.
    pub fn region(&self) -> Region {
        self.objects
            .values()
            .map(|object| object.region())
            .fold(Region::default(), |acc, region| acc.merge(&region))
    }
}

impl ColoredObject {
    pub fn add_to(self, layer: &mut Layer) {
        layer.add_anon(self);
    }

    pub fn add_named_to(self, name: impl Display, layer: &mut Layer) {
        layer.add(name, self);
    }
}
