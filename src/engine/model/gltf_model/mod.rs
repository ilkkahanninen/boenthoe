mod data;
mod node;
mod primitive;

use super::{Model, ModelRenderContext};
use crate::engine::{shaders, Asset, Engine, EngineError};
use node::Node;
use std::path::Path;

pub type Matrix4 = cgmath::Matrix4<f32>;

pub struct GltfModel {
    nodes: Vec<Node>,
}

impl Model for GltfModel {
    fn render(&self, context: &mut ModelRenderContext) {
        let transform = cgmath::SquareMatrix::identity();
        for node in self.nodes.iter() {
            node.render(context, &transform);
        }
    }
}

impl GltfModel {
    pub fn new(engine: &Engine, source: &Asset) -> Result<Self, EngineError> {
        let (gltf, buffers, images) = gltf::import_slice(source.data()?)
            .or_else(|error| Err(EngineError::parse_error(source, error)))?;

        let initData = data::InitData::load(&engine.device, &buffers)?;

        let scene = gltf
            .default_scene()
            .or_else(|| gltf.scenes().next())
            .ok_or_else(|| EngineError::parse_error(source, "The file does not have any scenes"))?;

        Ok(GltfModel {
            nodes: scene
                .nodes()
                .map(|node| Node::new(engine, &node, &initData))
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
