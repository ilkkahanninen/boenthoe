use crate::engine::object::Object;
use wgpu::util::DeviceExt;

#[derive(Debug, Copy, Clone)]
pub struct LightModel {
    pub position: cgmath::Vector3<f32>,
    pub color: cgmath::Vector3<f32>,
}

impl LightModel {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: cgmath::Vector3::new(x, y, z),
            color: cgmath::Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Default for LightModel {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LightUniform {
    pub position: cgmath::Vector3<f32>,
    pub _padding: u32,
    pub color: cgmath::Vector3<f32>,
}

unsafe impl bytemuck::Zeroable for LightUniform {}
unsafe impl bytemuck::Pod for LightUniform {}

impl From<&LightModel> for LightUniform {
    fn from(light: &LightModel) -> Self {
        Self {
            position: light.position,
            color: light.color,
            _padding: 0,
        }
    }
}

pub struct LightObject {
    pub model: LightModel,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl LightObject {
    // TODO: Use vector of light models instead of one
    pub fn new(device: &wgpu::Device, model: LightModel) -> Self {
        let buffer = Self::create_buffer(device, &model, true);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.slice(..)),
            }],
            label: None,
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
            std::mem::size_of::<LightUniform>() as wgpu::BufferAddress,
        );
    }

    fn create_buffer(
        device: &wgpu::Device,
        model: &LightModel,
        is_destination: bool,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[LightUniform::from(model)]),
            usage: wgpu::BufferUsage::UNIFORM
                | if is_destination {
                    wgpu::BufferUsage::COPY_DST
                } else {
                    wgpu::BufferUsage::COPY_SRC
                },
        })
    }
}

impl Object for LightObject {
    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
