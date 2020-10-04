use crate::engine::*;
use tobj;
use wgpu::util::DeviceExt;

pub type Resources = std::collections::HashMap<String, Vec<u8>>;

#[macro_export]
macro_rules! include_resources {
    ($($k:expr),*) => {{
        let mut resources = model::Resources::new();
        $(
            let file_name = std::path::Path::new($k).file_name().expect("Invalid file path");
            resources.insert(
                String::from(file_name.to_str().unwrap()),
                include_bytes!($k).to_vec(),
            );
        )*
        (resources)
    }};
}

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ModelVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
}

unsafe impl bytemuck::Pod for ModelVertex {}
unsafe impl bytemuck::Zeroable for ModelVertex {}

impl Vertex for ModelVertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
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
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: Option<texture::Texture>,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

impl Model {
    pub fn load_obj_buf(
        device: &wgpu::Device,
        obj_data: &[u8],
        resources: &Resources,
        texture_builder: &mut texture::TextureBuilder,
    ) -> Result<Self, failure::Error> {
        let mut reader = std::io::Cursor::new(obj_data);
        let (obj_models, obj_materials) = tobj::load_obj_buf(&mut reader, true, |path| {
            match resources.get(path.file_name().unwrap().to_str().unwrap()) {
                Some(data) => tobj::load_mtl_buf(&mut std::io::Cursor::new(data)),
                _ => Err(tobj::LoadError::OpenFileFailed),
            }
        })?;

        let mut materials = Vec::new();
        for mat in obj_materials {
            let diffuse_path = mat.diffuse_texture;
            let diffuse_texture = if diffuse_path.is_empty() {
                None
            } else {
                Some(texture_builder.diffuse(
                    &resources.get(&diffuse_path).expect(&format!(
                        "Resource {} is not defined in the resource object",
                        diffuse_path
                    )),
                    "diffuse_texture",
                ))
            };

            materials.push(Material {
                name: mat.name,
                diffuse_texture,
            });
        }

        let mut meshes = Vec::new();
        for m in obj_models {
            let mut vertices = Vec::new();
            for i in 0..m.mesh.positions.len() / 3 {
                vertices.push(ModelVertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                    normal: [
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2],
                    ],
                });
            }

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsage::VERTEX,
                label: Some("vertex_buffer"),
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                contents: bytemuck::cast_slice(&m.mesh.indices),
                usage: wgpu::BufferUsage::INDEX,
                label: Some("index_buffer"),
            });

            meshes.push(Mesh {
                name: m.name,
                vertex_buffer,
                index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            });
        }

        Ok(Self { meshes, materials })
    }
}
