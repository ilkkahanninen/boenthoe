mod background;
mod blur;
mod layer;
mod meshes;
mod postprocess;
mod state;
mod titlelayer;

use crate::demo::state::State;
use crate::engine::engine::Engine;
use futures::executor::block_on;
use std::rc::Rc;

pub fn init(window: &mut winit::window::Window) -> Engine<state::State> {
    let state = State::new();

    let mut engine = block_on(Engine::new(window, state));

    engine.set_music(include_bytes!("assets/musa.mp3"));

    let bg_buffer = Rc::new(engine.create_render_buffer());
    let env_buffer = Rc::new(engine.create_render_buffer());
    let stuff_buffer = Rc::new(engine.create_render_buffer());

    // Background
    engine.add_renderer(background::Background::new(&engine, bg_buffer.clone()));
    engine.add_renderer(blur::Blur::new(
        &engine,
        bg_buffer.clone(),
        env_buffer.clone(),
    ));

    // 3D stuff
    engine.add_renderer(layer::Layer::new(
        &engine,
        bg_buffer.clone(),
        stuff_buffer.clone(),
    ));
    engine.add_renderer(meshes::Meshes::new(
        &engine,
        env_buffer.clone(),
        stuff_buffer.clone(),
    ));
    engine.add_renderer(titlelayer::TitleLayer::new(&engine, stuff_buffer.clone()));

    // Post-processing
    engine.add_renderer(postprocess::PostProcess::new(&engine, stuff_buffer.clone()));

    engine
}
