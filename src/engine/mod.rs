#![allow(dead_code)]

pub mod assets;
pub mod camera;
pub mod engine;
pub mod model;
pub mod music;
pub mod object;
pub mod pipeline;
pub mod renderer;
pub mod shaders;
pub mod storagebuffer;
pub mod texture;
pub mod timer;
pub mod transform;
pub mod view;
pub mod window;

pub enum EngineError {
    AssetLoadError { asset_name: String, message: String },
}
