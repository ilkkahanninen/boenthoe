#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub color: cgmath::Vector4<f32>,
    pub position: cgmath::Point3<f32>,
}

impl Light {
    pub fn new(color: cgmath::Vector4<f32>, position: cgmath::Point3<f32>) -> Self {
        Self { color, position }
    }
}

impl Default for Light {
    fn default() -> Self {
        Self {
            color: (1.0, 1.0, 1.0, 1.0).into(),
            position: (0.0, 50.0, 0.0).into(),
        }
    }
}

unsafe impl bytemuck::Zeroable for Light {}
unsafe impl bytemuck::Pod for Light {}

#[derive(Debug, Copy, Clone)]
pub enum LightingModel {
    Phong,
}

impl Default for LightingModel {
    fn default() -> Self {
        Self::Phong
    }
}
