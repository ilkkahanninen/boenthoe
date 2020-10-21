mod data;
mod node;
mod primitive;

use super::{Model, ModelProperties, ModelRenderContext};
use crate::engine::{camera::Camera, prelude::*, shaders};
use node::Node;
use std::path::Path;

pub struct GltfModel {
    nodes: Vec<Node>,
    lights: Vec<Light>,
    lights_buffer: StorageBuffer<LightBufferObject>,
    camera: Camera,
}

pub struct ModelRenderData<'a> {
    view_projection_matrix: &'a Matrix4,
    eye_position: &'a Point3,
    model_matrix: &'a Matrix4,
    lights: &'a wgpu::BindGroup,
    number_of_lights: u32,
}

impl Model for GltfModel {
    fn render(&self, context: &mut ModelRenderContext) {
        let light_buffer_objects: Vec<LightBufferObject> = self
            .lights
            .iter()
            .filter(|light| light.is_lit())
            .map(LightBufferObject::from)
            .collect();

        if light_buffer_objects.len() > 0 {
            self.lights_buffer
                .copy_to_gpu(context.device, context.encoder, &light_buffer_objects);
        }

        let data = ModelRenderData {
            view_projection_matrix: &self.camera.view_projection_matrix(),
            eye_position: &self.camera.eye,
            model_matrix: &cgmath::SquareMatrix::identity(),
            lights: self.lights_buffer.get_bind_group(),
            number_of_lights: light_buffer_objects.len() as u32,
        };

        for node in self.nodes.iter() {
            node.render(context, &data);
        }
    }

    fn set_camera(&mut self, camera: &Camera) {
        self.camera = camera.clone();
    }

    fn set_lighting(&mut self, lights: &[Light]) {
        self.lights = lights.to_vec();
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

        let camera = options.camera.unwrap_or(Camera::default());

        Ok(GltfModel {
            nodes: scene
                .nodes()
                .map(|node| Node::new(engine, &node, &data))
                .collect(),
            lights: Vec::new(),
            lights_buffer: StorageBuffer::new(&engine.device, 16, "gltf::Lights"),
            camera,
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
