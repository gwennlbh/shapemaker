use crate::video::hooks::{AttachHooks, Hook};

pub struct Scene<C: Default> {
    pub name: String,
    pub hooks: Vec<Hook<C>>,
}

impl<C: Default> Scene<C> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            hooks: Vec::new(),
        }
    }
}

impl<C: Default + 'static> AttachHooks<C> for Scene<C> {
    fn with_hook(self, hook: Hook<C>) -> Self {
        let mut hooks = self.hooks;
        let scene_name = self.name.clone();

        hooks.push(Hook {
            when: Box::new(move |canvas, ctx, prev_beat, prev_frame| {
                if ctx.current_scene.as_ref() == Some(&scene_name) {
                    (hook.when)(canvas, ctx, prev_beat, prev_frame)
                } else {
                    false
                }
            }),
            ..hook
        });

        Self { hooks, ..self }
    }

    fn init(
        self,
        render_function: &'static super::hooks::RenderFunction<C>,
    ) -> Self {
        self.with_hook(Hook {
            render_function: Box::new(render_function),
            when: Box::new(move |_, ctx, _, _| ctx.scene_frame() == Some(0)),
        })
    }
}
