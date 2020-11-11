mod simple;
mod testeffect;

use crate::engine::prelude::*;
use futures::executor::block_on;
use std::path::Path;

pub fn init(window: &mut winit::window::Window) -> Result<Engine, EngineError> {
    let engine = block_on(Engine::new(window, &Path::new("src/demo")));

    // engine.set_music(include_bytes!("assets/musa.mp3"));

    // let buffer = Rc::new(textures::color_buffer(&engine, 1.0));
    let depth_buffer = Rc::new(textures::depth_buffer(&engine));

    let test_model = testeffect::TestEffect::new(&engine, depth_buffer.clone(), None)?;
    engine.add_renderer(Box::new(test_model));

    // let simple = simple::Simple::new(&engine)?;
    // engine.add_renderer(Box::new(simple));

    // let bloom = effect_layer::Bloom::new(&engine, buffer.clone(), None)?;
    // engine.add_renderer(Box::new(bloom));
    // let fod =
    //     effect_layer::FieldOfDepth::new(&engine, 6, buffer.clone(), depth_buffer.clone(), None)?;
    // engine.add_renderer(Box::new(fod));

    Ok(engine)
}
