mod background;
mod layer;
mod state;
mod testeffect;
mod titles;

use crate::demo::state::State;
use crate::engine::engine::Engine;
use futures::executor::block_on;
use std::rc::Rc;

pub fn init(window: &mut winit::window::Window) -> Engine<state::State> {
    let state = State::new();

    let mut engine = block_on(Engine::new(window, state));

    // engine.set_music(include_bytes!("assets/musa.mp3"));

    let buffer = Rc::new(engine.create_render_buffer());
    engine.add_renderer(background::Background::new(&engine, buffer.clone()));
    // engine.add_renderer(testeffect::TestEffect::new(&engine, buffer.clone()));
    engine.add_renderer(titles::Titles::new(&engine, buffer.clone()));
    engine.add_renderer(layer::Layer::new(&engine, buffer.clone()));

    engine
}
