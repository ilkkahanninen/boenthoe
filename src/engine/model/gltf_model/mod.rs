mod data;
mod node;
mod primitive;
mod texture;

use super::{Model, ModelProperties, ModelRenderContext, RenderingMode};
use crate::engine::{camera::Camera, prelude::*};
use node::Node;

pub const MAX_NUMBER_OF_LIGHTS: usize = 8;
pub type LightsArray = [LightBufferObject; MAX_NUMBER_OF_LIGHTS];
pub struct GltfModel {
    nodes: Vec<Node>,
    lights: Vec<Light>,
    lights_buffer: UniformBuffer<LightsArray>,
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
        let mut light_buffer_objects = [LightBufferObject::default(); MAX_NUMBER_OF_LIGHTS];
        let visible_lights: Vec<&Light> = self
            .lights
            .iter()
            .filter(|light| light.is_lit())
            .take(MAX_NUMBER_OF_LIGHTS)
            .collect();

        for (index, light) in visible_lights.iter().enumerate() {
            light_buffer_objects[index] = LightBufferObject::from(*light)
        }

        if light_buffer_objects.len() > 0 {
            self.lights_buffer
                .copy_to_gpu(context.queue, &light_buffer_objects);
        }

        let data = ModelRenderData {
            view_projection_matrix: &self.camera.view_projection_matrix(),
            eye_position: &self.camera.eye,
            model_matrix: &cgmath::SquareMatrix::identity(),
            lights: self.lights_buffer.get_bind_group(),
            number_of_lights: visible_lights.len() as u32,
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
        let (gltf, buffers, images) = gltf::import_slice(source.data()?)
            .or_else(|error| Err(EngineError::parse_error(source, error)))?;

        let data = data::InitData::load(engine, &buffers, &images, options)?;

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
            lights_buffer: UniformBuffer::new(&engine.device, "gltf::Lights"),
            camera,
        })
    }
}
