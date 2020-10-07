mod testeffect;

use crate::engine::engine::Engine;
use futures::executor::block_on;

pub fn init(window: &mut winit::window::Window) -> Result<Engine, String> {
    let mut engine = block_on(Engine::new(window, "src/demo"));

    // engine.set_music(include_bytes!("assets/musa.mp3"));

    testeffect::TestEffect::attach(&mut engine)?;

    Ok(engine)
}
