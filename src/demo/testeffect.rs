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
            &engine.load_asset(&Path::new("assets/Duck.glb")),
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
        let time = ctx.time as f32;
        self.script.set_time(ctx.time);
        self.camera.eye = (
            time.sin() * 4.0,
            (time * 2.7).sin() + 1.0,
            time.cos() * 4.0
            // self.script.get("eye_x").to_f() as f32,
            // self.script.get("eye_y").to_f() as f32,
            // self.script.get("eye_z").to_f() as f32,
        )
            .into();
        self.camera.target.y = 1.0;

        self.model.set_camera(&self.camera);

        self.model.set_lighting(&[
            Light::Directional {
                direction: (1.0, -1.0, -0.33).into(),
                ambient: (0.0, 0.0, 0.0).into(),
                diffuse: (1.0, 0.0, 1.0).into(),
                specular: (1.0, 1.0, 1.0).into(),
            },
            Light::Directional {
                direction: (-1.0, -1.0, 0.33).into(),
                ambient: (0.0, 0.0, 0.0).into(),
                diffuse: (0.0, 1.0, 1.0).into(),
                specular: (1.0, 1.0, 1.0).into(),
            },
        ]);
    }

    fn render(&mut self, ctx: &mut RenderingContext) {
        ctx.clear(wgpu::Color::BLACK, None, Some(&self.depth_buffer.view));

        self.model.render(&mut model::ModelRenderContext {
            device: ctx.device,
            output: ctx.output,
            encoder: ctx.encoder,
            depth_buffer: &self.depth_buffer,
        });
    }
}
