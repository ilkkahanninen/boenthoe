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
    Unknown,
}

#[derive(Debug, Clone)]
pub enum Asset {
    Pending { path: PathBuf },
    Ready { path: PathBuf, data: Vec<u8> },
    Error { path: PathBuf, message: String },
}

impl Asset {
    pub fn get_type(&self) -> AssetType {
        match self.path().extension() {
            Some(ext) => match ext.to_string_lossy().to_lowercase().as_str() {
                "vert" => AssetType::GlslVertexShader,
                "frag" => AssetType::GlslFragmentShader,
                "boe" => AssetType::BoenthoeScript,
                "png" => AssetType::PngImage,
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

    pub fn pending(&self) -> Self {
        if let Self::Ready { path, data: _ } = self.to_owned() {
            Self::Pending { path }
        } else {
            self.clone()
        }
    }

    pub fn get_data(&self) -> Result<&Vec<u8>, String> {
        match self {
            Asset::Ready { data, .. } => Ok(data),
            _ => Err("Asset data is not available".into()),
        }
    }
}

impl From<PathBuf> for Asset {
    fn from(path: PathBuf) -> Self {
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
}

pub struct AssetLibrary {
    asset_path: String,
    assets: HashMap<PathBuf, Rc<Asset>>,
    watcher: Option<AssetWatcher>,
}

struct AssetWatcher {
    watcher: notify::FsEventWatcher,
    receiver: std::sync::mpsc::Receiver<notify::DebouncedEvent>,
}

impl AssetLibrary {
    pub fn new(asset_path: &str) -> Self {
        Self {
            asset_path: asset_path.into(),
            assets: HashMap::new(),
            watcher: None,
        }
    }

    pub fn file(&mut self, filename: &str) -> Rc<Asset> {
        let path = std::fs::canonicalize(Path::new(&self.asset_path).join(filename)).unwrap();
        let asset = Rc::<Asset>::new(path.clone().into());
        self.assets.insert(path, asset.clone());
        asset.clone()
    }

    pub fn path(&mut self, path: &Path) -> Rc<Asset> {
        match std::fs::canonicalize(Path::new(&self.asset_path).join(path)) {
            Ok(path) => {
                let asset = Rc::<Asset>::new(path.clone().into());
                self.assets.insert(path, asset.clone());
                asset.clone()
            }
            Err(error) => Rc::new(Asset::Error {
                path: path.to_path_buf(),
                message: error.to_string(),
            }),
        }
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
            new_assets.insert(path.clone(), Rc::new(asset.pending()));
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
                            .insert(path.clone(), Rc::<Asset>::new(path.clone().into()));
                        changes_detected = true;
                    }
                }
            }
        }

        changes_detected
    }
}
