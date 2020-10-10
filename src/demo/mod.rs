mod testeffect;

use crate::engine::{engine::Engine, EngineError};
use futures::executor::block_on;
use std::path::Path;

pub fn init(window: &mut winit::window::Window) -> Result<Engine, EngineError> {
    let engine = block_on(Engine::new(window, &Path::new("src/demo")));

    // engine.set_music(include_bytes!("assets/musa.mp3"));

    testeffect::TestEffect::attach(&engine)?;

    Ok(engine)
}
