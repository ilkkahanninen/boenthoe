use super::EffectLayer;
use crate::engine::prelude::*;

pub struct FieldOfDepth {
    initial_horizontal_blur: EffectLayer,
    horizontal_blur: EffectLayer,
    vertical_blur: EffectLayer,

    input: Rc<Texture>,
    depth_buffer: Rc<Texture>,
    pingpong_buffers: (Rc<Texture>, Rc<Texture>),
    output: Option<Rc<Texture>>,

    quality: u32,
    focus: f32,
}

impl FieldOfDepth {
    pub fn new(
        engine: &Engine,
        quality: u32,
        input: Rc<Texture>,
        depth_buffer: Rc<Texture>,
        output: Option<Rc<Texture>>,
    ) -> Result<Self, EngineError> {
        let fragment_shader = engine.add_asset(
            Path::new("effect_layer/shaders/fod_blur.frag"),
            include_bytes!("shaders/fod_blur.frag"),
        );

        let pingpong_buffers = (
            Rc::new(textures::color_buffer(engine, 1.0)),
            Rc::new(textures::color_buffer(engine, 1.0)),
        );

        let mut initial_horizontal_blur = EffectLayer::new(
            engine,
            &[input.get_layout(), depth_buffer.get_layout()],
            &fragment_shader,
            &[],
            "FieldOfDepth::InitialHorizontal",
        )?;

        let mut horizontal_blur = EffectLayer::new(
            engine,
            &[pingpong_buffers.1.get_layout(), depth_buffer.get_layout()],
            &fragment_shader,
            &[],
            "FieldOfDepth::Horizontal",
        )?;

        let mut vertical_blur = EffectLayer::new(
            engine,
            &[pingpong_buffers.0.get_layout(), depth_buffer.get_layout()],
            &fragment_shader,
            &[],
            "FieldOfDepth::Vertical",
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
            depth_buffer,
            pingpong_buffers,
            output,

            quality,
            focus: 0.33,
        })
    }

    fn focus(&mut self, distance: f32, znear: f32, zfar: f32) {
        self.focus = (distance - znear) / (zfar - znear);
    }

    fn focus_camera(&mut self, camera: Camera) {
        let dx = camera.target.x - camera.eye.x;
        let dy = camera.target.y - camera.eye.y;
        let dz = camera.target.z - camera.eye.z;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        self.focus(distance, camera.znear, camera.zfar);
    }
}

impl Renderer for FieldOfDepth {
    fn update(&mut self, _context: &mut RenderingContext) {}

    fn render(&mut self, context: &mut RenderingContext) {
        let threshold_step = self.focus.max(1.0 - self.focus) / self.quality as f32;

        for i in 0..self.quality {
            let args = [threshold_step * (i + 1) as f32, self.focus, 0.0, 0.0];

            if i == 0 {
                self.initial_horizontal_blur.set_args2(&args);
                self.initial_horizontal_blur.update(context);
                self.initial_horizontal_blur.render(
                    context,
                    &[
                        self.input.get_bind_group(),
                        self.depth_buffer.get_bind_group(),
                    ],
                    &self.pingpong_buffers.0.view,
                );
            } else {
                self.horizontal_blur.set_args2(&args);
                self.horizontal_blur.update(context);
                self.horizontal_blur.render(
                    context,
                    &[
                        self.pingpong_buffers.1.get_bind_group(),
                        self.depth_buffer.get_bind_group(),
                    ],
                    &self.pingpong_buffers.0.view,
                );
            }

            self.vertical_blur.set_args2(&args);
            self.vertical_blur.update(context);
            self.vertical_blur.render(
                context,
                &[
                    self.pingpong_buffers.0.get_bind_group(),
                    self.depth_buffer.get_bind_group(),
                ],
                if i < self.quality - 1 {
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
