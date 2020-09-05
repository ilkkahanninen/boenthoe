use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LightModel {
    pub position: cgmath::Vector3<f32>,
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    pub _padding: u32,
    pub color: cgmath::Vector3<f32>,
}

unsafe impl bytemuck::Zeroable for LightModel {}
unsafe impl bytemuck::Pod for LightModel {}

impl LightModel {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: cgmath::Vector3::new(x, y, z),
            _padding: 0,
            color: cgmath::Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Default for LightModel {
    fn default() -> Self {
        Self::new(0.0, 10.0, 0.0)
    }
}

pub struct Lights {
    pub light: LightModel,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl Lights {
    // TODO: Use vector of light models instead of one
    pub fn new(device: &wgpu::Device, light: LightModel) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[light]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

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
            light,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn update(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[self.light]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_SRC,
        });
        encoder.copy_buffer_to_buffer(
            &buffer,
            0,
            &self.buffer,
            0,
            std::mem::size_of::<LightModel>() as wgpu::BufferAddress,
        );
    }
}
