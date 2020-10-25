use crate::engine::prelude::*;
use std::rc::Rc;

pub struct EffectLayer {
    pipeline: wgpu::RenderPipeline,
    inputs: Vec<Rc<Texture>>,
    uniforms: Uniforms,
    uniforms_storage: UniformBuffer<Uniforms>,
}

impl EffectLayer {
    pub fn new(
        engine: &Engine,
        inputs: &[Rc<Texture>],
        fragment_shader: &Rc<Asset>,
        label: &str,
    ) -> Result<Self, EngineError> {
        // API for fragment shaders
        engine.add_asset(
            Path::new("postprocess/shaders/uniforms.glsl"),
            include_bytes!("shaders/uniforms.glsl"),
        );

        // Common vertex shader
        let vertex_shader = shaders::build(
            engine,
            &engine.add_asset(
                Path::new("postprocess/shaders/effect_layer.vert"),
                include_bytes!("shaders/effect_layer.vert"),
            ),
            None,
        )?;

        // Fragment shader
        let fragment_shader = shaders::build(engine, &fragment_shader, None)?;

        // Uniforms
        let uniforms = Uniforms::new(inputs.len() as u32);
        let uniforms_storage =
            UniformBuffer::init(&engine.device, uniforms, "EffectLayer::Uniforms");

        // Bind group layouts
        let mut bind_group_layouts = vec![uniforms_storage.get_layout()];
        for input in inputs {
            bind_group_layouts.push(input.get_layout());
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
            inputs: inputs.to_vec(),
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
}

impl Model for EffectLayer {
    fn render(&self, context: &mut ModelRenderContext) {
        self.uniforms_storage
            .copy_to_gpu(context.device, context.encoder, &self.uniforms);

        let mut render_pass = context.begin_draw();
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, self.uniforms_storage.get_bind_group(), &[]);
        for (index, input) in self.inputs.iter().enumerate() {
            render_pass.set_bind_group(index as u32 + 1, input.get_bind_group(), &[]);
        }
        render_pass.draw(0..6, 0..1);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Uniforms {
    args: [f32; 4],
    number_of_inputs: u32,
    time: f32,
    _padding: [f32; 2],
}

impl Uniforms {
    fn new(number_of_inputs: u32) -> Self {
        Self {
            number_of_inputs,
            args: [0.0; 4],
            time: 0.0,
            _padding: [0.0, 0.0],
        }
    }
}

unsafe impl bytemuck::Zeroable for Uniforms {}
unsafe impl bytemuck::Pod for Uniforms {}
