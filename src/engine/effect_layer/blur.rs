use super::EffectLayer;
use crate::engine::prelude::*;

pub struct Blur {
    initial_horizontal_blur: EffectLayer,
    horizontal_blur: EffectLayer,
    vertical_blur: EffectLayer,

    input: Rc<Texture>,
    pingpong_buffers: (Rc<Texture>, Rc<Texture>),
    output: Option<Rc<Texture>>,
    output_blend: Option<Rc<Texture>>,

    amount: u32,
}

impl Blur {
    pub fn new(
        engine: &Engine,
        input: Rc<Texture>,
        output: Option<Rc<Texture>>,
        output_blend: Option<Rc<Texture>>,
    ) -> Result<Self, EngineError> {
        let fragment_shader = engine.add_asset(
            Path::new("effect_layer/shaders/blur.frag"),
            include_bytes!("shaders/blur.frag"),
        );

        let pingpong_buffers = (
            Rc::new(textures::color_buffer(engine, 0.5)),
            Rc::new(textures::color_buffer(engine, 0.5)),
        );

        let mut initial_horizontal_blur = EffectLayer::new(
            engine,
            &[input.get_layout()],
            &fragment_shader,
            &[],
            "Blur::InitialHorizontal",
        )?;

        let mut horizontal_blur = EffectLayer::new(
            engine,
            &[pingpong_buffers.1.get_layout()],
            &fragment_shader,
            &[],
            "Blur::Horizontal",
        )?;

        let mut vertical_blur = EffectLayer::new(
            engine,
            &[pingpong_buffers.0.get_layout()],
            &fragment_shader,
            match output_blend {
                Some(_) => &["BLEND_WITH_SECONDARY"],
                None => &[],
            },
            "Blur::Vertical",
        )?;

        // Set coefficients
        let resolution = (engine.size.width as f32, engine.size.height as f32);
        let k1 = 1.3846153846;
        let k2 = 3.2307692308;
        let h_off1 = k1 / resolution.0;
        let h_off2 = k2 / resolution.0;
        let v_off1 = k1 / resolution.1;
        let v_off2 = k2 / resolution.1;

        initial_horizontal_blur.set_args(&[h_off1, 0.0, h_off2, 0.0]);
        horizontal_blur.set_args(&[h_off1, 0.0, h_off2, 0.0]);
        vertical_blur.set_args(&[0.0, v_off1, 0.0, v_off2]);

        Ok(Self {
            initial_horizontal_blur,
            horizontal_blur,
            vertical_blur,

            input,
            pingpong_buffers,
            output,
            output_blend,

            amount: 5,
        })
    }

    pub fn set_blur_size(&mut self, blur_size: u32) {
        self.amount = blur_size;
    }
}

impl Renderer for Blur {
    fn update(&mut self, context: &mut RenderingContext) {
        // TODO: Update only on first run
        self.initial_horizontal_blur.update(context);
        self.horizontal_blur.update(context);
        self.vertical_blur.update(context);
    }

    fn render(&mut self, context: &mut RenderingContext) {
        // TODO: Copy texture from input to output if amount == 0
        for i in 0..self.amount {
            if i == 0 {
                self.initial_horizontal_blur.render(
                    context,
                    &[self.input.get_bind_group()],
                    &self.pingpong_buffers.0.view,
                );
            } else {
                self.horizontal_blur.render(
                    context,
                    &[self.pingpong_buffers.1.get_bind_group()],
                    &self.pingpong_buffers.0.view,
                );
            }

            self.vertical_blur.render(
                context,
                &[self.pingpong_buffers.0.get_bind_group()],
                if i < self.amount - 1 {
                    &self.pingpong_buffers.1.view
                } else {
                    match self.output {
                        Some(ref output) => &output.view,
                        None => context.output,
                    }
                },
            );
        }
    }
}
