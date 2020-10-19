use crate::engine::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct ViewModel {
    pub camera: Camera,
}

impl Default for ViewModel {
    fn default() -> Self {
        Self {
            camera: Camera {
                eye: (0.0, 0.0, -10.0).into(),
                target: (0.0, 0.0, 0.0).into(),
                up: cgmath::Vector3::unit_y(),
                aspect: 16.0 / 9.0,
                fovy: 45.0,
                znear: 0.1,
                zfar: 100.0,
            },
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ViewUniform {
    view_position: cgmath::Vector4<f32>,
    view_proj: cgmath::Matrix4<f32>,
}

unsafe impl bytemuck::Pod for ViewUniform {}
unsafe impl bytemuck::Zeroable for ViewUniform {}

impl From<&ViewModel> for ViewUniform {
    fn from(model: &ViewModel) -> Self {
        Self {
            view_position: model.camera.eye.to_homogeneous(),
            view_proj: model.camera.view_projection_matrix(),
        }
    }
}

pub struct ViewObject {
    pub model: ViewModel,
    storage: UniformBuffer<ViewUniform>,
}

impl ViewObject {
    pub fn new(device: &wgpu::Device) -> Self {
        Self::init(device, ViewModel::default())
    }

    pub fn init(device: &wgpu::Device, model: ViewModel) -> Self {
        Self {
            model,
            storage: UniformBuffer::init(device, ViewUniform::from(&model), "View"),
        }
    }

    pub fn copy_to_gpu(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let data = ViewUniform::from(&self.model);
        self.storage.copy_to_gpu(device, encoder, &data);
    }
}

impl Object for ViewObject {
    fn get_bind_group(&self) -> &wgpu::BindGroup {
        self.storage.get_bind_group()
    }

    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        self.storage.get_layout()
    }
}
