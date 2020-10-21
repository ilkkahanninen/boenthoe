use crate::engine::prelude::*;
use gltf::image::Format;
use wgpu::{util::DeviceExt, TextureFormat};

pub struct GltfTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl GltfTexture {
    pub fn build(engine: &Engine, data: &gltf::image::Data) -> Self {
        let format = FormatDescriptor::from(data.format);

        let realigned_pixels = if format.source_size != format.target_size {
            let mut pixels =
                Vec::with_capacity(data.pixels.len() / format.source_size * format.target_size);
            let pad_size = format.target_size - format.source_size;
            let pad_at = format.source_size - 1;
            for (index, pixel) in data.pixels.iter().enumerate() {
                pixels.push(*pixel);
                if index % format.source_size == pad_at {
                    for _ in 0..pad_size {
                        pixels.push(0xff);
                    }
                }
            }
            Some(pixels)
        } else {
            None
        };

        let texture_descriptor = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: data.width,
                height: data.height,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: format.wgpu_format,
            usage: wgpu::TextureUsage::SAMPLED
                | wgpu::TextureUsage::COPY_DST
                | wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            label: Some("gltf::texture::Texture"),
        };

        let texture = engine.device.create_texture(&texture_descriptor);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let buffer = engine
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                contents: realigned_pixels.as_ref().unwrap_or(&data.pixels),
                usage: wgpu::BufferUsage::COPY_SRC,
                label: Some("gltf::texture::Buffer"),
            });

        let mut encoder = engine
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                layout: wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: format.target_size as u32 * data.width,
                    rows_per_image: data.height,
                },
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            texture_descriptor.size,
        );

        engine.add_command_buffer(encoder.finish());

        Self { texture, view }
    }
}
struct FormatDescriptor {
    wgpu_format: wgpu::TextureFormat,
    source_size: usize,
    target_size: usize,
}

impl From<Format> for FormatDescriptor {
    fn from(format: Format) -> Self {
        match format {
            Format::R8 => Self {
                wgpu_format: TextureFormat::R8Unorm,
                source_size: 1,
                target_size: 1,
            },
            Format::R8G8 => Self {
                wgpu_format: TextureFormat::Rg8Unorm,
                source_size: 2,
                target_size: 2,
            },
            Format::R8G8B8 => Self {
                wgpu_format: TextureFormat::Rgba8UnormSrgb,
                source_size: 3,
                target_size: 4,
            },
            Format::R8G8B8A8 => Self {
                wgpu_format: TextureFormat::Rgba8UnormSrgb,
                source_size: 4,
                target_size: 4,
            },
            Format::B8G8R8 | Format::B8G8R8A8 => Self {
                wgpu_format: TextureFormat::Bgra8UnormSrgb,
                source_size: 4,
                target_size: 4,
            },
            Format::R16 => Self {
                wgpu_format: TextureFormat::R16Float,
                source_size: 2,
                target_size: 2,
            },
            Format::R16G16 => Self {
                wgpu_format: TextureFormat::Rg16Float,
                source_size: 4,
                target_size: 4,
            },
            Format::R16G16B16 => Self {
                wgpu_format: TextureFormat::Rgba16Float,
                source_size: 6,
                target_size: 8,
            },
            Format::R16G16B16A16 => Self {
                wgpu_format: TextureFormat::Rgba16Float,
                source_size: 8,
                target_size: 8,
            },
        }
    }
}
