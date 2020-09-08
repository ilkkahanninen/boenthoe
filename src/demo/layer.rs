use crate::demo::state::State;
use crate::engine::model::Vertex;
use crate::engine::*;

pub struct Layer {
    pipeline: wgpu::RenderPipeline,
    image: texture::Texture,
}

impl Layer {
    pub fn new<T>(engine: &engine::Engine<T>) -> Box<Self> {
        let device = &engine.device;

        let mut texture_builder = texture::TextureBuilder::new(engine);
        let image = texture_builder.diffuse(include_bytes!("assets/test.png"), "test.png");

        let pipeline = pipeline::PipelineBuilder::new()
            .vertex_shader(include_str!("shaders/layer.vert"), "layer.vert")
            .fragment_shader(include_str!("shaders/layer.frag"), "layer.frag")
            .bind_objects(&[&image])
            .add_command_buffers(texture_builder.command_buffers)
            .build(engine);

        Box::new(Self { pipeline, image })
    }
}

impl renderer::Renderer<State> for Layer {
    fn update(&mut self, _ctx: &mut renderer::RenderingContext<State>) {}

    fn render(&mut self, ctx: &mut renderer::RenderingContext<State>) {
        let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: ctx.output,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.01,
                        g: 0.01,
                        b: 0.01,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.image.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}
