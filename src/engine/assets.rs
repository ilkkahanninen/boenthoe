use std::{
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Asset {
    pub name: String,
    pub path: Option<PathBuf>,
    pub asset_type: AssetType,
    pub state: AssetState,
    pub data: Result<Vec<u8>, String>,
}

impl Asset {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            path: None,
            data: Err("Uninitialized asset".into()),
            asset_type: name.into(),
            state: AssetState::Uninitialized,
        }
    }
}

impl From<PathBuf> for Asset {
    fn from(path: PathBuf) -> Self {
        let name = String::from(path.file_name().unwrap().to_string_lossy());
        let data = fs::read(&path)
            .or_else(|err| Err(format!("Loading asset `{}` failed: {:?}", &name, err)));
        let asset_type = AssetType::from(name.as_str());
        Self {
            name,
            path: Some(path),
            data,
            asset_type,
            state: AssetState::Loaded,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AssetState {
    /// An empty asset
    Uninitialized,
    /// Loading asset data from file
    Loading,
    /// Data loaded from file or memory and must be consumed immediately
    Loaded,
    /// Pending for file changes
    Pending,
}

#[derive(Debug, Copy, Clone)]
pub enum AssetType {
    GlslVertexShader,
    GlslFragmentShader,
    Unknown,
}

impl From<&str> for AssetType {
    fn from(s: &str) -> Self {
        match Path::new(s).extension() {
            Some(ext) => match ext.to_string_lossy().to_lowercase().as_str() {
                "vert" => Self::GlslVertexShader,
                "frag" => Self::GlslFragmentShader,
                _ => Self::Unknown,
            },
            None => Self::Unknown,
        }
    }
}

pub type SharedAsset = Rc<Asset>;

pub struct AssetLibrary {
    asset_path: String,
    assets: Vec<SharedAsset>,
}

impl AssetLibrary {
    pub fn new(asset_path: &str) -> Self {
        Self {
            asset_path: asset_path.into(),
            assets: vec![],
        }
    }

    pub fn file(&mut self, filename: &str) -> SharedAsset {
        let path = Path::new(&self.asset_path).join(filename);
        let asset = SharedAsset::new(path.into());
        self.assets.push(asset.clone());
        asset.clone()
    }
}
