use std::{
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

#[derive(Debug, Copy, Clone)]
pub enum AssetType {
    GlslVertexShader,
    GlslFragmentShader,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum Asset {
    Pending {
        name: String,
        path: Option<PathBuf>,
    },
    Ready {
        name: String,
        path: Option<PathBuf>,
        data: Vec<u8>,
    },
    Error {
        name: String,
        message: String,
    },
}

impl Asset {
    pub fn get_type(&self) -> AssetType {
        let name = match self {
            Self::Pending { name, .. } | Self::Ready { name, .. } | Self::Error { name, .. } => {
                name
            }
        };
        match Path::new(name).extension() {
            Some(ext) => match ext.to_string_lossy().to_lowercase().as_str() {
                "vert" => AssetType::GlslVertexShader,
                "frag" => AssetType::GlslFragmentShader,
                _ => AssetType::Unknown,
            },
            None => AssetType::Unknown,
        }
    }

    pub fn make_pending(&mut self) {
        if let Self::Ready {
            name,
            path,
            data: _,
        } = self.to_owned()
        {
            *self = Self::Pending { name, path }
        }
    }
}

impl From<PathBuf> for Asset {
    fn from(path: PathBuf) -> Self {
        let name = String::from(path.file_name().unwrap().to_string_lossy());
        let data = fs::read(&path).or_else(|err| {
            Err(format!(
                "Loading asset `{}` failed: {:?}",
                path.to_string_lossy(),
                err
            ))
        });
        match data {
            Ok(data) => Asset::Ready {
                name,
                path: Some(path),
                data,
            },
            Err(message) => Asset::Error { name, message },
        }
    }
}

pub struct AssetLibrary {
    asset_path: String,
    assets: Vec<Rc<Asset>>,
}

impl AssetLibrary {
    pub fn new(asset_path: &str) -> Self {
        Self {
            asset_path: asset_path.into(),
            assets: vec![],
        }
    }

    pub fn file(&mut self, filename: &str) -> Rc<Asset> {
        let path = Path::new(&self.asset_path).join(filename);
        let asset = Rc::<Asset>::new(path.into());
        self.assets.push(asset.clone());
        asset.clone()
    }
}
