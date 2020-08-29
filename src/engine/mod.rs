pub mod camera;
pub mod pipeline;
pub mod shaders;
pub mod texture;
pub mod timer;
pub mod uniforms;
pub mod window;

use winit::{event::*, window::Window};

pub struct Engine {
  pub surface: wgpu::Surface,
  pub adapter: wgpu::Adapter,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub sc_desc: wgpu::SwapChainDescriptor,
  pub swap_chain: wgpu::SwapChain,
  pub rendering_contexts: Vec<RenderingContext>,
  pub size: winit::dpi::PhysicalSize<u32>,
}

pub struct RenderingContext {
  pub pipeline: wgpu::RenderPipeline,
  pub uniform_bind_group: wgpu::BindGroup,
  pub render: Box<dyn Fn(RenderFnContext, f64) -> ()>,
}

pub struct RenderFnContext<'a> {
  pub target: &'a wgpu::TextureView,
  pub encoder: &'a mut wgpu::CommandEncoder,
  pub pipeline: &'a wgpu::RenderPipeline,
  pub uniform_bind_group: &'a wgpu::BindGroup,
}

impl Engine {
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
      rendering_contexts: vec![],
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

  pub fn render(&mut self, time: f64) {
    let frame = self
      .swap_chain
      .get_next_texture()
      .expect("Timeout getting a frame texture");

    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    for ctx in self.rendering_contexts.iter() {
      (ctx.render)(
        RenderFnContext {
          target: &frame.view,
          encoder: &mut encoder,
          pipeline: &ctx.pipeline,
          uniform_bind_group: &ctx.uniform_bind_group,
        },
        time,
      );
    }

    self.queue.submit(&[encoder.finish()]);
  }

  pub fn get_aspect_ratio(&self) -> f32 {
    self.sc_desc.width as f32 / self.sc_desc.height as f32
  }
}

impl<'a> RenderFnContext<'a> {
  pub fn begin(&mut self, clear_color: wgpu::Color) -> wgpu::RenderPass {
    let mut render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
        attachment: self.target,
        resolve_target: None,
        load_op: wgpu::LoadOp::Clear,
        store_op: wgpu::StoreOp::Store,
        clear_color,
      }],
      depth_stencil_attachment: None,
    });

    render_pass.set_pipeline(&self.pipeline);
    render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);

    render_pass
  }
}
