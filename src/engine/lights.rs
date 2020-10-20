use crate::engine::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Light {
    Unlit,
    Directional {
        direction: Vector3,

        ambient: Vector3,
        diffuse: Vector3,
        specular: Vector3,
    },
    Point {
        position: Point3,

        ambient: Vector3,
        diffuse: Vector3,
        specular: Vector3,

        constant: f32,
        linear: f32,
        quadrant: f32,
    },
    Spotlight {
        position: Point3,
        look_at: Point3,

        ambient: Vector3,
        diffuse: Vector3,
        specular: Vector3,

        angle: cgmath::Deg<f32>,
        hardness: f32,
    },
}

impl Light {
    pub fn is_lit(&self) -> bool {
        match self {
            Self::Unlit => false,
            Self::Directional {
                ambient,
                diffuse,
                specular,
                ..
            } => is_nonblack(ambient) || is_nonblack(diffuse) || is_nonblack(specular),
            Self::Point {
                ambient,
                diffuse,
                specular,
                ..
            } => is_nonblack(ambient) || is_nonblack(diffuse) || is_nonblack(specular),
            Self::Spotlight {
                ambient,
                diffuse,
                specular,
                ..
            } => is_nonblack(ambient) || is_nonblack(diffuse) || is_nonblack(specular),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum LightType {
    Unlit,
    Directional,
    Point,
    Spotlight,
}

impl Into<u32> for LightType {
    fn into(self) -> u32 {
        match self {
            Self::Unlit => 0,
            Self::Directional => 1,
            Self::Point => 2,
            Self::Spotlight => 3,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LightBufferObject {
    position: Vector4,
    direction: Vector4,

    ambient: Vector4,
    diffuse: Vector4,
    specular: Vector4,

    parameters: Vector4,

    light_type: u32,
    _padding: [u32; 3],
}

unsafe impl bytemuck::Zeroable for LightBufferObject {}
unsafe impl bytemuck::Pod for LightBufferObject {}

impl From<&Light> for LightBufferObject {
    fn from(light: &Light) -> Self {
        match light {
            Light::Unlit => Self::default(),
            Light::Directional {
                direction,
                ambient,
                diffuse,
                specular,
            } => Self {
                light_type: LightType::Directional.into(),
                direction: homogeneous_direction(direction),
                ambient: rgba_color(ambient),
                diffuse: rgba_color(diffuse),
                specular: rgba_color(specular),
                ..Default::default()
            },
            Light::Point {
                position,
                ambient,
                diffuse,
                specular,
                constant,
                linear,
                quadrant,
            } => Self {
                light_type: LightType::Point.into(),
                position: position.to_homogeneous(),
                ambient: rgba_color(ambient),
                diffuse: rgba_color(diffuse),
                specular: rgba_color(specular),
                parameters: Vector4::new(*constant, *linear, *quadrant, 0.0),
                ..Default::default()
            },
            Light::Spotlight {
                position,
                look_at,
                ambient,
                diffuse,
                specular,
                angle,
                hardness,
            } => {
                let clamped_hardness = hardness.min(1.0).max(0.0);
                let angle = cgmath::Rad::from(*angle).0;
                let inner_angle = angle * (1.0 - clamped_hardness);
                let outer_angle = angle * (1.0 + clamped_hardness);
                Self {
                    light_type: LightType::Spotlight.into(),
                    position: position.to_homogeneous(),
                    direction: homogeneous_direction(&(look_at - position)),
                    ambient: rgba_color(ambient),
                    diffuse: rgba_color(diffuse),
                    specular: rgba_color(specular),
                    parameters: Vector4::new(inner_angle.cos(), outer_angle.cos(), 0.0, 0.0),
                    ..Default::default()
                }
            }
        }
    }
}

impl Default for LightBufferObject {
    fn default() -> Self {
        Self {
            light_type: LightType::Unlit.into(),
            position: (0.0, 0.0, 0.0, 0.0).into(),
            direction: (0.0, 0.0, 0.0, 0.0).into(),
            ambient: (0.0, 0.0, 0.0, 0.0).into(),
            diffuse: (0.0, 0.0, 0.0, 0.0).into(),
            specular: (0.0, 0.0, 0.0, 0.0).into(),
            parameters: (0.0, 0.0, 0.0, 0.0).into(),
            _padding: [0xbe, 0xee, 0xef],
        }
    }
}

fn homogeneous_direction(direction: &Vector3) -> Vector4 {
    Vector4::new(direction.x, direction.y, direction.z, 0.0)
}

fn rgba_color(rgb: &Vector3) -> Vector4 {
    Vector4::new(rgb.x, rgb.y, rgb.z, 1.0)
}

fn is_nonblack(rgb: &Vector3) -> bool {
    rgb.x != 0.0 && rgb.y != 0.0 && rgb.z != 0.0
}

#[derive(Debug, Copy, Clone)]
pub enum LightingModel {
    Phong,
}

impl Default for LightingModel {
    fn default() -> Self {
        Self::Phong
    }
}
