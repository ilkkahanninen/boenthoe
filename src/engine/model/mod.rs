mod gltf_model;

use crate::engine::{Asset, AssetType, Engine, EngineError};
use gltf_model::GltfModel;

pub fn load(engine: &Engine, asset: &Asset) -> Result<Box<dyn Model>, EngineError> {
    match asset.get_type() {
        AssetType::GltfModel => Ok(Box::new(GltfModel::new(engine, asset)?)),
        _ => Err(EngineError::unsupported_asset_format(asset, "GLTF")),
    }
}

pub trait Model {
    fn render(&self, context: &mut ModelRenderContext);
}

pub struct ModelRenderContext {
    encoder: wgpu::CommandEncoder,
    output: wgpu::TextureView,
}

impl ModelRenderContext {
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
            depth_stencil_attachment: None,
        })
    }
}
