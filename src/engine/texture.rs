use crate::engine::*;
use image::GenericImageView;
use wgpu::util::DeviceExt;

pub struct PipelineTexture {
  pub bind_group_layout: wgpu::BindGroupLayout,
  pub command_buffer: wgpu::CommandBuffer,
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

  pub fn diffuse(&mut self, bytes: &[u8], label: &str) -> wgpu::BindGroup {
    let image = image::load_from_memory(bytes).unwrap();
    let rgba = image.as_rgba8().unwrap();
    let dimensions = image.dimensions();

    let size = wgpu::Extent3d {
      width: dimensions.0,
      height: dimensions.1,
      depth: 1,
    };

    let texture = self.engine.device.create_texture(&wgpu::TextureDescriptor {
      size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
      label: Some(label),
    });

    let buffer = self
      .engine
      .device
      .create_buffer_init(&wgpu::util::BufferInitDescriptor {
        contents: &rgba,
        usage: wgpu::BufferUsage::COPY_SRC,
        label: Some("texture_diffuse_buffer"),
      });

    let mut encoder = self
      .engine
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

    let diffuse_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let diffuse_sampler = self.engine.device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::FilterMode::Nearest,
      lod_min_clamp: -100.0,
      lod_max_clamp: 100.0,
      compare: Some(wgpu::CompareFunction::Always),
      anisotropy_clamp: None,
      label: Some("texture_diffuse_sampler"),
    });

    let diffuse_bind_group = self
      .engine
      .device
      .create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &self.diffuse_bind_group_layout(),
        entries: &[
          wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
          },
          wgpu::BindGroupEntry {
            binding: 1,
            resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
          },
        ],
        label: Some("diffuse_bind_group"),
      });

    self.command_buffers.push(encoder.finish());

    diffuse_bind_group
  }

  pub fn diffuse_bind_group_layout(&self) -> wgpu::BindGroupLayout {
    self
      .engine
      .device
      .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
      })
  }
}
