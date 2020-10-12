use crate::engine::EngineError;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

#[derive(Debug, Copy, Clone)]
pub enum AssetType {
    GlslVertexShader,
    GlslFragmentShader,
    BoenthoeScript,
    PngImage,
    JpegImage,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum Asset {
    Pending { path: PathBuf },
    Ready { path: PathBuf, data: Vec<u8> },
    Error { path: PathBuf, message: String },
}

impl Asset {
    fn load(path: PathBuf) -> Self {
        let data = fs::read(&path).or_else(|err| {
            Err(format!(
                "Loading asset `{}` failed: {:?}",
                path.to_string_lossy(),
                err
            ))
        });
        match data {
            Ok(data) => Asset::Ready { path, data },
            Err(message) => Asset::Error { path, message },
        }
    }

    pub fn get_type(&self) -> AssetType {
        match self.path().extension() {
            Some(ext) => match ext.to_string_lossy().to_lowercase().as_str() {
                "vert" => AssetType::GlslVertexShader,
                "frag" => AssetType::GlslFragmentShader,
                "boe" => AssetType::BoenthoeScript,
                "png" => AssetType::PngImage,
                "jpg" => AssetType::JpegImage,
                _ => AssetType::Unknown,
            },
            None => AssetType::Unknown,
        }
    }

    pub fn path(&self) -> &PathBuf {
        match self {
            Self::Pending { path, .. } | Self::Ready { path, .. } | Self::Error { path, .. } => {
                path
            }
        }
    }

    pub fn data(&self) -> Result<&Vec<u8>, EngineError> {
        match self {
            Asset::Ready { data, .. } => Ok(data),
            Asset::Error { path, message } => Err(EngineError::AssetLoadError {
                path: path.clone(),
                message: message.clone(),
            }),
            _ => Err(EngineError::AssetNotLoaded {
                path: self.path().clone(),
            }),
        }
    }

    pub fn to_utf8(&self) -> Result<&str, EngineError> {
        let data = self.data()?;
        let utf8 = std::str::from_utf8(data).or_else(|err| {
            Err(EngineError::AssetParseError {
                path: self.path().clone(),
                message: format!("UTF-8 error at {}", err.valid_up_to()),
            })
        })?;
        Ok(utf8)
    }

    pub fn to_pending(&self) -> Self {
        if let Self::Ready { path, data: _ } = self.to_owned() {
            Self::Pending { path }
        } else {
            self.clone()
        }
    }
}

pub struct AssetLibrary {
    asset_path: PathBuf,
    assets: HashMap<PathBuf, Rc<Asset>>,
    watcher: Option<AssetWatcher>,
}

struct AssetWatcher {
    watcher: notify::FsEventWatcher,
    receiver: std::sync::mpsc::Receiver<notify::DebouncedEvent>,
}

impl AssetLibrary {
    pub fn new(asset_path: &Path) -> Self {
        Self {
            asset_path: asset_path.into(),
            assets: HashMap::new(),
            watcher: None,
        }
    }

    /// Load asset from asset path
    pub fn load(&mut self, path: &Path) -> Rc<Asset> {
        let path = Path::new(&self.asset_path).join(path);
        let exact_path = std::fs::canonicalize(&path).unwrap();
        let relative_path = self.relative_path(&path);

        println!("Load asset {:?}...", relative_path);
        let asset = Rc::<Asset>::new(Asset::load(exact_path.clone()));
        self.assets.insert(relative_path, asset.clone());
        asset.clone()
    }

    pub fn asset_dir(&self, asset: &Asset) -> PathBuf {
        let mut path = self.relative_path(asset.path());
        path.pop();
        path
    }

    pub fn changed(&self, filename: &str) -> Option<Rc<Asset>> {
        for (path, asset) in self.assets.iter() {
            match asset.as_ref() {
                Asset::Ready { .. } => {
                    if path.file_name().unwrap().to_string_lossy() == filename {
                        return Some(asset.clone());
                    }
                }
                _ => (),
            }
        }
        None
    }

    pub fn clear_assets(&mut self) {
        let mut new_assets = HashMap::new();
        for (path, asset) in self.assets.iter() {
            new_assets.insert(path.clone(), Rc::new(asset.to_pending()));
        }
        self.assets = new_assets;
    }

    pub fn start_watcher(&mut self) {
        use notify::Watcher;

        self.clear_assets();

        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::watcher(tx, std::time::Duration::from_millis(250)).unwrap();
        watcher
            .watch(&self.asset_path, notify::RecursiveMode::Recursive)
            .unwrap();

        self.watcher = Some(AssetWatcher {
            watcher,
            receiver: rx,
        });
    }

    pub fn detect_changes(&mut self) -> bool {
        let mut changes_detected = false;

        if let Some(watcher) = &self.watcher {
            while let Ok(event) = watcher.receiver.try_recv() {
                if let notify::DebouncedEvent::Write(path) = event {
                    if let Some(_) = self.assets.get(&path) {
                        println!("Change detected: {:?}", path);
                        self.assets
                            .insert(path.clone(), Rc::<Asset>::new(Asset::load(path.clone())));
                        changes_detected = true;
                    }
                }
            }
        }

        changes_detected
    }

    fn relative_path(&self, path: &Path) -> PathBuf {
        let diff = if path.is_absolute() {
            pathdiff::diff_paths(path, &std::fs::canonicalize(&self.asset_path).unwrap())
        } else {
            pathdiff::diff_paths(path, &self.asset_path)
        };
        diff.unwrap_or_else(|| {
            panic!(
                "Could not resolve relation of {:?} to path {:?}",
                path, self.asset_path
            );
        })
    }
}
