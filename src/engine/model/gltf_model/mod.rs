mod data;
mod node;
mod primitive;

use super::{Model, ModelRenderContext};
use crate::engine::{camera::Camera, prelude::*, shaders};
use node::Node;
use std::path::Path;

pub struct GltfModel {
    view_projection_matrix: Matrix4,
    nodes: Vec<Node>,
}

pub struct TransformMatrices<'a> {
    view_projection: &'a Matrix4,
    space: &'a Matrix4,
}

impl Model for GltfModel {
    fn render(&self, context: &mut ModelRenderContext) {
        let transforms = TransformMatrices {
            view_projection: &self.view_projection_matrix,
            space: &cgmath::SquareMatrix::identity(),
        };
        for node in self.nodes.iter() {
            node.render(context, &transforms);
        }
    }

    fn set_view_projection_matrix(&mut self, matrix: &Matrix4) {
        self.view_projection_matrix = matrix.clone();
    }
}

impl GltfModel {
    pub fn new(engine: &Engine, source: &Asset) -> Result<Self, EngineError> {
        let (gltf, buffers, _images) = gltf::import_slice(source.data()?)
            .or_else(|error| Err(EngineError::parse_error(source, error)))?;

        let data = data::InitData::load(&engine.device, &buffers)?;

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
