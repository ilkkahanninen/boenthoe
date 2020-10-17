use crate::engine::{assets::Asset, shaders, EngineError};

pub struct InitData<'a> {
    pub buffers: &'a Vec<gltf::buffer::Data>,
    pub vertex_shader: wgpu::ShaderModule,
    pub fragment_shader: wgpu::ShaderModule,
}

impl<'a> InitData<'a> {
    pub fn load(
        device: &wgpu::Device,
        buffers: &'a Vec<gltf::buffer::Data>,
    ) -> Result<Self, EngineError> {
        let vertex_shader = shaders::build(
            device,
            &Asset::Ready {
                path: "engine/model/gltf_model/shaders/gltf.vert".into(),
                data: include_bytes!("shaders/gltf.vert").to_vec(),
            },
        )?;

        let fragment_shader = shaders::build(
            device,
            &Asset::Ready {
                path: "engine/model/gltf_model/shaders/gltf.frag".into(),
                data: include_bytes!("shaders/gltf.frag").to_vec(),
            },
        )?;

        Ok(Self {
            buffers,
            vertex_shader,
            fragment_shader,
        })
    }
}
