use super::{Blur, EffectLayer};
use crate::engine::prelude::*;

pub struct Bloom {
    threshold: EffectLayer,
    blur: Blur,
    buffer: Rc<Texture>,
}

impl Bloom {
    pub fn new(
        engine: &Engine,
        input: Rc<Texture>,
        output: Option<Rc<Texture>>,
    ) -> Result<Self, EngineError> {
        let fragment_shader = engine.add_asset(
            Path::new("effect_layer/shaders/bloom_threshold.frag"),
            include_bytes!("shaders/bloom_threshold.frag"),
        );

        let buffer = Rc::new(textures::color_buffer(engine, 1.0));
        let threshold = EffectLayer::new(
            engine,
            Some(buffer.clone()),
            &[input.clone()],
            &fragment_shader,
            &[],
            "BloomThreshold",
        )?;

        let mut blur = Blur::new(engine, buffer.clone(), Some(input.clone()), output)?;
        blur.set_size(16, 0.02);

        Ok(Self {
            threshold,
            blur,
            buffer,
        })
    }
}

impl Renderer for Bloom {
    fn update(&mut self, context: &mut RenderingContext) {
        self.threshold.update(context);
        self.blur.update(context);
    }

    fn render(&mut self, context: &mut RenderingContext) {
        self.threshold.render(context);
        self.blur.render(context);
    }
}
