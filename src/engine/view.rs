//use crate::engine::*;
use crate::engine::{camera::Camera, object::Object};
use wgpu::util::DeviceExt;

#[derive(Debug, Copy, Clone)]
pub struct ViewModel {
    pub camera: Camera,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ViewUniform {
    view_position: cgmath::Vector4<f32>,
    view_proj: cgmath::Matrix4<f32>,
}

impl From<&ViewModel> for ViewUniform {
    fn from(model: &ViewModel) -> Self {
        Self {
            view_position: model.camera.eye.to_homogeneous(),
            view_proj: model.camera.build_view_projection_matrix(),
        }
    }
}

unsafe impl bytemuck::Pod for ViewUniform {}
unsafe impl bytemuck::Zeroable for ViewUniform {}

pub struct ViewObject {
    pub model: ViewModel,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

#[allow(dead_code)]
impl ViewObject {
    pub fn new(device: &wgpu::Device, model: ViewModel) -> Self {
        let buffer = Self::create_buffer(device, &model, true);

        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<ViewUniform>() as _
                        ),
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.slice(..)),
            }],
            label: Some("uniform_bind_group"),
        });

        Self {
            model,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn update(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let buffer = Self::create_buffer(device, &self.model, false);
        encoder.copy_buffer_to_buffer(
            &buffer,
            0,
            &self.buffer,
            0,
            std::mem::size_of::<ViewUniform>() as wgpu::BufferAddress,
        );
    }

    fn create_buffer(
        device: &wgpu::Device,
        model: &ViewModel,
        is_destination: bool,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::cast_slice(&[ViewUniform::from(model)]),
            usage: wgpu::BufferUsage::UNIFORM
                | if is_destination {
                    wgpu::BufferUsage::COPY_DST
                } else {
                    wgpu::BufferUsage::COPY_SRC
                },
            label: None,
        })
    }
}

impl Object for ViewObject {
    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
