use crate::engine::*;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniforms {
  view_proj: cgmath::Matrix4<f32>,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

#[allow(dead_code)]
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
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStage::VERTEX,
        ty: wgpu::BindingType::UniformBuffer {
          dynamic: false,
          min_binding_size: None,
        },
        count: None,
      }],
      label: Some("uniform_bind_group_layout"),
    })
  }

  pub fn create_bind_group(self, device: &wgpu::Device) -> wgpu::BindGroup {
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      contents: bytemuck::cast_slice(&[self]),
      usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
      label: Some("uniform_buffer"),
    });

    device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &Self::create_bind_group_layout(device),
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: wgpu::BindingResource::Buffer(buffer.slice(..)),
      }],
      label: Some("uniform_bind_group"),
    })
  }
}
