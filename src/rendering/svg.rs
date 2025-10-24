use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use crate::{Color, ColorMapping, Point, Region};

#[derive(Debug, Clone)]
pub struct Element {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub styles: HashMap<String, String>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(String),
    SVG(String),
}

impl Into<Node> for Element {
    fn into(self) -> Node {
        self.node()
    }
}

pub fn tag(tag: &str) -> Element {
    Element::new(tag)
}

impl Element {
    pub fn node(self) -> Node {
        Node::Element(self)
    }

    pub fn new(tag: &str) -> Self {
        Element {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            styles: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn attr(self, key: &str, value: impl Display) -> Self {
        // assert!(
        //     key != "style",
        //     "Use `style` method instead of `attr` for style attributes."
        // );
        let mut attributes = self.attributes.clone();
        attributes.insert(key.to_string(), value.to_string());
        Element { attributes, ..self }
    }

    /// Sets x and y
    pub fn coords(self, p: impl Into<(f32, f32)>) -> Self {
        let (x, y) = p.into();
        self.attr("x", x).attr("y", y)
    }

    pub fn fill(self, c: Color, colormap: &ColorMapping) -> Self {
        self.attr("fill", c.render(colormap))
    }

    /// Sets cx and cy
    pub fn center_position(self, p: impl Into<Point>, cell_size: usize) -> Self {
        let (x, y) = p.into().coords(cell_size);
        self.attr("cx", x).attr("cy", y)
    }

    /// Sets x1, y1 and x2, y2
    pub fn position_pair(
        self,
        p1: impl Into<Point>,
        p2: impl Into<Point>,
        cell_size: usize,
    ) -> Self {
        let (x1, y1) = p1.into().coords(cell_size);
        let (x2, y2) = p2.into().coords(cell_size);
        self.attr("x1", x1)
            .attr("y1", y1)
            .attr("x2", x2)
            .attr("y2", y2)
    }

    /// Sets x and y
    pub fn position(self, p: impl Into<Point>, cell_size: usize) -> Self {
        self.coords(p.into().coords(cell_size))
    }

    /// Sets width and height
    pub fn dimensions(self, p: impl Into<(usize, usize)>) -> Self {
        let (w, h) = p.into();
        self.attr("width", w).attr("height", h)
    }

    /// Sets width and height
    pub fn size(self, r: impl Into<Region>, cell_size: usize) -> Self {
        self.dimensions(r.into().size(cell_size))
    }

    /// Sets x, y, width and height according to the region
    pub fn region(self, r: impl Into<Region>, cell_size: usize) -> Self {
        let region: Region = r.into();
        self.position(region.start, cell_size)
            .size(region, cell_size)
    }

    pub fn style(self, key: &str, value: &str) -> Self {
        let mut styles = self.styles.clone();
        styles.insert(key.to_string(), value.to_string());
        Element { styles, ..self }
    }

    pub fn dataset(self, key: &str, value: &str) -> Self {
        self.attr(&format!("data-{key}"), value)
    }

    pub fn class(self, class: &str) -> Self {
        self.attr("class", class)
    }

    pub fn add(&mut self, child: impl Into<Node>) -> &mut Self {
        self.children.push(child.into());
        self
    }

    pub fn with_attributes(self, attributes: HashMap<String, String>) -> Self {
        Element { attributes, ..self }
    }

    pub fn wrapping(
        self,
        children: impl IntoIterator<Item = impl Into<Node>>,
    ) -> Self {
        Element {
            children: children.into_iter().map(|n| n.into()).collect(),
            ..self
        }
    }

    pub fn wrap(self, tag: &str, attrs: HashMap<String, String>) -> Self {
        Element {
            tag: tag.to_string(),
            styles: HashMap::new(),
            attributes: attrs,
            children: vec![Node::Element(self)],
        }
    }
}

pub enum PathInstruction {
    MoveTo((f32, f32)),
    LineTo((f32, f32)),
    HorizontalLineTo(f32),
    VerticalLineTo(f32),
    CurveTo((f32, f32), (f32, f32), (f32, f32)),
    SmoothCurveTo((f32, f32), (f32, f32)),
    QuadraticCurveTo((f32, f32), (f32, f32)),
    SmoothQuadraticCurveTo((f32, f32)),
    ArcTo((f32, f32), f32, bool, bool, (f32, f32)),
    ClosePath,
}

pub struct Path(Vec<PathInstruction>);

impl Path {
    pub fn new() -> Self {
        Path(Vec::new())
    }

    pub fn node(self) -> Node {
        self.element().node()
    }

    pub fn element(self) -> Element {
        tag("path").attr("d", self.to_string())
    }

    pub fn move_to(
        &mut self,
        p: impl Into<Point>,
        cell_size: usize,
    ) -> &mut Self {
        self.0
            .push(PathInstruction::MoveTo(p.into().coords(cell_size)));
        self
    }

    pub fn line_to(
        &mut self,
        p: impl Into<Point>,
        cell_size: usize,
    ) -> &mut Self {
        self.0
            .push(PathInstruction::LineTo(p.into().coords(cell_size)));
        self
    }

    pub fn quadratic_curve_to(
        &mut self,
        control: impl Into<(f32, f32)>,
        end: impl Into<Point>,
        cell_size: usize,
    ) -> &mut Self {
        self.0.push(PathInstruction::QuadraticCurveTo(
            control.into(),
            end.into().coords(cell_size),
        ));
        self
    }

    pub fn close(&mut self) -> &mut Self {
        self.0.push(PathInstruction::ClosePath);
        self
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}

impl Display for PathInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MoveTo((x, y)) => write!(f, "M {} {}", x, y),
            Self::LineTo((x, y)) => write!(f, "L {} {}", x, y),
            Self::HorizontalLineTo(x) => write!(f, "H {}", x),
            Self::VerticalLineTo(y) => write!(f, "V {}", y),
            Self::CurveTo((x1, y1), (x2, y2), (x3, y3)) => {
                write!(f, "C {} {} {} {} {} {}", x1, y1, x2, y2, x3, y3)
            }
            Self::SmoothCurveTo((x2, y2), (x3, y3)) => {
                write!(f, "S {} {} {} {}", x2, y2, x3, y3)
            }
            Self::QuadraticCurveTo((x1, y1), (x2, y2)) => {
                write!(f, "Q {} {} {} {}", x1, y1, x2, y2)
            }
            Self::SmoothQuadraticCurveTo((x2, y2)) => {
                write!(f, "T {} {}", x2, y2)
            }
            Self::ArcTo(
                (rx, ry),
                angle,
                large_arc_flag,
                sweep_flag,
                (x2, y2),
            ) => {
                write!(
                    f,
                    "A {rx} {ry} {angle} {large_arc_flag} {sweep_flag} {x2} {y2}"
                )
            }
            Self::ClosePath => write!(f, "Z"),
        }
    }
}

