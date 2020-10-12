use crate::engine::{assets::*, EngineError};
use boenthoescript::{EnvelopeFn, Vector};
use std::collections::HashMap;

pub fn build(asset: &Asset) -> Result<Script, EngineError> {
    if let AssetType::BoenthoeScript = asset.get_type() {
        boenthoescript::build(asset.to_utf8()?)
            .or_else(|err| {
                Err(EngineError::AssetParseError {
                    path: asset.path().clone(),
                    message: err,
                })
            })
            .and_then(|functions| Ok(Script::new(functions)))
    } else {
        Err(EngineError::unsupported_asset_format(asset, ".boe"))
    }
}

pub struct Script {
    envelopes: HashMap<String, EnvelopeFn>,
    state: HashMap<String, Vector>,
    default: Vector,
}

impl Script {
    fn new(envelopes: HashMap<String, EnvelopeFn>) -> Self {
        Self {
            envelopes,
            state: HashMap::new(),
            default: 0.0.into(),
        }
    }

    pub fn set_time(&mut self, time: f64) {
        for (name, envelope) in self.envelopes.iter() {
            self.state.insert(name.clone(), envelope.get_value(time));
        }
    }

    pub fn get(&self, key: &str) -> &Vector {
        self.state.get(key).unwrap_or(&self.default)
    }
}
