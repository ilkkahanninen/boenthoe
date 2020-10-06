use crate::engine::{camera::Camera, object::Object, storagebuffer::StorageObject};

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

unsafe impl bytemuck::Pod for ViewUniform {}
unsafe impl bytemuck::Zeroable for ViewUniform {}

impl From<&ViewModel> for ViewUniform {
    fn from(model: &ViewModel) -> Self {
        Self {
            view_position: model.camera.eye.to_homogeneous(),
            view_proj: model.camera.build_view_projection_matrix(),
        }
    }
}

pub struct ViewObject {
    pub model: ViewModel,
    storage: StorageObject<ViewUniform>,
}

impl ViewObject {
    pub fn new(device: &wgpu::Device, model: ViewModel) -> Self {
        Self {
            model,
            storage: StorageObject::new(device, ViewUniform::from(&model)),
        }
    }

    pub fn copy_to_gpu(&mut self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        self.storage.data = ViewUniform::from(&self.model);
        self.storage.copy_to_gpu(device, encoder);
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
