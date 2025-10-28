use crate::{Transformation, graphics::TransformationType};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct TransformationWASM {
    pub kind: TransformationType,
    pub parameters: Vec<f32>,
}

impl From<TransformationWASM> for Transformation {
    fn from(transformation: TransformationWASM) -> Self {
        match transformation.kind {
            TransformationType::Scale => Transformation::Scale(
                transformation.parameters[0],
                transformation.parameters[1],
            ),
            TransformationType::Rotate => {
                Transformation::Rotate(transformation.parameters[0])
            }
            TransformationType::Skew => Transformation::Skew(
                transformation.parameters[0],
                transformation.parameters[1],
            ),
            TransformationType::Matrix => Transformation::Matrix(
                transformation.parameters[0],
                transformation.parameters[1],
                transformation.parameters[2],
                transformation.parameters[3],
                transformation.parameters[4],
                transformation.parameters[5],
            ),
        }
    }
}
