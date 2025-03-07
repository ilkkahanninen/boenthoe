use crate::engine::prelude::*;

pub trait Renderer {
    fn reload_assets(&mut self, _assets: &AssetLibrary) -> Result<(), EngineError> {
        Ok(())
    }
    fn should_render(&self, _context: &RenderingContext) -> bool {
        true
    }
    fn update(&mut self, context: &mut RenderingContext);
    fn render(&mut self, context: &mut RenderingContext);
}

pub struct RenderingContext<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a mut wgpu::Queue,
    pub output: &'a wgpu::TextureView,
    pub time: f64,
    pub screen_size: &'a winit::dpi::PhysicalSize<u32>,
}

impl<'a> RenderingContext<'a> {
    pub fn create_encoder(&mut self, label: &str) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some(label) })
    }

    pub fn submit(&self, encoder: wgpu::CommandEncoder) {
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}

impl<'a> RenderingContext<'a> {
    pub fn clear(
        &mut self,
        color: wgpu::Color,
        output: Option<&wgpu::TextureView>,
        depth_buffer: Option<&wgpu::TextureView>,
    ) {
        let mut encoder = self.create_encoder("Clear");
        {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: output.unwrap_or(self.output),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color),
                        store: true,
                    },
                }],
                depth_stencil_attachment: depth_buffer.map(|attachment| {
                    wgpu::RenderPassDepthStencilAttachmentDescriptor {
                        attachment,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }
                }),
            });
        }
        self.submit(encoder);
    }
}
