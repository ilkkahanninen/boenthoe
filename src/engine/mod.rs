#![allow(dead_code)]
use std::path::PathBuf;

pub mod assets;
pub mod camera;
pub mod engine;
pub mod model;
pub mod music;
pub mod object;
pub mod pipeline;
pub mod renderer;
pub mod scripts;
pub mod shaders;
pub mod storagebuffer;
pub mod textures;
pub mod timer;
pub mod transform;
pub mod view;
pub mod window;

#[derive(Debug)]
pub enum EngineError {
    UnsupportedAssetType { path: PathBuf, expected: String },
    AssetParseError { path: PathBuf, message: String },
    AssetLoadError { path: PathBuf, message: String },
    AssetNotLoaded { path: PathBuf },
}
