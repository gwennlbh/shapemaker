use std::hash::Hash;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterType {
    Glow,
    NaturalShadow,
    Saturation,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Filter {
    pub kind: FilterType,
    pub parameter: f32,
}

#[wasm_bindgen]
impl Filter {
    pub fn name(&self) -> String {
        match self.kind {
            FilterType::Glow => "glow",
            FilterType::NaturalShadow => "natural-shadow-filter",
            FilterType::Saturation => "saturation",
        }
        .to_owned()
    }

    pub fn glow(intensity: f32) -> Self {
        Self {
            kind: FilterType::Glow,
            parameter: intensity,
        }
    }

    pub fn id(&self) -> String {
        format!(
            "filter-{}-{}",
            self.name(),
            self.parameter.to_string().replace('.', "_")
        )
    }
}

impl PartialEq for Filter {
    fn eq(&self, other: &Self) -> bool {
        // TODO use way less restrictive epsilon
        self.kind == other.kind && (self.parameter - other.parameter).abs() < f32::EPSILON
    }
}

impl Hash for Filter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl Eq for Filter {}
