// mod bloom;
mod blur;

// pub use bloom::Bloom;
pub use blur::Blur;

use crate::engine::prelude::*;
use std::rc::Rc;

pub struct EffectLayer {
    pipeline: wgpu::RenderPipeline,
    uniforms: Uniforms,
    uniforms_storage: UniformBuffer<Uniforms>,
}

impl EffectLayer {
    pub fn new(
        engine: &Engine,
        input_bind_group_layouts: &[&wgpu::BindGroupLayout],
        fragment_shader: &Rc<Asset>,
        shader_macro_flags: &[&str],
        label: &str,
    ) -> Result<Self, EngineError> {
        // API for fragment shaders
        engine.add_asset(
            Path::new("effect_layer/shaders/uniforms.glsl"),
            include_bytes!("shaders/uniforms.glsl"),
        );

        // Common vertex shader
        let vertex_shader = shaders::build(
            engine,
            &engine.add_asset(
                Path::new("effect_layer/shaders/effect_layer.vert"),
                include_bytes!("shaders/effect_layer.vert"),
            ),
            None,
        )?;

        // Fragment shader
        let fragment_shader = shaders::build(
            engine,
            &fragment_shader,
            Some(&shaders::ShaderBuildOptions {
                macro_flags: shader_macro_flags,
            }),
        )?;

        // Uniforms
        let uniforms = Uniforms::new();
        let uniforms_storage =
            UniformBuffer::init(&engine.device, uniforms, &format!("{}::Uniforms", label));

        // Bind group layouts
        let mut bind_group_layouts = vec![uniforms_storage.get_layout()];
        for input in input_bind_group_layouts {
            bind_group_layouts.push(input);
        }

        // Pipeline
        let pipeline_descriptor = pipeline::PipelineDescriptor::builder()
            .label(label)
            .vertex_shader(&vertex_shader)
            .fragment_shader(&fragment_shader)
            .bind_group_layouts(&bind_group_layouts)
            .build();

        Ok(Self {
            pipeline: pipeline::build_pipeline(engine, pipeline_descriptor),
            uniforms,
            uniforms_storage,
        })
    }

    pub fn set_time(&mut self, time: f32) {
        self.uniforms.time = time;
    }

    pub fn set_args(&mut self, args: &[f32; 4]) {
        self.uniforms.args = args.clone();
    }

    pub fn set_arg(&mut self, index: usize, arg: f32) {
        self.uniforms.args[index] = arg;
    }

    fn update(&mut self, context: &mut RenderingContext) {
        let mut encoder = context.create_encoder();
        self.uniforms_storage
            .copy_to_gpu(context.device, &mut encoder, &self.uniforms);
        context.submit(encoder);
    }

    fn render(
        &mut self,
        context: &mut RenderingContext,
        inputs: &[&wgpu::BindGroup],
        output: &wgpu::TextureView,
    ) {
        let mut encoder = context.create_encoder();
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: output,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, self.uniforms_storage.get_bind_group(), &[]);
            for (index, input) in inputs.iter().enumerate() {
                render_pass.set_bind_group(index as u32 + 1, input, &[]);
            }
            render_pass.draw(0..6, 0..1);
        }
        context.submit(encoder);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Uniforms {
    args: [f32; 4],
    time: f32,
    _padding: [f32; 3],
}

impl Uniforms {
    fn new() -> Self {
        Self {
            args: [0.0; 4],
            time: 0.0,
            _padding: [0.0, 0.0, 0.0],
        }
    }
}

unsafe impl bytemuck::Zeroable for Uniforms {}
unsafe impl bytemuck::Pod for Uniforms {}
