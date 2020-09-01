use cgmath::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Transform(Matrix4<f32>);

impl Transform {
    pub fn new() -> Self {
        Self(Matrix4::<f32>::identity())
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        Self(self.0 * Matrix4::from_translation(Vector3::new(x, y, z)))
    }

    pub fn scale(&self, factor: f32) -> Self {
        Self(self.0 * Matrix4::from_scale(factor))
    }

    pub fn scale_xyz(mut self, x: f32, y: f32, z: f32) -> Self {
        Self(self.0 * Matrix4::from_nonuniform_scale(x, y, z))
    }

    pub fn rotate<A: Into<Rad<f32>>>(
        mut self,
        axis_x: f32,
        axis_y: f32,
        axis_z: f32,
        angle: A,
    ) -> Self {
        Self(
            self.0
                * Matrix4::from_axis_angle(Vector3::new(axis_x, axis_y, axis_z).normalize(), angle),
        )
    }
}

unsafe impl bytemuck::Pod for Transform {}
unsafe impl bytemuck::Zeroable for Transform {}
