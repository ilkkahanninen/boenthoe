use crate::demo::state::State;
use crate::engine::*;
use std::rc::Rc;

pub struct PostProcess {
    pipeline: wgpu::RenderPipeline,
    input: Rc<texture::Texture>,
    vignette: texture::Texture,
}

impl PostProcess {
    pub fn new<T>(engine: &engine::Engine<T>, input: Rc<texture::Texture>) -> Box<Self> {
        let mut texture_builder = texture::TextureBuilder::new(engine);
        let vignette = texture_builder.diffuse(include_bytes!("assets/vignette.jpg"), "vignette");

        let pipeline = pipeline::PipelineBuilder::new()
            .vertex_shader(include_str!("shaders/layer.vert"), "layer.vert")
            .fragment_shader(include_str!("shaders/postprocess.frag"), "postprocess.frag")
            .bind_objects(&[input.as_ref(), &vignette])
            .add_command_buffers(texture_builder.command_buffers)
            .build(engine);

        Box::new(Self {
            pipeline,
            input,
            vignette,
        })
    }
}

impl renderer::Renderer<State> for PostProcess {
    fn update(&mut self, _ctx: &mut renderer::RenderingContext<State>) {}

    fn render(&mut self, ctx: &mut renderer::RenderingContext<State>) {
        let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: ctx.output,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.input.bind_group, &[]);
        render_pass.set_bind_group(1, &self.vignette.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}
