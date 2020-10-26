use super::EffectLayer;
use crate::engine::prelude::*;

pub struct Blur {
    horizontal_blur: EffectLayer,
    vertical_blur: EffectLayer,
    buffer: Rc<Texture>,
}

impl Blur {
    pub fn new(
        engine: &Engine,
        input: Rc<Texture>,
        blend_input: Option<Rc<Texture>>,
        output: Option<Rc<Texture>>,
    ) -> Result<Self, EngineError> {
        let fragment_shader = engine.add_asset(
            Path::new("effect_layer/shaders/blur.frag"),
            include_bytes!("shaders/blur.frag"),
        );

        let buffer = Rc::new(textures::color_buffer(engine, 0.5));
        let horizontal_blur = EffectLayer::new(
            engine,
            Some(buffer.clone()),
            &[input.clone()],
            &fragment_shader,
            &[],
            "Blur::Horizontal",
        )?;

        let mut inputs = vec![buffer.clone()];
        let mut shader_macro_flags = vec![];
        if let Some(blend) = blend_input {
            inputs.push(blend.clone());
            shader_macro_flags.push("BLEND_WITH_SECONDARY");
        }

        let vertical_blur = EffectLayer::new(
            engine,
            output,
            &inputs,
            &fragment_shader,
            &shader_macro_flags,
            "Blur::Vertical",
        )?;

        Ok(Self {
            horizontal_blur,
            vertical_blur,
            buffer,
        })
    }

    pub fn set_size(&mut self, samples: u32, size: f32) {
        let delta = size / (samples - 1) as f32;
        let start = -size / 2.0;
        self.horizontal_blur
            .set_args(&[delta, start, samples as f32, 1.0]);
        self.vertical_blur
            .set_args(&[delta, start, samples as f32, 0.0]);
    }

    pub fn get_temp_buffer(&self) -> Rc<Texture> {
        self.buffer.clone()
    }
}

impl Renderer for Blur {
    fn update(&mut self, context: &mut RenderingContext) {
        self.horizontal_blur.update(context);
        self.vertical_blur.update(context);
    }

    fn render(&mut self, context: &mut RenderingContext) {
        self.horizontal_blur.render(context);
        self.vertical_blur.render(context);
    }
}
