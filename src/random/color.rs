use rand::Rng;
#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

use crate::Color;

#[cfg_attr(feature = "web", wasm_bindgen)]
pub fn random_color(except: Option<Color>) -> Color {
    let all = [
        Color::Black,
        Color::White,
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::Yellow,
        Color::Orange,
        Color::Purple,
        Color::Brown,
        Color::Cyan,
        Color::Pink,
        Color::Gray,
    ];
    let candidates = all
        .iter()
        .filter(|c| match except {
            None => true,
            Some(color) => &&color != c,
        })
        .collect::<Vec<_>>();

    *candidates[rand::thread_rng().gen_range(0..candidates.len())]
}
