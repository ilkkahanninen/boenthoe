use crate::demo::state::State;
use crate::engine::*;
use std::rc::Rc;

pub struct Blur {
    passes: [BlurPass; 2],
    input: Rc<texture::Texture>,
    output: Rc<texture::Texture>,
}

struct BlurPass {
    pipeline: wgpu::RenderPipeline,
    output: Option<texture::Texture>,
}

impl Blur {
    pub fn new<T>(
        engine: &engine::Engine<T>,
        input: Rc<texture::Texture>,
        output: Rc<texture::Texture>,
    ) -> Box<Self> {
        let temp_buffer = engine.create_render_buffer();

        let pipeline1 = pipeline::PipelineBuilder::new()
            .vertex_shader(include_str!("shaders/layer.vert"), "layer.vert")
            .fragment_shader(include_str!("shaders/blur_h.frag"), "blur_h.frag")
            .bind_objects(&[input.as_ref()])
            .build(engine);

        let pipeline2 = pipeline::PipelineBuilder::new()
            .vertex_shader(include_str!("shaders/layer.vert"), "layer.vert")
            .fragment_shader(include_str!("shaders/blur_v.frag"), "blur_v.frag")
            .bind_objects(&[input.as_ref()])
            .build(engine);

        Box::new(Self {
            passes: [
                BlurPass {
                    pipeline: pipeline1,
                    output: Some(temp_buffer),
                },
                BlurPass {
                    pipeline: pipeline2,
                    output: None,
                },
            ],
            input,
            output,
        })
    }
}

impl renderer::Renderer<State> for Blur {
    fn update(&mut self, _ctx: &mut renderer::RenderingContext<State>) {}

    fn render(&mut self, ctx: &mut renderer::RenderingContext<State>) {
        let mut input = self.input.as_ref();
        for blur_pass in self.passes.iter() {
            let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: if let Some(output) = blur_pass.output.as_ref() {
                        &output.view
                    } else {
                        &self.output.view
                    },
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&blur_pass.pipeline);
            render_pass.set_bind_group(0, &input.bind_group, &[]);
            render_pass.draw(0..6, 0..1);

            if let Some(output) = blur_pass.output.as_ref() {
                input = &output;
            }
        }
    }
}
