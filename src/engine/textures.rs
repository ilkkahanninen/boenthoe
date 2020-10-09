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

pub fn diffuse(engine: &engine::Engine, asset: &assets::Asset) -> Result<Texture, String> {
    let texture = create_rgba_texture(engine, load_image(asset)?);
    let device = &engine.device;
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&default_sampler_descriptor());
    let bind_group_layout =
        device.create_bind_group_layout(&default_bind_group_layout_descriptor());
    let bind_group = create_bind_group(device, &bind_group_layout, &view, &sampler);

    Ok(Texture {
        bind_group_layout,
        bind_group,
        sampler,
        view,
    })
}

pub fn color_buffer(engine: &engine::Engine) -> Texture {
    buffer(engine, engine.swap_chain_descriptor.format)
}

pub fn depth_buffer(engine: &engine::Engine) -> Texture {
    buffer(engine, DEPTH_FORMAT)
}

pub fn buffer(engine: &engine::Engine, format: wgpu::TextureFormat) -> Texture {
    let device = &engine.device;
    let swap_chain_descriptor = &engine.swap_chain_descriptor;

    let texture = device.create_texture(&default_texture_descriptor(
        swap_chain_descriptor.width,
        swap_chain_descriptor.height,
        format,
    ));

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&default_sampler_descriptor());
    let bind_group_layout =
        device.create_bind_group_layout(&default_bind_group_layout_descriptor());
    let bind_group = create_bind_group(device, &bind_group_layout, &view, &sampler);

    Texture {
        bind_group_layout,
        bind_group,
        view,
        sampler,
    }
}

fn load_image(asset: &assets::Asset) -> Result<image::DynamicImage, String> {
    let mut image =
        image::load_from_memory_with_format(asset.get_data()?, get_image_format(asset)?)
            .or_else(|err| Err(format!("Loading asset {:?} failed: {:?}", asset, err)))?;

    let dimensions = image.dimensions();
    if let Some(target_width) = width_resize_requirement(dimensions.0) {
        image = image.resize_exact(
            target_width,
            dimensions.1,
            image::imageops::FilterType::Triangle,
        );
    }

    Ok(image)
}

fn create_rgba_texture(engine: &engine::Engine, image: image::DynamicImage) -> wgpu::Texture {
    let device = &engine.device;

    let descriptor = default_texture_descriptor(
        image.width(),
        image.height(),
        wgpu::TextureFormat::Rgba8UnormSrgb,
    );
    let rgba = image.into_rgba();
    let texture = device.create_texture(&descriptor);

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        contents: &rgba,
        usage: wgpu::BufferUsage::COPY_SRC,
        label: None,
    });

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    encoder.copy_buffer_to_texture(
        wgpu::BufferCopyView {
            buffer: &buffer,
            layout: wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * rgba.width(),
                rows_per_image: rgba.height(),
            },
        },
        wgpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        descriptor.size,
    );

    engine.add_command_buffer(encoder.finish());

    texture
}

fn get_image_format(asset: &assets::Asset) -> Result<image::ImageFormat, String> {
    match asset.get_type() {
        assets::AssetType::PngImage => Ok(image::ImageFormat::Png),
        _ => return Err(format!("Unsupported image type: {:?}", asset)),
    }
}

fn width_resize_requirement(source_width: u32) -> Option<u32> {
    let bytes_per_pixel = std::mem::size_of::<u32>() as u32;
    let unpadded_bytes_per_row = source_width * bytes_per_pixel;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
    if padded_bytes_per_row_padding > 0 {
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        Some(padded_bytes_per_row / bytes_per_pixel)
    } else {
        None
    }
}

fn create_bind_group(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
    texture_view: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(sampler),
            },
        ],
        label: None,
    })
}

fn default_texture_descriptor<'a>(
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
) -> wgpu::TextureDescriptor<'a> {
    let size = wgpu::Extent3d {
        width,
        height,
        depth: 1,
    };

    wgpu::TextureDescriptor {
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        label: None,
    }
}

fn default_sampler_descriptor<'a>() -> wgpu::SamplerDescriptor<'a> {
    wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare: None,
        anisotropy_clamp: None,
        label: None,
    }
}

fn default_bind_group_layout_descriptor<'a>() -> wgpu::BindGroupLayoutDescriptor<'a> {
    wgpu::BindGroupLayoutDescriptor {
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
        label: None,
    }
}
