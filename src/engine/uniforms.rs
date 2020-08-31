use crate::engine::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniforms {
  view_proj: cgmath::Matrix4<f32>,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

impl Uniforms {
  pub fn new() -> Self {
    use cgmath::SquareMatrix;
    Self {
      view_proj: cgmath::Matrix4::identity(),
    }
  }

  pub fn update_view_proj(&mut self, camera: &camera::Camera) {
    self.view_proj = camera.build_view_projection_matrix();
  }

  pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      bindings: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStage::VERTEX,
        ty: wgpu::BindingType::UniformBuffer { dynamic: false },
      }],
      label: Some("uniform_bind_group_layout"),
    })
  }

  pub fn create_bind_group(self, device: &wgpu::Device) -> wgpu::BindGroup {
    let buffer = device.create_buffer_with_data(
      bytemuck::cast_slice(&[self]),
      wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    );

    device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &Self::create_bind_group_layout(device),
      bindings: &[wgpu::Binding {
        binding: 0,
        resource: wgpu::BindingResource::Buffer {
          buffer: &buffer,
          range: 0..std::mem::size_of_val(&self) as wgpu::BufferAddress,
        },
      }],
      label: Some("uniform_bind_group"),
    })
  }
}
