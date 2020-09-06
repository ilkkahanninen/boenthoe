use crate::engine::{object::Object, transform::Transform};
use wgpu::util::DeviceExt;

#[derive(Debug, Copy, Clone)]
pub struct InstanceModel {
    pub transform: Transform,
}

impl InstanceModel {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
        }
    }
}

unsafe impl bytemuck::Zeroable for InstanceModel {}
unsafe impl bytemuck::Pod for InstanceModel {}

pub struct InstanceListObject {
    pub models: Vec<InstanceModel>,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl InstanceListObject {
    pub fn new(device: &wgpu::Device, models: Vec<InstanceModel>) -> Self {
        let buffer = Self::create_buffer(device, &models, true);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::StorageBuffer {
                    dynamic: false,
                    readonly: true,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.slice(..)),
            }],
        });

        Self {
            models,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn update(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let buffer = Self::create_buffer(device, &self.models, false);
        encoder.copy_buffer_to_buffer(
            &buffer,
            0,
            &self.buffer,
            0,
            (self.models.len() * std::mem::size_of::<InstanceModel>()) as wgpu::BufferAddress,
        );
    }

    pub fn all(&self) -> std::ops::Range<u32> {
        0..self.models.len() as _
    }

    fn create_buffer(
        device: &wgpu::Device,
        models: &Vec<InstanceModel>,
        is_destination: bool,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::cast_slice(models),
            usage: wgpu::BufferUsage::STORAGE
                | if is_destination {
                    wgpu::BufferUsage::COPY_DST
                } else {
                    wgpu::BufferUsage::COPY_SRC
                },
            label: None,
        })
    }
}

impl Object for InstanceListObject {
    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
