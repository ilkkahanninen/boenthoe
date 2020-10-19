use crate::engine::{model, prelude::*, scripts};
use std::path::Path;

pub struct TestEffect {
    model: Box<dyn model::Model>,
    script: scripts::Script,
    depth_buffer: Texture,
    camera: Camera,
}

impl TestEffect {
    pub fn attach(engine: &Engine) -> Result<(), EngineError> {
        let model = model::load(
            engine,
            &engine.load_asset(&Path::new("assets/Box.glb")),
            &model::ModelProperties::default(),
        )?;
        let script = scripts::build(&engine.load_asset(&Path::new("assets/camerajump.boe")))?;
        let depth_buffer = textures::depth_buffer(engine);
        let camera = Camera::default();

        Ok(engine.add_renderer(Box::new(Self {
            model,
            script,
            depth_buffer,
            camera,
        })))
    }
}

impl Renderer for TestEffect {
    fn reload_assets(&mut self, assets: &AssetLibrary) -> Result<(), EngineError> {
        if let Some(script) = assets.changed("camerajump.boe") {
            println!("TestEffect: reload script");
            self.script = scripts::build(&script)?;
        }

        Ok(())
    }

    fn update(&mut self, ctx: &mut RenderingContext) {
        self.script.set_time(ctx.time);
        self.camera.eye = (
            self.script.get("eye_x").to_f() as f32,
            self.script.get("eye_y").to_f() as f32,
            self.script.get("eye_z").to_f() as f32,
        )
            .into();
        self.model
            .set_view_projection_matrix(&self.camera.view_projection_matrix());
        self.model.set_lighting(&[Light {
            color: (1.0, 0.9, 0.8, 1.0).into(),
            position: (5.0, 10.0, -15.0).into(),
        }]);
    }

    fn render(&mut self, ctx: &mut RenderingContext) {
        ctx.clear(
            wgpu::Color {
                r: 0.02,
                g: 0.05,
                b: 0.1,
                a: 1.0,
            },
            None,
            Some(&self.depth_buffer.view),
        );

        self.model.render(&mut model::ModelRenderContext {
            device: ctx.device,
            output: ctx.output,
            encoder: ctx.encoder,
            depth_buffer: &self.depth_buffer,
        });
    }
}
