use super::{texture::GltfTexture, ModelProperties};
use crate::engine::prelude::*;

pub struct InitData<'a> {
    pub buffers: &'a Vec<gltf::buffer::Data>,
    pub textures: Vec<GltfTexture>,
    pub vertex_shader: wgpu::ShaderModule,
    pub fragment_shader: wgpu::ShaderModule,
}

impl<'a> InitData<'a> {
    pub fn load(
        engine: &Engine,
        buffers: &'a Vec<gltf::buffer::Data>,
        images: &'a Vec<gltf::image::Data>,
        _options: &ModelProperties,
    ) -> Result<Self, EngineError> {
        engine.add_asset(
            Path::new("gltf_model/shaders/uniforms.glsl"),
            include_bytes!("shaders/uniforms.glsl"),
        );

        let phong_frag = engine.add_asset(
            Path::new("gltf_model/shaders/gltf.frag"),
            include_bytes!("shaders/phong.frag"),
        );

        let vertex_shader = shaders::build(
            engine,
            &engine.add_asset(
                Path::new("gltf_model/shaders/gltf.vert"),
                include_bytes!("shaders/gltf.vert"),
            ),
        )?;

        let fragment_shader = shaders::build(engine, &phong_frag)?;

        let textures = images
            .iter()
            .map(|image| GltfTexture::build(engine, image))
            .collect();

        Ok(Self {
            buffers,
            textures,
            vertex_shader,
            fragment_shader,
        })
    }
}
