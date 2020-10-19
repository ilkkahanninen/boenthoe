mod data;
mod node;
mod primitive;

use super::{Model, ModelProperties, ModelRenderContext};
use crate::engine::{camera::Camera, prelude::*, shaders};
use node::Node;
use std::path::Path;

pub struct GltfModel {
    view_projection_matrix: Matrix4,
    nodes: Vec<Node>,
    lights: Light,
    lights_buffer: StorageObject<Light>,
}

pub struct ModelRenderData<'a> {
    view_projection_matrix: &'a Matrix4,
    model_matrix: &'a Matrix4,
    lights: &'a wgpu::BindGroup,
}

impl Model for GltfModel {
    fn render(&self, context: &mut ModelRenderContext) {
        self.lights_buffer
            .copy_to_gpu(context.device, context.encoder, &self.lights);

        let transforms = ModelRenderData {
            view_projection_matrix: &self.view_projection_matrix,
            model_matrix: &cgmath::SquareMatrix::identity(),
            lights: self.lights_buffer.get_bind_group(),
        };

        for node in self.nodes.iter() {
            node.render(context, &transforms);
        }
    }

    fn set_view_projection_matrix(&mut self, matrix: &Matrix4) {
        self.view_projection_matrix = matrix.clone();
    }

    fn set_lighting(&mut self, lights: &[Light]) {
        self.lights = lights.get(0).unwrap().clone();
    }
}

impl GltfModel {
    pub fn new(
        engine: &Engine,
        source: &Asset,
        options: &ModelProperties,
    ) -> Result<Self, EngineError> {
        let (gltf, buffers, _images) = gltf::import_slice(source.data()?)
            .or_else(|error| Err(EngineError::parse_error(source, error)))?;

        let data = data::InitData::load(&engine.device, &buffers, options)?;

        let scene = gltf
            .default_scene()
            .or_else(|| gltf.scenes().next())
            .ok_or_else(|| EngineError::parse_error(source, "The file does not have any scenes"))?;

        Ok(GltfModel {
            view_projection_matrix: Camera::default().view_projection_matrix(),
            nodes: scene
                .nodes()
                .map(|node| Node::new(engine, &node, &data))
                .collect(),
            lights: Light::default(),
            lights_buffer: StorageObject::new(&engine.device, "Lights"),
        })
    }

    fn default_vertex_shader(device: &wgpu::Device) -> Result<wgpu::ShaderModule, EngineError> {
        shaders::build(
            device,
            &Asset::Ready {
                path: Path::new("engine::model::gltf.vert").to_path_buf(),
                data: include_bytes!("shaders/gltf.vert").to_vec(),
            },
        )
    }
}
