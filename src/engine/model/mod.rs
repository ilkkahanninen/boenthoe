mod gltf_model;

use crate::engine::prelude::*;
use gltf_model::GltfModel;

pub fn load(engine: &Engine, asset: &Asset) -> Result<Box<dyn Model>, EngineError> {
    match asset.get_type() {
        AssetType::GltfModel => Ok(Box::new(GltfModel::new(engine, asset)?)),
        _ => Err(EngineError::unsupported_asset_format(asset, "GLTF")),
    }
}

pub trait Model {
    fn render(&self, context: &mut ModelRenderContext);
    fn set_view_projection_matrix(&mut self, matrix: &Matrix4);
}

pub struct ModelRenderContext<'a> {
    pub device: &'a wgpu::Device,
    pub output: &'a wgpu::TextureView,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub depth_buffer: &'a Texture,
}

impl ModelRenderContext<'_> {
    pub fn begin_draw(&mut self) -> wgpu::RenderPass {
        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &self.output,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: &self.depth_buffer.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                }),
                stencil_ops: None,
            }),
        })
    }
}
