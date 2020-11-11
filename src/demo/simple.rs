use crate::engine::prelude::*;

pub struct Simple {
    pipeline: wgpu::RenderPipeline,
    uniforms: UniformBuffer<SimpleUniforms>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
            ],
        }
    }
}
const VERTICES: &[Vertex] = &[
    Vertex {
        // tl
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        // bl
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        // tr
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        // br
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
];

const INDICES: &[u32] = &[0, 1, 3, 3, 2, 0];

impl Simple {
    pub fn new(engine: &Engine) -> Result<Self, EngineError> {
        use wgpu::util::DeviceExt;

        let vertex_shader = shaders::build(
            engine,
            &engine.load_asset(Path::new("shaders/simple.vert")),
            None,
        )?;

        let fragment_shader = shaders::build(
            engine,
            &engine.load_asset(Path::new("shaders/simple.frag")),
            None,
        )?;

        let uniforms = UniformBuffer::default(&engine.device, "SimpleUniforms");
        let layouts = [uniforms.get_layout()];
        let vertex_buffer_desc = [Vertex::desc()];
        let vertex_buffer = engine
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsage::VERTEX,
                label: Some("Layer vertices"),
            });
        let index_buffer = engine
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsage::INDEX,
                label: Some("Layer indices"),
            });

        let pipeline_descriptor = pipeline::PipelineDescriptor::builder()
            .label("SimplePipeline")
            .vertex_shader(&vertex_shader)
            .fragment_shader(&fragment_shader)
            .bind_group_layouts(&layouts)
            .vertex_buffers(&vertex_buffer_desc)
            .blend_mode(pipeline::BlendMode::Screen)
            .build();

        Ok(Self {
            pipeline: pipeline::build_pipeline(engine, pipeline_descriptor),
            uniforms,
            vertex_buffer,
            index_buffer,
        })
    }
}

impl Renderer for Simple {
    fn update(&mut self, context: &mut RenderingContext) {
        self.uniforms.copy_to_gpu(
            context.queue,
            &SimpleUniforms {
                time: context.time as f32,
                _padding: [0.0; 3],
            },
        );
    }

    fn render(&mut self, context: &mut RenderingContext) {
        let mut encoder = context.create_encoder("Simple::render::Encoder");
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &context.output,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: context.time.sin() * 0.5 + 0.5,
                            g: 0.0,
                            b: context.time.cos() * 0.5 + 0.5,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, self.uniforms.get_bind_group(), &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..));
            render_pass.draw_indexed(0..6, 0, 0..1);
        }
        context.submit(encoder);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct SimpleUniforms {
    time: f32,
    _padding: [f32; 3],
}

unsafe impl bytemuck::Zeroable for SimpleUniforms {}
unsafe impl bytemuck::Pod for SimpleUniforms {}

impl Default for SimpleUniforms {
    fn default() -> Self {
        Self {
            time: 123.123,
            _padding: [0.0; 3],
        }
    }
}
