use crate::engine::prelude::*;
use wgpu::util::DeviceExt;

pub struct UniformBuffer<T> {
    buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    label: String,
    phantom: std::marker::PhantomData<T>,
}

impl<T> UniformBuffer<T>
where
    T: bytemuck::Pod,
{
    pub fn new(device: &wgpu::Device, label: &str) -> Self {
        Self::init(device, T::zeroed(), label)
    }

    pub fn default(device: &wgpu::Device, label: &str) -> Self
    where
        T: Default,
    {
        Self::init(device, T::default(), label)
    }

    pub fn init(device: &wgpu::Device, initial_data: T, label: &str) -> Self {
        let buffer = Self::create_buffer(device, &initial_data, label);

        let bind_group_layout = Self::create_layout(device, label);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.slice(..)),
            }],
        });

        Self {
            buffer,
            bind_group_layout,
            bind_group,
            label: label.into(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn copy_to_gpu(&self, queue: &wgpu::Queue, data: &T) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[*data]));
    }

    pub fn create_layout(device: &wgpu::Device, label: &str) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(label),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    fn create_buffer(device: &wgpu::Device, data: &T, label: &str) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::cast_slice(&[*data]),
            usage: wgpu::BufferUsage::UNIFORM
                | wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::COPY_SRC,
            label: Some(label),
        })
    }
}

impl<T> Object for UniformBuffer<T> {
    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}

pub struct StorageBuffer<T> {
    label: String,
    buffer: wgpu::Buffer,
    staging_buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    phantom: std::marker::PhantomData<T>,
}

impl<T> StorageBuffer<T>
where
    T: bytemuck::Pod,
{
    pub fn new(device: &wgpu::Device, item_count: usize, label: &str) -> Self {
        Self::init(device, vec![T::zeroed(); item_count], label)
    }

    pub fn default(device: &wgpu::Device, item_count: usize, label: &str) -> Self
    where
        T: Default,
    {
        Self::init(device, vec![T::default(); item_count], label)
    }

    pub fn init(device: &wgpu::Device, initial_data: Vec<T>, label: &str) -> Self {
        let buffer = Self::create_buffer(device, &initial_data, true, label);
        let staging_buffer = Self::create_buffer(device, &initial_data, false, label);

        let bind_group_layout = Self::create_layout(device, label);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.slice(..)),
            }],
        });

        Self {
            label: label.into(),
            buffer,
            staging_buffer,
            bind_group_layout,
            bind_group,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn copy_to_gpu(&self, encoder: &mut wgpu::CommandEncoder, queue: &wgpu::Queue, data: &[T]) {
        queue.write_buffer(&self.staging_buffer, 0, bytemuck::cast_slice(&data));
        encoder.copy_buffer_to_buffer(
            &self.staging_buffer,
            0,
            &self.buffer,
            0,
            (data.len() * std::mem::size_of::<T>()) as wgpu::BufferAddress,
        );
    }

    pub fn create_layout(device: &wgpu::Device, label: &str) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(label),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::StorageBuffer {
                    dynamic: false,
                    readonly: true,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    fn create_buffer(
        device: &wgpu::Device,
        data: &[T],
        is_destination: bool,
        label: &str,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::cast_slice(&data),
            usage: wgpu::BufferUsage::STORAGE
                | if is_destination {
                    wgpu::BufferUsage::COPY_DST
                } else {
                    wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC
                },
            label: Some(label),
        })
    }
}

impl<T> Object for StorageBuffer<T> {
    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}
