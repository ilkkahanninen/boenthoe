use crate::engine::assets::*;
use boenthoescript::{EnvelopeFn, Vector};
use std::collections::HashMap;

pub fn build(asset: &Asset) -> Result<Script, String> {
    if let AssetType::BoenthoeScript = asset.get_type() {
        match asset {
            Asset::Ready { data, .. } => {
                let source = std::str::from_utf8(data)
                    .or_else(|err| Err(format!("UTF-8 error at {}", err.valid_up_to())))?;
                boenthoescript::build(source).and_then(|functions| Ok(Script::new(functions)))
            }
            _ => Err("Asset not ready".into()),
        }
    } else {
        Err(format!("Not boenthoescript: {:?}", asset))
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
