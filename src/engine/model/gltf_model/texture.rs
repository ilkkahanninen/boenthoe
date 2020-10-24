use crate::engine::prelude::*;
use gltf::image::Format;
use wgpu::{util::DeviceExt, TextureFormat};

pub struct GltfTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl GltfTexture {
    pub fn build_solid(engine: &Engine, data: &[u8; 4], linear_colors: bool) -> Self {
        let size = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let mut pixels = vec![0; size];
        for i in 0..size {
            pixels[i] = data[i % 4];
        }

        Self::build(
            engine,
            &gltf::image::Data {
                format: gltf::image::Format::R8G8B8A8,
                pixels,
                width: size as u32 / 4,
                height: 1,
            },
            linear_colors,
        )
    }

    pub fn build(engine: &Engine, data: &gltf::image::Data, linear_colors: bool) -> Self {
        let format = FormatDescriptor::from(data.format, linear_colors);

        let pixels_with_padding = if format.source_size != format.target_size {
            Some(pad_data(
                &data.pixels,
                format.source_size,
                format.target_size,
            ))
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
                contents: pixels_with_padding.as_ref().unwrap_or(&data.pixels),
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

impl FormatDescriptor {
    fn from(format: Format, linear_colors: bool) -> Self {
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
                wgpu_format: if linear_colors {
                    TextureFormat::Rgba8Unorm
                } else {
                    TextureFormat::Rgba8UnormSrgb
                },
                source_size: 3,
                target_size: 4,
            },
            Format::R8G8B8A8 => Self {
                wgpu_format: if linear_colors {
                    TextureFormat::Rgba8Unorm
                } else {
                    TextureFormat::Rgba8UnormSrgb
                },
                source_size: 4,
                target_size: 4,
            },
            Format::B8G8R8 | Format::B8G8R8A8 => Self {
                wgpu_format: if linear_colors {
                    TextureFormat::Bgra8Unorm
                } else {
                    TextureFormat::Bgra8UnormSrgb
                },
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

fn pad_data(data: &[u8], source_item_size: usize, target_item_size: usize) -> Vec<u8> {
    assert!(source_item_size < target_item_size);
    let item_count = data.len() / source_item_size;
    let result = vec![0xff; item_count * target_item_size].into_raw_parts();
    let mut i = 0;
    while i < item_count {
        let sp = i * source_item_size;
        let tp = i * target_item_size;
        i += 1;
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data[sp],
                result.0.offset(tp as isize),
                source_item_size,
            );
        }
    }
    unsafe { Vec::from_raw_parts(result.0, result.1, result.2) }
}
