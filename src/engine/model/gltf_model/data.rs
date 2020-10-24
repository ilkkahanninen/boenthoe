use super::{texture::GltfTexture, ModelProperties};
use crate::engine::prelude::*;

pub struct InitData<'a> {
    pub buffers: &'a Vec<gltf::buffer::Data>,
    pub images: &'a Vec<gltf::image::Data>,
    pub textures_bind_group_layout: wgpu::BindGroupLayout,
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

        let fragment_shader = engine.add_asset(
            Path::new("gltf_model/shaders/gltf.frag"),
            include_bytes!("shaders/gltf.frag"),
        );

        let vertex_shader = shaders::build(
            engine,
            &engine.add_asset(
                Path::new("gltf_model/shaders/gltf.vert"),
                include_bytes!("shaders/gltf.vert"),
            ),
        )?;

        let fragment_shader = shaders::build(engine, &fragment_shader)?;

        let textures_bind_group_layout =
            engine
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: wgpu::TextureViewDimension::D2,
                                component_type: wgpu::TextureComponentType::Uint,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Sampler { comparison: false },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: wgpu::TextureViewDimension::D2,
                                component_type: wgpu::TextureComponentType::Uint,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Sampler { comparison: false },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 4,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: wgpu::TextureViewDimension::D2,
                                component_type: wgpu::TextureComponentType::Uint,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 5,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Sampler { comparison: false },
                            count: None,
                        },
                    ],
                });

        Ok(Self {
            buffers,
            images,
            textures_bind_group_layout,
            vertex_shader,
            fragment_shader,
        })
    }

    pub fn create_texture_bind_group(
        &self,
        engine: &Engine,
        material: &gltf::material::Material,
    ) -> wgpu::BindGroup {
        let (base_color_texture, base_color_sampler) = self.build_texture_and_sampler(
            engine,
            material
                .pbr_metallic_roughness()
                .base_color_texture()
                .map(|info| info.texture()),
            &TextureSpec::base_color(),
        );

        let (normal_map_texture, normal_map_sampler) = self.build_texture_and_sampler(
            engine,
            material
                .normal_texture()
                .map(|normal_texture| normal_texture.texture()),
            &TextureSpec::normal_map(),
        );

        let (emissive_texture, emissive_sampler) = self.build_texture_and_sampler(
            engine,
            material
                .emissive_texture()
                .map(|emissive_texture| emissive_texture.texture()),
            &TextureSpec::emissive_texture(),
        );

        engine.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.textures_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&base_color_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&base_color_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&normal_map_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&normal_map_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&emissive_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(&emissive_sampler),
                },
            ],
        })
    }

    fn build_texture_and_sampler(
        &self,
        engine: &Engine,
        gltf_texture: Option<gltf::texture::Texture>,
        texture_spec: &TextureSpec,
    ) -> (GltfTexture, wgpu::Sampler) {
        fn wrapping_mode_to_address_mode(wrap: gltf::texture::WrappingMode) -> wgpu::AddressMode {
            match wrap {
                gltf::texture::WrappingMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
                gltf::texture::WrappingMode::MirroredRepeat => wgpu::AddressMode::MirrorRepeat,
                gltf::texture::WrappingMode::Repeat => wgpu::AddressMode::Repeat,
            }
        }

        let (texture, sampler_desc) = if let Some(gltf_texture) = gltf_texture {
            let texture = GltfTexture::build(
                engine,
                self.images.get(gltf_texture.index()).unwrap(),
                texture_spec.linear_colors,
            );
            let sampler_spec = gltf_texture.sampler();
            let sampler = wgpu::SamplerDescriptor {
                address_mode_u: wrapping_mode_to_address_mode(sampler_spec.wrap_s()),
                address_mode_v: wrapping_mode_to_address_mode(sampler_spec.wrap_t()),
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: match sampler_spec.mag_filter() {
                    Some(gltf::texture::MagFilter::Nearest) => wgpu::FilterMode::Nearest,
                    _ => wgpu::FilterMode::Linear,
                },
                min_filter: match sampler_spec.mag_filter() {
                    Some(gltf::texture::MagFilter::Linear) => wgpu::FilterMode::Linear,
                    _ => wgpu::FilterMode::Nearest,
                },
                mipmap_filter: wgpu::FilterMode::Nearest,
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                compare: None,
                anisotropy_clamp: None,
                label: None,
            };

            (texture, sampler)
        } else {
            let texture = GltfTexture::build_solid(
                engine,
                &texture_spec.default_data,
                texture_spec.linear_colors,
            );
            let sampler = wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                compare: None,
                anisotropy_clamp: None,
                label: None,
            };

            (texture, sampler)
        };

        (texture, engine.device.create_sampler(&sampler_desc))
    }
}

struct TextureSpec {
    /// Linear colors are used by data textures e.g. normal maps. Leave false for sRGB textures.
    linear_colors: bool,
    /// If model is missing the required texture, generate a new texture using `default_data` as pixel value.
    default_data: [u8; 4],
}

impl TextureSpec {
    fn base_color() -> Self {
        Self {
            linear_colors: false,
            default_data: [0xff, 0xff, 0xff, 0xff],
        }
    }

    fn normal_map() -> Self {
        Self {
            linear_colors: true,
            default_data: [0x00, 0x00, 0xff, 0x00],
        }
    }

    fn emissive_texture() -> Self {
        Self {
            linear_colors: false,
            default_data: [0x00, 0x00, 0x00, 0x00],
        }
    }
}
