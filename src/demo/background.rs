use crate::demo::state::State;
use crate::engine::*;
use std::rc::Rc;

pub struct Background {
    pipeline: wgpu::RenderPipeline,
    image: texture::Texture,
    output: Rc<texture::Texture>,
}

impl Background {
    pub fn new<T>(engine: &engine::Engine<T>, output: Rc<texture::Texture>) -> Box<Self> {
        let mut texture_builder = texture::TextureBuilder::new(engine);
        let image = texture_builder.diffuse(include_bytes!("assets/lakeus.jpg"), "lakeus.jpg");

        let pipeline = pipeline::PipelineBuilder::new()
            .vertex_shader(include_str!("shaders/layer.vert"), "layer.vert")
            .fragment_shader(include_str!("shaders/Background.frag"), "background.frag")
            .bind_objects(&[&image])
            .add_command_buffers(texture_builder.command_buffers)
            .build(engine);

        Box::new(Self {
            pipeline,
            image,
            output,
        })
    }
}

impl renderer::Renderer<State> for Background {
    fn update(&mut self, _ctx: &mut renderer::RenderingContext<State>) {}

    fn render(&mut self, ctx: &mut renderer::RenderingContext<State>) {
        let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &self.output.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
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