fn space_if(add_space: bool) -> &'static str {
    if add_space {
        " "
    } else {
        ""
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Text(text) => write!(f, "{}", quick_xml::escape::escape(text)),
            Node::SVG(svg) => write!(f, "{}", svg),
            Node::Element(Element {
                tag,
                attributes,
                styles,
                children,
            }) => {
                write!(f, "<{tag} ")?;

                let non_style_attributes: Vec<_> = attributes
                    .iter()
                    .filter(|(k, _)| *k != "style")
                    .sorted_by_key(|(k, _)| *k)
                    .collect();

                for (i, (key, value)) in non_style_attributes.iter().enumerate() {
                    write!(
                        f,
                        r#"{spacing}{key}="{value}""#,
                        spacing = space_if(i > 0),
                        key = key,
                        value = value
                            .replace("&", "&amp;")
                            .replace('"', "&quot;")
                            .replace("'", "&apos;")
                    )?;
                }

                if attributes.contains_key("style") || !styles.is_empty() {
                    write!(
                        f,
                        r#"{spacing}style="{value}""#,
                        spacing = space_if(non_style_attributes.len() > 0),
                        value = styles
                            .iter()
                            .map(|(k, v)| format!("{k}: {v};"))
                            .chain::<Option<String>>(
                                attributes.get("style").map(|s| s.to_string()),
                            )
                            .collect::<Vec<_>>()
                            .join(" ")
                    )?;
                }

                if children.is_empty() {
                    write!(f, "/>\n")?;
                } else {
                    write!(f, ">\n")?;

                    for child in children {
                        write!(f, "{}", child)?;
                    }

                    write!(f, "</{tag}>")?;
                }

                Ok(())
            }
        }
    }
}
