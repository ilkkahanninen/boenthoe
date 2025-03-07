use crate::engine::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Light {
    Unlit,
    Ambient {
        color: Vector4,
    },
    Directional {
        direction: Vector3,

        ambient: Vector4,
        diffuse: Vector3,
        specular: Vector3,
    },
    Point {
        position: Point3,

        ambient: Vector4,
        diffuse: Vector3,
        specular: Vector3,

        range: f32,
    },
    Spotlight {
        position: Point3,
        look_at: Point3,

        ambient: Vector4,
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
            Self::Ambient { color } => is_nonblack_rgba(color),
            Self::Directional {
                ambient,
                diffuse,
                specular,
                ..
            } => is_nonblack_rgba(ambient) || is_nonblack(diffuse) || is_nonblack(specular),
            Self::Point {
                ambient,
                diffuse,
                specular,
                ..
            } => is_nonblack_rgba(ambient) || is_nonblack(diffuse) || is_nonblack(specular),
            Self::Spotlight {
                ambient,
                diffuse,
                specular,
                ..
            } => is_nonblack_rgba(ambient) || is_nonblack(diffuse) || is_nonblack(specular),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum LightType {
    Unlit,
    Directional,
    Point,
    Spotlight,
    Ambient,
}

impl Into<u32> for LightType {
    fn into(self) -> u32 {
        match self {
            Self::Unlit => 0,
            Self::Directional => 1,
            Self::Point => 2,
            Self::Spotlight => 3,
            Self::Ambient => 4,
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
            Light::Ambient { color } => Self {
                light_type: LightType::Ambient.into(),
                ambient: *color,
                ..Default::default()
            },
            Light::Directional {
                direction,
                ambient,
                diffuse,
                specular,
            } => Self {
                light_type: LightType::Directional.into(),
                direction: homogeneous_direction(direction),
                ambient: *ambient,
                diffuse: rgba_color(diffuse),
                specular: rgba_color(specular),
                ..Default::default()
            },
            Light::Point {
                position,
                ambient,
                diffuse,
                specular,
                range,
            } => Self {
                light_type: LightType::Point.into(),
                position: position.to_homogeneous(),
                ambient: *ambient,
                diffuse: rgba_color(diffuse),
                specular: rgba_color(specular),
                parameters: Vector4::new(*range, 1.0, 4.5 / range, 75.0 / (range * range)),
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
                    ambient: *ambient,
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

fn is_nonblack_rgba(rgba: &Vector4) -> bool {
    rgba.x != 0.0 && rgba.y != 0.0 && rgba.z != 0.0 && rgba.w != 0.0
}
