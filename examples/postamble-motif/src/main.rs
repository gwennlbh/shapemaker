use enum2str::EnumStr;
use shapemaker::*;

fn main() {
    let mut canvas = Canvas::new(27, 48);
    canvas.set_background(Color::White);
    canvas
        .root()
        .add_anon(Letter::P.object(Region::new(0, 0, 0, 1).unwrap()));
    canvas
        .root()
        .add_anon(Letter::O.object(Region::new(1, 0, 1, 0).unwrap()));

    canvas
        .render_to_svg_file("out.svg")
        .expect("Failed to render SVG");
    canvas
        .render_to_png("out.png", 500)
        .expect("Failed to render PNG");
}

#[derive(Debug, Clone, Copy, EnumStr)]
enum Letter {
    P,
    O,
    S,
    T,
    A,
    M,
    B,
    L,
    E,
}

impl Letter {
    fn object(&self, at: Region) -> ColoredObject {
        Image(
            at,
            // format!(
            //     "data:image/svg+xml;base64,{}",
            //     BASE64.encode(
            //         fs::read_to_string(format!(
            //             "./letters/{}.svg",
            //             self.to_string().to_lowercase()
            //         ))
            //         .expect("Letter {self} not found")
            //         .replace(r#"<?xml version="1.0" encoding="UTF-8"?>"#, ""),
            //     )
            // ),
            format!(
                "{}/letters/{}.svg",
                std::path::absolute(".").unwrap().display(),
                self.to_string().to_lowercase()
            ),
        )
        .into()
    }
}
