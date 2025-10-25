use crate::State;
use shapemaker::*;

pub fn first_break() -> Scene<State> {
    Scene::<State>::new("first break").init(&|canvas, _| {
        canvas.set_background(Color::Black);
        Ok(())
    })
}
