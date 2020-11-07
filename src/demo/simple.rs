use crate::engine::prelude::*;

pub struct Simple {
    pipeline: wgpu::RenderPipeline,
    output: Rc<Texture>,
}

impl Simple {
    pub fn new(engine: &Engine, output: Rc<Texture>) -> Result<Self, EngineError> {
        let vertex_shader = shaders::build(
            engine,
            &engine.load_asset(Path::new("shaders/layer.vert")),
            None,
        )?;
        let fragment_sahder = shaders::build(
            engine,
            &engine.load_asset(Path::new("shaders/simple.frag")),
            None,
        )?;

        let pipeline_descriptor = pipeline::PipelineDescriptor::builder()
            .label("Simple")
            .vertex_shader(&vertex_shader)
            .fragment_shader(&fragment_sahder)
            .build();

        Ok(Self {
            pipeline: pipeline::build_pipeline(engine, pipeline_descriptor),
            output,
        })
    }
}

impl Renderer for Simple {
    fn update(&mut self, _context: &mut RenderingContext) {}

    fn render(&mut self, context: &mut RenderingContext) {
        let mut encoder = context.create_encoder("Simple");
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &self.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.draw(0..6, 0..1);
        }
        context.submit(encoder);
    }
}
