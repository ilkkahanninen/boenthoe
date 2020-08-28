use crate::engine::state;
use image::GenericImageView;

pub struct PipelineTexture {
  pub bind_group_layout: wgpu::BindGroupLayout,
  pub command_buffer: wgpu::CommandBuffer,
}

pub struct TextureBuilder<'a> {
  pub state: &'a mut state::State,
  pub pipeline_textures: Vec<PipelineTexture>,
}

impl<'a> TextureBuilder<'a> {
  pub fn new(state: &'a mut state::State) -> Self {
    TextureBuilder {
      state,
      pipeline_textures: vec![],
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

    let texture = self.state.device.create_texture(&wgpu::TextureDescriptor {
      size,
      array_layer_count: 1,
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
      label: Some(label),
    });

    let buffer = self
      .state
      .device
      .create_buffer_with_data(&rgba, wgpu::BufferUsage::COPY_SRC);

    let mut encoder = self
      .state
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("texture_buffer_copy_encoder"),
      });

    encoder.copy_buffer_to_texture(
      wgpu::BufferCopyView {
        buffer: &buffer,
        offset: 0,
        bytes_per_row: 4 * dimensions.0,
        rows_per_image: dimensions.1,
      },
      wgpu::TextureCopyView {
        texture: &texture,
        mip_level: 0,
        array_layer: 0,
        origin: wgpu::Origin3d::ZERO,
      },
      size,
    );

    let diffuse_texture_view = texture.create_default_view();

    let diffuse_sampler = self.state.device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::FilterMode::Nearest,
      lod_min_clamp: -100.0,
      lod_max_clamp: 100.0,
      compare: wgpu::CompareFunction::Always,
    });

    let texture_bind_group_layout =
      self
        .state
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
          bindings: &[
            wgpu::BindGroupLayoutEntry {
              binding: 0,
              visibility: wgpu::ShaderStage::FRAGMENT,
              ty: wgpu::BindingType::SampledTexture {
                multisampled: false,
                dimension: wgpu::TextureViewDimension::D2,
                component_type: wgpu::TextureComponentType::Uint,
              },
            },
            wgpu::BindGroupLayoutEntry {
              binding: 1,
              visibility: wgpu::ShaderStage::FRAGMENT,
              ty: wgpu::BindingType::Sampler { comparison: false },
            },
          ],
          label: Some("texture_bind_group_layout"),
        });

    let diffuse_bind_group = self
      .state
      .device
      .create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        bindings: &[
          wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
          },
          wgpu::Binding {
            binding: 1,
            resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
          },
        ],
        label: Some("diffuse_bind_group"),
      });

    self.pipeline_textures.push(PipelineTexture {
      bind_group_layout: texture_bind_group_layout,
      command_buffer: encoder.finish(),
    });

    diffuse_bind_group
  }

  pub fn finish(self) -> Vec<PipelineTexture> {
    self.pipeline_textures
  }
}
