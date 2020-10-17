use crate::engine::object::Object;
use wgpu::util::DeviceExt;

pub struct StorageObject<T> {
    buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    phantom: std::marker::PhantomData<T>,
}

impl<T> StorageObject<T>
where
    T: bytemuck::Pod,
{
    pub fn new(device: &wgpu::Device) -> Self {
        Self::init(device, T::zeroed())
    }

    pub fn default(device: &wgpu::Device) -> Self
    where
        T: Default,
    {
        Self::init(device, T::default())
    }

    pub fn init(device: &wgpu::Device, initial_data: T) -> Self {
        let buffer = Self::create_buffer(device, &initial_data, true);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
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
            buffer,
            bind_group_layout,
            bind_group,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn copy_to_gpu(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, data: &T) {
        let staging_buffer = Self::create_buffer(device, data, false);
        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.buffer,
            0,
            std::mem::size_of::<T>() as wgpu::BufferAddress,
        );
    }

    fn create_buffer(device: &wgpu::Device, data: &T, is_destination: bool) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::cast_slice(&[*data]),
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

impl<T> Object for StorageObject<T> {
    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}

pub struct StorageVecObject<T> {
    pub data: Vec<T>,
    buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl<T> StorageVecObject<T>
where
    T: bytemuck::Pod,
{
    pub fn new(device: &wgpu::Device, item_count: usize) -> Self {
        Self::init(device, vec![T::zeroed(); item_count])
    }

    pub fn default(device: &wgpu::Device, item_count: usize) -> Self
    where
        T: Default,
    {
        Self::init(device, vec![T::default(); item_count])
    }

    pub fn init(device: &wgpu::Device, initial_data: Vec<T>) -> Self {
        let buffer = Self::create_buffer(device, &initial_data, true);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
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
            data: initial_data,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn copy_to_gpu(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let staging_buffer = Self::create_buffer(device, &self.data, false);
        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.buffer,
            0,
            (self.data.len() * std::mem::size_of::<T>()) as wgpu::BufferAddress,
        );
    }

    pub fn all(&self) -> std::ops::Range<u32> {
        0..(self.data.len() as u32)
    }

    fn create_buffer(device: &wgpu::Device, data: &Vec<T>, is_destination: bool) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::cast_slice(&data),
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

impl<T> Object for StorageVecObject<T> {
    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}
