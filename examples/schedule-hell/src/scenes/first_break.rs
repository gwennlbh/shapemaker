use crate::State;
use shapemaker::*;

fn random_shape(at: Point) -> ColoredObject {
    let thickness = 7.0;
    match rand::random_range(1..=5) {
        1 => Object::BigCircle(at),
        2 => Object::CurveInward(at, at.translated(1, 1), thickness),
        3 => Object::CurveOutward(at, at.translated(1, 1), thickness),
        4 => Object::Line(at, at.translated(1, 1), thickness),
        5 => Object::Line(at.translated(0, 1), at.translated(1, 0), thickness),
        _ => panic!("souhldn't happend, update rand:: call"),
    }
    .colored(Color::Black)
}

pub fn first_break() -> Scene<State> {
    Scene::<State>::new("first break")
        .init(&|canvas, _| {
            canvas.colormap = ColorMapping {
                black: "#676E95".to_string(),
                white: "#ffffff".to_string(),
                red: "#ff5572".to_string(),
                green: "#a9c77d".to_string(),
                blue: "#82AAFF".to_string(),
                yellow: "#FFCB6B".to_string(),
                orange: "#FFCB6B".to_string(),
                purple: "#C792EA".to_string(),
                brown: "#ff5572".to_string(),
                pink: "#C792EA".to_string(),
                gray: "#ffffff".to_string(),
                cyan: "#89DDFF".to_string(),
            };

            canvas.clear();

            let world = canvas.world_region.clone();
            let shapes_area = world.resized(-4, -4);
            let mut tiling = Layer::new("tiling");
            let mut shapes = Layer::new("shapes");

            for (i, point) in world.iter().enumerate() {
                let bgcolor =
                    Color::random_except(&mut rand::rng(), Color::Black);

                tiling.add(
                    format!("tile{i}"),
                    Object::Rectangle(point, point).colored(bgcolor),
                );

                if shapes_area.contains(&point) {
                    shapes.add(format!("shape{i}"), random_shape(point));
                }
            }

            canvas.add_layer(shapes);
            canvas.add_layer(tiling);

            canvas.reorder_layers(vec!["shapes", "tiling", "root"]);
            Ok(())
        })
        .on_note("goup", &|canvas, _| {
            let world = canvas.world_region.clone();
            let shapes = &mut canvas.layer("shapes").objects;

            for (i, point) in world.iter().enumerate() {
                let shape = shapes.get_mut(&format!("shape{i}"));

                if let Some(shape) = shape {
                    *shape = random_shape(point);
                }
            }

            Ok(())
        })
        .on_note("powerful clap hit, clap, perclap", &|canvas, _| {
            let world = canvas.world_region.clone();

            for (i, _) in world.iter().enumerate() {
                canvas
                    .layer("tiling")
                    .objects
                    .get_mut(&format!("tile{i}"))
                    .unwrap()
                    .recolor(Color::random_except(
                        &mut rand::rng(),
                        Color::Black,
                    ));
            }

            Ok(())
        })
}
