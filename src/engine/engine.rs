use crate::engine::*;
use winit::{event::*, window::Window};

pub struct Engine<T> {
  pub instance: wgpu::Instance,
  pub surface: wgpu::Surface,
  pub adapter: wgpu::Adapter,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub swap_chain_descriptor: wgpu::SwapChainDescriptor,
  pub swap_chain: wgpu::SwapChain,
  pub size: winit::dpi::PhysicalSize<u32>,
  pub renderers: Vec<Box<dyn renderer::Renderer<T>>>,
  pub get_state: Box<dyn Fn(&f64) -> T>,
  pub timer: timer::Timer,
}

impl<T> Engine<T> {
  pub async fn new(window: &Window, get_state: Box<dyn Fn(&f64) -> T>) -> Self {
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
      get_state,
      renderers: vec![],
      timer: timer::Timer::new(),
    }
  }

  pub fn add_renderer(&mut self, renderer: Box<dyn renderer::Renderer<T>>) {
    self.renderers.push(renderer);
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    self.size = new_size;
    self.swap_chain_descriptor.width = new_size.width;
    self.swap_chain_descriptor.height = new_size.height;
    self.swap_chain = self
      .device
      .create_swap_chain(&self.surface, &self.swap_chain_descriptor);
  }

  pub fn input(&mut self, _event: &WindowEvent) -> bool {
    false
  }

  pub fn start(&mut self) {
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
    let state = (self.get_state)(&time);
    for renderer in self.renderers.iter_mut() {
      if renderer.should_render(time) {
        renderer.update(&renderer::UpdateContext {
          time: &time,
          device: &self.device,
          state: &state,
        });
        renderer.render(&mut renderer::RenderingContext {
          device: &self.device,
          encoder: &mut encoder,
          output: &frame.output.view,
          state: &state,
        });
      }
    }

    self.queue.submit(vec![encoder.finish()]);
  }
}
