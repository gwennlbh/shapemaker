#![allow(unused)]

use std::sync::Mutex;

use once_cell::sync::Lazy;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

use crate::{
    Canvas, Color, ColorMapping, Fill, Filter, Layer, Object, Point,
    SVGRenderable,
};

use super::LayerWeb;

static WEB_CANVAS: Lazy<Mutex<Canvas>> =
    Lazy::new(|| Mutex::new(Canvas::default_settings()));

pub(super) fn canvas() -> std::sync::MutexGuard<'static, Canvas> {
    WEB_CANVAS.lock().unwrap()
}

// Can't bind Color.name directly, see https://github.com/rustwasm/wasm-bindgen/issues/1715
#[wasm_bindgen]
pub fn color_name(c: Color) -> String {
    c.name()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn render_image(opacity: f32, color: Color) -> Result<(), JsValue> {
    let mut canvas = Canvas::with_colors(ColorMapping {
        black: "#ffffff".into(),
        white: "#ffffff".into(),
        red: "#cf0a2b".into(),
        green: "#22e753".into(),
        blue: "#2734e6".into(),
        yellow: "#f8e21e".into(),
        orange: "#f05811".into(),
        purple: "#6a24ec".into(),
        brown: "#a05634".into(),
        pink: "#e92e76".into(),
        gray: "#81a0a8".into(),
        cyan: "#4fecec".into(),
    });

    *WEB_CANVAS.lock().unwrap() = canvas;
    render_canvas_at(String::from("body"));

    Ok(())
}

#[wasm_bindgen]
pub fn map_to_midi_controller() {}

#[wasm_bindgen]
pub fn render_canvas_into(selector: String) {
    append_new_div_inside(render_canvas(), selector)
}

#[wasm_bindgen]
pub fn render_canvas_at(selector: String) {
    replace_content_with(render_canvas(), selector);
}

#[wasm_bindgen]
pub enum MidiEvent {
    Note,
    ControlChange,
}

#[wasm_bindgen]
pub struct MidiEventData([u8; 3]);

#[wasm_bindgen]
pub struct MidiPitch(u8);

#[wasm_bindgen]
impl MidiPitch {
    pub fn octave(&self) -> u8 {
        self.0 / 12
    }
}

pub struct Percentage(pub f32);

impl From<u8> for Percentage {
    fn from(value: u8) -> Self {
        Self(value as f32 / 127.0)
    }
}

pub enum MidiMessage {
    NoteOn(MidiPitch, Percentage),
    NoteOff(MidiPitch),
    PedalOn,
    PedalOff,
    ControlChange(u8, Percentage),
}

impl From<(MidiEvent, MidiEventData)> for MidiMessage {
    fn from(value: (MidiEvent, MidiEventData)) -> Self {
        match value {
            (MidiEvent::Note, MidiEventData([pitch, velocity, _])) => {
                if velocity == 0 {
                    MidiMessage::NoteOff(MidiPitch(pitch))
                } else {
                    MidiMessage::NoteOn(MidiPitch(pitch), velocity.into())
                }
            }
            (MidiEvent::ControlChange, MidiEventData([64, value, _])) => {
                if value == 0 {
                    MidiMessage::PedalOff
                } else {
                    MidiMessage::PedalOn
                }
            }
            (MidiEvent::ControlChange, MidiEventData([_, controller, value])) => {
                MidiMessage::ControlChange(controller, value.into())
            }
        }
    }
}

#[wasm_bindgen]
pub fn render_canvas() -> String {
    let can = canvas();
    can.render_to_svg(
        can.colormap.clone(),
        can.cell_size,
        can.object_sizes,
        "web_root_canvas",
    )
    .unwrap_throw()
    .to_string()
}

#[wasm_bindgen]
pub fn set_palette(palette: ColorMapping) {
    canvas().colormap = palette;
}

#[wasm_bindgen]
pub fn get_layer(name: &str) -> Result<LayerWeb, JsValue> {
    match canvas().layer_safe(name) {
        Some(layer) => Ok(LayerWeb {
            name: layer.name.clone(),
        }),
        None => Err(JsValue::from_str("Layer not found")),
    }
}

#[wasm_bindgen]
pub fn random_linelikes(name: &str) -> LayerWeb {
    let layer = canvas().random_linelikes(name);
    canvas().add_or_replace_layer(layer);
    LayerWeb {
        name: name.to_string(),
    }
}

fn document() -> web_sys::Document {
    let window = web_sys::window().expect_throw("no global `window` exists");
    window
        .document()
        .expect_throw("should have a document on window")
}

fn query_selector(selector: String) -> web_sys::Element {
    document()
        .query_selector(&selector)
        .expect_throw(&format!("selector '{}' not found", selector))
        .expect_throw(
            "could not get the element, but is was found (shouldn't happen)",
        )
}

pub(super) fn append_new_div_inside(content: String, selector: String) {
    let output = document().create_element("div").unwrap();
    output.set_class_name("frame");
    output.set_inner_html(&content);
    query_selector(selector).append_child(&output).unwrap();
}

pub(super) fn replace_content_with(content: String, selector: String) {
    query_selector(selector).set_inner_html(&content);
}
