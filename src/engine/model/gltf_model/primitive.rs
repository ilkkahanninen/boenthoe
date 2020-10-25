use super::{data::InitData, Matrix4, ModelRenderContext, ModelRenderData};
use crate::engine::prelude::*;
use gltf::mesh::Mode;
use wgpu::util::DeviceExt;
use wgpu::PrimitiveTopology;

pub struct Primitive {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_elements: u32,
    uniforms_storage: UniformBuffer<Uniforms>,
    material: Material,
}

impl Primitive {
    pub fn new(engine: &Engine, primitive: &gltf::Primitive, data: &InitData) -> Self {
        let label = format!("gltf::Primitive[{}]", primitive.index());

        // Vertices
        let vertices = Vertex::build_vec(primitive, data.buffers).unwrap();
        let vertex_buffer = engine
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{}::vertex_buffer", &label)),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsage::VERTEX,
            });
        let vertex_buffer_descriptors = [Vertex::buffer_descriptor()];

        // Indices
        let reader = primitive.reader(|buffer| Some(&data.buffers[buffer.index()]));
        let indices: Vec<u32> = reader.read_indices().unwrap().into_u32().collect();
        let index_buffer = engine
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{}::index_buffer", &label)),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsage::INDEX,
            });

        // Material
        let primitive_material = primitive.material();
        let pbr_model = primitive_material.pbr_metallic_roughness();
        let material = Material {
            base_color: pbr_model.base_color_factor().into(),
            textures: data.create_texture_bind_group(engine, &primitive_material),
            metallic_factor: pbr_model.metallic_factor(),
            roughness_factor: pbr_model.roughness_factor(),
        };

        // Uniforms
        let uniforms_storage = UniformBuffer::default(&engine.device, "gltf::Uniforms");
        let light_layout =
            StorageBuffer::<LightBufferObject>::create_layout(&engine.device, "gltf::Lights");
        let bind_group_layouts = [
            uniforms_storage.get_layout(),
            &light_layout,
            &data.textures_bind_group_layout,
        ];

        // Render pipeline
        let label = format!("{}::pipeline", &label);
        let pipeline_descriptor = pipeline::PipelineDescriptor::builder()
            .label(&label)
            .vertex_shader(&data.vertex_shader)
            .fragment_shader(&data.fragment_shader)
            .vertex_buffers(&vertex_buffer_descriptors)
            .bind_group_layouts(&bind_group_layouts)
            .cull_mode(if primitive_material.double_sided() {
                wgpu::CullMode::None
            } else {
                wgpu::CullMode::Back
            })
            .enable_depth_buffer(true)
            .primitive_topology(match primitive.mode() {
                Mode::Points => PrimitiveTopology::PointList,
                Mode::Lines => PrimitiveTopology::LineList,
                Mode::LineStrip => PrimitiveTopology::LineStrip,
                Mode::Triangles => PrimitiveTopology::TriangleList,
                Mode::TriangleStrip => PrimitiveTopology::TriangleStrip,
                mode => panic!(format!("Unsupported primitive mode: {:?}", mode)),
            })
            .build();

        Self {
            pipeline: pipeline::build_pipeline(engine, pipeline_descriptor),
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            uniforms_storage,
            material,
        }
    }

    pub fn render(&self, context: &mut ModelRenderContext, data: &ModelRenderData) {
        // Update uniforms buffer
        self.uniforms_storage.copy_to_gpu(
            context.device,
            &mut context.encoder,
            &Uniforms::new(data, &self.material),
        );

        // Render
        let mut render_pass = context.begin_draw();
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, self.uniforms_storage.get_bind_group(), &[]);
        render_pass.set_bind_group(1, data.lights, &[]);
        render_pass.set_bind_group(2, &self.material.textures, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..));
        render_pass.draw_indexed(0..self.num_elements, 0, 0..1);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coord: [f32; 2],
    color: [f32; 4],
    tangent: [f32; 4],
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
            tangent: [0.0, 0.0, 0.0, 0.0],
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

            if let Some(tangents) = reader.read_tangents() {
                for (index, tangent) in tangents.enumerate() {
                    buf[index].tangent = tangent;
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
                // tangent
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float4,
                },
            ],
        }
    }
}

struct Material {
    base_color: Vector4,
    textures: wgpu::BindGroup,
    metallic_factor: f32,
    roughness_factor: f32,
}

#[derive(Debug, Copy, Clone)]
struct Uniforms {
    // 16 byte properties
    view_projection_matrix: Matrix4,
    model_matrix: Matrix4,
    eye_position: Vector4,
    base_color: Vector4,

    // 4 byte properties
    number_of_lights: u32,
    metallic_factor: f32,
    roughness_factor: f32,

    // Pad to 16 byte stride
    _padding: [f32; 1],
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            view_projection_matrix: cgmath::SquareMatrix::identity(),
            model_matrix: cgmath::SquareMatrix::identity(),
            eye_position: (0.0, 0.0, 0.0, 1.0).into(),
            base_color: (1.0, 1.0, 1.0, 1.0).into(),
            number_of_lights: 0,
            metallic_factor: 1.0,
            roughness_factor: 1.0,
            _padding: [1234.5678; 1],
        }
    }
}

impl Uniforms {
    fn new(render_data: &ModelRenderData, material: &Material) -> Self {
        Self {
            view_projection_matrix: render_data.view_projection_matrix.clone(),
            model_matrix: render_data.model_matrix.clone(),
            eye_position: render_data.eye_position.to_homogeneous(),
            base_color: material.base_color,
            number_of_lights: render_data.number_of_lights,
            metallic_factor: material.metallic_factor,
            roughness_factor: material.roughness_factor,
            ..Default::default()
        }
    }
}

unsafe impl bytemuck::Zeroable for Uniforms {}
unsafe impl bytemuck::Pod for Uniforms {}
