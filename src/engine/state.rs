use winit::{event::*, window::Window};

pub struct State {
  pub surface: wgpu::Surface,
  pub adapter: wgpu::Adapter,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub sc_desc: wgpu::SwapChainDescriptor,
  pub swap_chain: wgpu::SwapChain,
  pub render_pipelines: Vec<wgpu::RenderPipeline>,
  pub size: winit::dpi::PhysicalSize<u32>,
}

impl State {
  pub async fn new(window: &Window) -> Self {
    let size = window.inner_size();

    let surface = wgpu::Surface::create(window);

    let adapter = wgpu::Adapter::request(
      &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::Default,
        compatible_surface: Some(&surface),
      },
      wgpu::BackendBit::PRIMARY, // Vulkan + Metal + DX12 + Browser WebGPU
    )
    .await
    .unwrap(); // Get used to seeing this

    let (device, queue) = adapter
      .request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
          anisotropic_filtering: false,
        },
        limits: Default::default(),
      })
      .await;

    let sc_desc = wgpu::SwapChainDescriptor {
      usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
      format: wgpu::TextureFormat::Bgra8UnormSrgb,
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
    };
    let swap_chain = device.create_swap_chain(&surface, &sc_desc);

    Self {
      surface,
      adapter,
      device,
      queue,
      sc_desc,
      swap_chain,
      render_pipelines: vec![],
      size,
    }
  }

  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    self.size = new_size;
    self.sc_desc.width = new_size.width;
    self.sc_desc.height = new_size.height;
    self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
  }

  // input() won't deal with GPU code, so it can be synchronous
  pub fn input(&mut self, _event: &WindowEvent) -> bool {
    false
  }

  pub fn update(&mut self) {
    // TODO
  }

  pub fn render(&mut self) {
    let frame = self
      .swap_chain
      .get_next_texture()
      .expect("Timeout getting a frame texture");

    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    for pipeline in self.render_pipelines.iter() {
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
          attachment: &frame.view, // what texture to save the colors to
          resolve_target: None,
          load_op: wgpu::LoadOp::Clear,
          store_op: wgpu::StoreOp::Store,
          clear_color: wgpu::Color {
            r: 0.9,
            g: 0.7,
            b: 0.1,
            a: 1.0,
          },
        }],
        depth_stencil_attachment: None,
      });

      render_pass.set_pipeline(pipeline);
      render_pass.draw(0..3, 0..1);
    }

    self.queue.submit(&[encoder.finish()]);
  }
}
