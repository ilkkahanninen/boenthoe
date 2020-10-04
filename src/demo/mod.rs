mod testeffect;

use crate::engine::engine::Engine;
use futures::executor::block_on;
use std::rc::Rc;

pub fn init(window: &mut winit::window::Window) -> Engine {
    let mut engine = block_on(Engine::new(window));

    // engine.set_music(include_bytes!("assets/musa.mp3"));

    engine.add_renderer(testeffect::TestEffect::new(&engine));

    engine
}
