use super::{data::InitData, Matrix4, ModelRenderContext};
use crate::engine::{pipeline, Engine};
use gltf::mesh::Mode;
use wgpu::util::DeviceExt;
use wgpu::PrimitiveTopology;

pub struct Primitive {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

impl Primitive {
    pub fn new(engine: &Engine, primitive: &gltf::Primitive, data: &InitData) -> Self {
        let vertices = Vertex::build_vec(primitive, data.buffers).unwrap();

        let vertex_buffer = engine
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsage::VERTEX,
            });

        let buffer_descs = [Vertex::buffer_descriptor()];

        let descriptor = pipeline::PipelineDescriptor::builder()
            .vertex_shader(&data.vertex_shader)
            .fragment_shader(&data.fragment_shader)
            .vertex_buffers(&buffer_descs)
            .cull_mode(wgpu::CullMode::Back)
            .enable_depth_buffer(true)
            .primitive_topology(match primitive.mode() {
                Mode::Points => PrimitiveTopology::PointList,
                Mode::Lines => PrimitiveTopology::LineList,
                Mode::LineStrip => PrimitiveTopology::LineStrip,
                Mode::Triangles => PrimitiveTopology::TriangleList,
                Mode::TriangleStrip => PrimitiveTopology::TriangleStrip,
                mode => panic!(format!("Unsupported primitive mode: {:?}", mode)),
            });

        Self {
            pipeline: pipeline::build_pipeline(engine, descriptor.build()),
            vertex_buffer,
        }
    }

    pub fn render(&self, context: &mut ModelRenderContext, transform: &Matrix4) {
        let mut render_pass = context.begin_draw();
        render_pass.set_pipeline(&self.pipeline);
        // TODO: Set bind group
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..0, 0..0);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coord: [f32; 2],
    color: [f32; 4],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 0.0],
            tex_coord: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

impl Vertex {
    fn build_vec(
        primitive: &gltf::Primitive,
        buffers: &Vec<gltf::buffer::Data>,
    ) -> Option<Vec<Self>> {
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        reader.read_positions().map(|positions| {
            let mut buf = Vec::<Self>::with_capacity(positions.len());

            for position in positions {
                buf.push(Vertex {
                    position,
                    ..Default::default()
                })
            }

            if let Some(normals) = reader.read_normals() {
                for (index, normal) in normals.enumerate() {
                    buf[index].normal = normal;
                }
            }

            if let Some(tex_coords) = reader.read_tex_coords(0) {
                for (index, tex_coord) in tex_coords.into_f32().enumerate() {
                    buf[index].tex_coord = tex_coord;
                }
            }

            if let Some(colors) = reader.read_colors(0) {
                for (index, color) in colors.into_rgba_f32().enumerate() {
                    buf[index].color = color;
                }
            }

            buf
        })
    }

    fn buffer_descriptor<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                // position
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                // normal
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
                // tex_coord
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float2,
                },
                // color
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float4,
                },
            ],
        }
    }
}
