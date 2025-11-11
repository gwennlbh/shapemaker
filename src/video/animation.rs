use crate::{Canvas, context::Context, video::hooks::InnerHook};
use easing_function::Easing;
pub use easing_function::{EasingFunction, easings};
use nanoid::nanoid;
use std::fmt::Display;

/// Arguments: animation progress (from 0.0 to 1.0), canvas, current ms
pub type AnimationUpdateFunction =
    dyn Fn(f32, &mut Canvas, usize) -> anyhow::Result<()> + Send + Sync;

pub struct Animation {
    pub name: String,
    // pub keyframes: Vec<Keyframe<C>>,
    pub update: Box<AnimationUpdateFunction>,
}

// pub struct Keyframe<C: Default> {
//     pub at: f32, // from 0 to 1
//     pub action: Box<RenderFunction<C>>,
// }

impl Animation {
    /// Example
    /// ```
    /// use shapemaker::*;
    /// Animation::new("example", &|t, canvas, _| {
    ///     let mut dot = canvas.root().object("dot");
    ///     dot.refill(Fill::Translucent(Color::Red, t));
    ///     Ok(())
    /// });
    /// ```
    pub fn new<N>(name: N, f: &'static AnimationUpdateFunction) -> Self
    where
        N: Display,
    {
        Self {
            name: format!("{}", name),
            update: Box::new(f),
        }
    }
}

impl From<(String, Box<AnimationUpdateFunction>)> for Animation {
    fn from((name, f): (String, Box<AnimationUpdateFunction>)) -> Self {
        Self { name, update: f }
    }
}

impl<C: Default> Context<'_, C> {
    /// duration is in milliseconds
    pub fn start_animation(
        &mut self,
        duration: usize,
        easing: impl Into<EasingFunction>,
        animation: Animation,
    ) {
        let start_ms = self.ms;
        let ms_range = start_ms..(start_ms + duration);
        let easing = easing.into();

        self.inner_hooks.push(InnerHook {
            once: false,
            when: Box::new(move |_, ctx, _| ms_range.contains(&ctx.ms)),
            render_function: Box::new(move |canvas, ms| {
                let t = (ms - start_ms) as f32 / duration as f32;
                (animation.update)(easing.ease(t), canvas, ms)
            }),
        })
    }

    /// duration is in milliseconds
    /// Animates with ease-in-out quadratic easing
    /// See animat_linear or animate_eased for other options
    pub fn animate(
        &mut self,
        duration: usize,
        f: &'static AnimationUpdateFunction,
    ) {
        self.animate_eased(duration, easings::EaseInOutQuadradic, f);
    }

    pub fn animate_linear(
        &mut self,
        duration: usize,
        f: &'static AnimationUpdateFunction,
    ) {
        self.animate_eased(duration, easings::Linear, f);
    }

    pub fn animate_eased(
        &mut self,
        duration: usize,
        easing: impl Into<EasingFunction>,
        f: &'static AnimationUpdateFunction,
    ) {
        self.start_animation(
            duration,
            easing.into(),
            Animation::new(format!("unnamed animation {}", nanoid!()), f),
        );
    }
}
