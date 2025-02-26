use wasm_bindgen::prelude::*;

use slug::slugify;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformationType {
    Scale,
    Rotate,
    Skew,
    Matrix,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Transformation {
    Scale(f32, f32),
    Rotate(f32),
    Skew(f32, f32),
    Matrix(f32, f32, f32, f32, f32, f32),
}

impl Transformation {
    pub fn name(&self) -> String {
        match self {
            Transformation::Matrix(..) => "matrix",
            Transformation::Rotate(..) => "rotate",
            Transformation::Scale(..) => "scale",
            Transformation::Skew(..) => "skew",
        }
        .to_owned()
    }

    #[allow(non_snake_case)]
    pub fn ScaleUniform(scale: f32) -> Self {
        Transformation::Scale(scale, scale)
    }

    pub fn id(&self) -> String {
        slugify(format!("{:?}", self))
    }
}
