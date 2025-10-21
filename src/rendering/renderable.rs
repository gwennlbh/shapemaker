use super::svg;
use crate::{graphics::objects::ObjectSizes, ColorMapping};
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

/// Struct can be rendered as a SVG element
pub trait SVGRenderable {
    fn render_to_svg(
        &self,
        colormap: ColorMapping,
        cell_size: usize,
        object_sizes: ObjectSizes,
        id: &str,
    ) -> Result<svg::Node>;
}

/// Struct can be rendered as attributes of a SVG element
pub trait SVGAttributesRenderable {
    /// When merging multiple SVGAttributesRenderable, this string is used to join multiple values for the same key
    const MULTIPLE_VALUES_JOIN_BY: &'static str = ", ";

    fn render_to_svg_attributes(
        &self,
        colormap: ColorMapping,
        cell_size: usize,
        object_sizes: ObjectSizes,
        id: &str,
    ) -> Result<HashMap<String, String>>;
}

/// Struct can be rendered as a CSS ruleset (e.g. no selectors)
pub trait CSSRenderable {
    fn render_to_css_filled(&self, colormap: &ColorMapping) -> String;
    fn render_to_css_stroked(&self, colormap: &ColorMapping) -> String;
    fn render_to_css(
        &self,
        colormap: &ColorMapping,
        fill_as_stroke_color: bool,
    ) -> String {
        if fill_as_stroke_color {
            self.render_to_css_stroked(colormap)
        } else {
            self.render_to_css_filled(colormap)
        }
    }
}

impl<T: CSSRenderable, V: Clone + IntoIterator<Item = T>> CSSRenderable for V {
    fn render_to_css_filled(&self, colormap: &ColorMapping) -> String {
        self.clone()
            .into_iter()
            .map(|v| v.render_to_css_filled(colormap))
            .join("\n")
    }

    fn render_to_css_stroked(&self, colormap: &ColorMapping) -> String {
        self.clone()
            .into_iter()
            .map(|v| v.render_to_css_stroked(colormap))
            .join("\n")
    }
}

// We get the Option<T> implementation for free since Option<T> implements IntoIterator, and it works:
// None => empty iterator => empty hashmap,
// Some(T) => iterator with one element => one hashmap, no merging.
// I love Rust <3.

impl<T, V> SVGAttributesRenderable for V
where
    T: SVGAttributesRenderable,
    V: Clone + IntoIterator<Item = T>,
{
    fn render_to_svg_attributes(
        &self,
        colormap: ColorMapping,
        cell_size: usize,
        object_sizes: ObjectSizes,
        id: &str,
    ) -> Result<HashMap<String, String>> {
        let mut attrs = HashMap::<String, String>::new();
        for attrmap in self.clone().into_iter().map(|v| {
            v.render_to_svg_attributes(
                colormap.clone(),
                cell_size,
                object_sizes,
                id,
            )
            .unwrap()
        }) {
            for (key, value) in attrmap {
                if attrs.contains_key(&key) {
                    attrs.insert(
                        key.clone(),
                        format!(
                            "{}{}{}",
                            attrs[&key],
                            T::MULTIPLE_VALUES_JOIN_BY,
                            value
                        ),
                    );
                } else {
                    attrs.insert(key, value);
                }
            }
        }
        Ok(attrs)
    }
}
