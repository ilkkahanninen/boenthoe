#![allow(dead_code)]
use std::path::PathBuf;

pub mod assets;
pub mod camera;
pub mod databuffers;
pub mod engine;
pub mod lights;
pub mod model;
pub mod music;
pub mod object;
pub mod pipeline;
pub mod renderer;
pub mod scripts;
pub mod shaders;
pub mod textures;
pub mod timer;
pub mod transform;
pub mod view;
pub mod window;

#[derive(Debug)]
pub enum EngineError {
    UnsupportedAssetFormat { path: PathBuf, expected: String },
    AssetParseError { path: PathBuf, message: String },
    AssetLoadError { path: PathBuf, message: String },
    AssetNotLoaded { path: PathBuf },
}

impl EngineError {
    pub fn parse_error<T>(asset: &assets::Asset, error: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::AssetParseError {
            path: asset.path().clone(),
            message: error.to_string(),
        }
    }

    pub fn unsupported_asset_format(asset: &assets::Asset, expected: &str) -> Self {
        Self::UnsupportedAssetFormat {
            path: asset.path().clone(),
            expected: String::from(expected),
        }
    }
}

pub mod prelude {
    pub type Point3 = cgmath::Point3<f32>;
    pub type Matrix3 = cgmath::Matrix3<f32>;
    pub type Matrix4 = cgmath::Matrix4<f32>;

    pub use super::assets::{Asset, AssetLibrary, AssetType};
    pub use super::camera::Camera;
    pub use super::databuffers::{StorageBuffer, UniformBuffer};
    pub use super::engine::Engine;
    pub use super::lights::{Light, LightingModel};
    pub use super::object::Object;
    pub use super::renderer::{Renderer, RenderingContext};
    pub use super::textures;
    pub use super::textures::Texture;
    pub use super::EngineError;
}
