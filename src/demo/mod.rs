mod layer;
mod state;
mod testeffect;

use crate::demo::state::State;
use crate::engine::engine::Engine;
use futures::executor::block_on;

pub fn init(window: &mut winit::window::Window) -> Engine<state::State> {
    let state = State::new();

    let mut engine = block_on(Engine::new(window, state));
    // engine.set_music(include_bytes!("assets/musa.mp3"));
    //engine.add_renderer(testeffect::TestEffect::new(&engine));
    engine.add_renderer(layer::Layer::new(&engine));

    engine
}
