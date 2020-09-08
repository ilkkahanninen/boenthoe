use crate::engine::object::Object;
use crate::engine::*;
use image::GenericImageView;
use wgpu::util::DeviceExt;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub struct Texture {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub sampler: wgpu::Sampler,
    pub view: wgpu::TextureView,
}

impl Object for Texture {
    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

pub struct TextureBuilder<'a, T> {
    pub engine: &'a engine::Engine<T>,
    pub command_buffers: Vec<wgpu::CommandBuffer>,
}

impl<'a, T> TextureBuilder<'a, T> {
    pub fn new(engine: &'a engine::Engine<T>) -> Self {
        TextureBuilder {
            engine,
            command_buffers: vec![],
        }
    }

    pub fn diffuse(&mut self, bytes: &[u8], label: &str) -> Texture {
        let device = &self.engine.device;

        let (rgba, dimensions) = {
            println!("Load {} from memory...", label);
            let image = image::load_from_memory(bytes).expect("Failed to load image from memory");
            let buffer_dimensions = BufferDimensions::new(&image.dimensions());
            println!("  Resize...");
            let rgba = image
                .resize_exact(
                    buffer_dimensions.padded_width,
                    image.dimensions().1,
                    image::imageops::FilterType::Triangle,
                )
                .into_rgba();
            println!("  Done");
            let dimensions = rgba.dimensions();
            (rgba, dimensions)
        };

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: Some(label),
        });

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: &rgba,
            usage: wgpu::BufferUsage::COPY_SRC,
            label: Some("texture_diffuse_buffer"),
        });

        // Copy buffer to texture

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("texture_buffer_copy_encoder"),
        });
        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                layout: wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: 4 * dimensions.0,
                    rows_per_image: dimensions.1,
                },
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );
        self.command_buffers.push(encoder.finish());

        // Create view and sampler

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: None,
            anisotropy_clamp: None,
            label: Some("texture_sampler"),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            ],
            label: Some("texture_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("bind_group"),
        });

        Texture {
            bind_group_layout,
            bind_group,
            sampler,
            view,
        }
    }

    pub fn depth_stencil_buffer(&self, label: &str) -> Texture {
        let device = &self.engine.device;
        let sc_desc = &self.engine.swap_chain_descriptor;
        let size = wgpu::Extent3d {
            width: sc_desc.width,
            height: sc_desc.height,
            depth: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT
                | wgpu::TextureUsage::SAMPLED
                | wgpu::TextureUsage::COPY_SRC,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: Some(wgpu::CompareFunction::LessEqual),
            anisotropy_clamp: None,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                    ty: wgpu::BindingType::Sampler { comparison: true },
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("bind_group"),
        });

        Texture {
            bind_group_layout,
            bind_group,
            view,
            sampler,
        }
    }
}

#[derive(Debug)]
struct BufferDimensions {
    width: u32,
    height: u32,
    unpadded_bytes_per_row: u32,
    padded_bytes_per_row: u32,
    padded_width: u32,
}

impl BufferDimensions {
    fn new(dimensions: &(u32, u32)) -> Self {
        let bytes_per_pixel = std::mem::size_of::<u32>() as u32;
        let unpadded_bytes_per_row = dimensions.0 * (bytes_per_pixel);
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        let padded_width = padded_bytes_per_row / bytes_per_pixel;
        Self {
            width: dimensions.0,
            height: dimensions.1,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
            padded_width,
        }
    }
}
