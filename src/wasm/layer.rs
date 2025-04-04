use super::canvas;
use crate::{
    wasm::{append_new_div_inside, render_canvas, replace_content_with, RNG},
    Color, Fill, Filter, Layer, Object, Point,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
pub struct LayerWeb {
    pub name: String,
}

#[wasm_bindgen]
pub fn new_layer(name: &str) -> LayerWeb {
    canvas().add_or_replace_layer(Layer::new(name));
    LayerWeb {
        name: name.to_string(),
    }
}

#[wasm_bindgen]
impl LayerWeb {
    pub fn render(&self) -> String {
        render_canvas()
    }

    pub fn render_into(&self, selector: String) {
        append_new_div_inside(self.render(), selector)
    }

    pub fn render_at(self, selector: String) {
        replace_content_with(self.render(), selector)
    }

    pub fn paint_all(&self, color: Color, opacity: Option<f32>, filter: Filter) {
        canvas()
            .layer(&self.name)
            .paint_all_objects(Fill::Translucent(color, opacity.unwrap_or(1.0)));
        canvas().layer(&self.name).filter_all_objects(filter);
    }

    pub fn random(name: &str) -> Self {
        unsafe {
            #[allow(static_mut_refs)]
            canvas().random_layer(&mut RNG, name);
        }
        LayerWeb {
            name: name.to_string(),
        }
    }

    pub fn new_line(
        &self,
        name: &str,
        start: Point,
        end: Point,
        thickness: f32,
        color: Color,
    ) {
        canvas()
            .layer(name)
            .set_object(name, Object::Line(start, end, thickness).colored(color))
    }
    pub fn new_curve_outward(
        &self,
        name: &str,
        start: Point,
        end: Point,
        thickness: f32,
        color: Color,
    ) {
        canvas().layer(name).set_object(
            name,
            Object::CurveOutward(start, end, thickness).colored(color),
        )
    }
    pub fn new_curve_inward(
        &self,
        name: &str,
        start: Point,
        end: Point,
        thickness: f32,
        color: Color,
    ) {
        canvas().layer(name).set_object(
            name,
            Object::CurveInward(start, end, thickness).colored(color),
        )
    }
    pub fn new_small_circle(&self, name: &str, center: Point, color: Color) {
        canvas()
            .layer(name)
            .set_object(name, Object::SmallCircle(center).colored(color))
    }
    pub fn new_dot(&self, name: &str, center: Point, color: Color) {
        canvas()
            .layer(name)
            .set_object(name, Object::Dot(center).colored(color))
    }
    pub fn new_big_circle(&self, name: &str, center: Point, color: Color) {
        canvas()
            .layer(name)
            .set_object(name, Object::BigCircle(center).colored(color))
    }
    pub fn new_text(
        &self,
        name: &str,
        anchor: Point,
        text: String,
        font_size: f32,
        color: Color,
    ) {
        canvas().layer(name).set_object(
            name,
            Object::Text(anchor, text, font_size).colored(color),
        )
    }
    pub fn new_rectangle(
        &self,
        name: &str,
        topleft: Point,
        bottomright: Point,
        color: Color,
    ) {
        canvas().layer(name).set_object(
            name,
            Object::Rectangle(topleft, bottomright).colored(color),
        )
    }
}
