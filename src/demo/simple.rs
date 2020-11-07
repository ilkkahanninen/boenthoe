use crate::engine::prelude::*;

pub struct Simple {
    pipeline: wgpu::RenderPipeline,
    output: Rc<Texture>,
    uniforms: UniformBuffer<SimpleUniforms>,
}

impl Simple {
    pub fn new(engine: &Engine, output: Rc<Texture>) -> Result<Self, EngineError> {
        let vertex_shader = shaders::build(
            engine,
            &engine.load_asset(Path::new("shaders/layer.vert")),
            None,
        )?;

        let fragment_shader = shaders::build(
            engine,
            &engine.load_asset(Path::new("shaders/simple.frag")),
            None,
        )?;

        let uniforms = UniformBuffer::new(&engine.device, "SimpleUniforms");
        let layouts = [uniforms.get_layout()];

        let pipeline_descriptor = pipeline::PipelineDescriptor::builder()
            .label("Simple")
            .vertex_shader(&vertex_shader)
            .fragment_shader(&fragment_shader)
            .bind_group_layouts(&layouts)
            .build();

        Ok(Self {
            pipeline: pipeline::build_pipeline(engine, pipeline_descriptor),
            output,
            uniforms,
        })
    }
}

impl Renderer for Simple {
    fn update(&mut self, context: &mut RenderingContext) {
        let mut encoder = context.create_encoder("update");
        self.uniforms.copy_to_gpu(
            &mut encoder,
            context.queue,
            &SimpleUniforms {
                time: context.time as f32,
            },
        );
        println!("Time: {}", context.time);
        context.submit(encoder);
    }

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
            render_pass.set_bind_group(0, self.uniforms.get_bind_group(), &[]);
            render_pass.draw(0..6, 0..1);
        }
        context.submit(encoder);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct SimpleUniforms {
    time: f32,
}

unsafe impl bytemuck::Zeroable for SimpleUniforms {}
unsafe impl bytemuck::Pod for SimpleUniforms {}
