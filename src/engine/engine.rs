use crate::engine::*;
use winit::{event::*, window::Window};

pub struct Engine {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swap_chain_descriptor: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub renderers: Vec<Box<dyn renderer::Renderer>>,
    pub timer: timer::Timer,
    pub music: Option<music::Music>,
}

impl Engine {
    pub async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(window);
            (size, surface)
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: Default::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .unwrap();

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            swap_chain_descriptor,
            swap_chain,
            size,
            renderers: vec![],
            timer: timer::Timer::new(),
            music: None,
        }
    }

    pub fn set_music(&mut self, bytes: &[u8]) {
        self.music = Some(music::Music::from_bytes(bytes));
    }

    pub fn create_render_buffer(&self) -> texture::Texture {
        let builder = texture::TextureBuilder::new(&self);
        let buffer = builder.color_buffer("render_buffer");
        self.queue.submit(builder.command_buffers);
        buffer
    }

    pub fn add_renderer(&mut self, renderer: Box<dyn renderer::Renderer>) {
        self.renderers.push(renderer);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        const REWIND_AMOUNT: f64 = 10.0;

        match event {
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Left),
                    ..
                } => {
                    self.forward(-REWIND_AMOUNT);
                    return true;
                }

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Right),
                    ..
                } => {
                    self.forward(REWIND_AMOUNT);
                    return true;
                }

                _ => {}
            },
            _ => {}
        }
        false
    }

    pub fn init(&mut self) {
        if let Some(music) = self.music.as_mut() {
            music.play();
        }
        self.timer.reset();
    }

    pub fn render(&mut self) {
        let frame = self
            .swap_chain
            .get_current_frame()
            .expect("Timeout getting a frame texture");

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let time = self.timer.elapsed();
        for renderer in self.renderers.iter_mut() {
            let mut context = renderer::RenderingContext {
                device: &self.device,
                encoder: &mut encoder,
                output: &frame.output.view,
                time,
                screen_size: &self.size,
            };
            if renderer.should_render(&context) {
                renderer.update(&mut context);
                renderer.render(&mut context);
            }
        }

        self.queue.submit(vec![encoder.finish()]);
    }

    pub fn forward(&mut self, seconds: f64) {
        if let Some(music) = self.music.as_mut() {
            music.forward(seconds);
        }
        self.timer.forward(seconds);
    }
}
