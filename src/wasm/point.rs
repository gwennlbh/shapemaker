use crate::Point;
use serde_wasm_bindgen;
use wasm_bindgen::{JsValue, convert::FromWasmAbi, convert::IntoWasmAbi};

impl From<Point> for JsValue {
    fn from(val: Point) -> Self {
        serde_wasm_bindgen::to_value(&val).unwrap()
    }
}

impl wasm_bindgen::describe::WasmDescribe for Point {
    fn describe() {
        JsValue::describe()
    }
}

impl wasm_bindgen::convert::IntoWasmAbi for Point {
    type Abi = <JsValue as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        serde_wasm_bindgen::to_value(&self).unwrap().into_abi()
    }
}

impl wasm_bindgen::convert::FromWasmAbi for Point {
    type Abi = <JsValue as FromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        serde_wasm_bindgen::from_value(unsafe { JsValue::from_abi(js) }).unwrap()
    }
}
