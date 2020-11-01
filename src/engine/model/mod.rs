mod gltf_model;

use crate::engine::prelude::*;
use gltf_model::GltfModel;

pub fn load(
    engine: &Engine,
    asset: &Asset,
    options: &ModelProperties,
) -> Result<Box<dyn Model>, EngineError> {
    match asset.get_type() {
        AssetType::GltfModel => Ok(Box::new(GltfModel::new(engine, asset, options)?)),
        _ => Err(EngineError::unsupported_asset_format(asset, "GLTF")),
    }
}

#[derive(Debug, Default)]
pub struct ModelProperties {
    pub camera: Option<Camera>,
    pub rendering_mode: RenderingMode,
}

#[derive(Debug)]
pub enum RenderingMode {
    Phong,
    PhongWithNormalMaps,
    PhysicalBasedRendering,
}

impl Default for RenderingMode {
    fn default() -> Self {
        Self::PhysicalBasedRendering
    }
}

impl RenderingMode {
    pub fn uses_normal_maps(&self) -> bool {
        match self {
            Self::PhongWithNormalMaps | Self::PhysicalBasedRendering => true,
            _ => false,
        }
    }
}

pub trait Model {
    fn render(&self, context: &mut ModelRenderContext);

    #[allow(unused_variables)]
    fn set_camera(&mut self, camera: &Camera) {}

    #[allow(unused_variables)]
    fn set_lighting(&mut self, lights: &[Light]) {}
}

pub struct ModelRenderContext<'a> {
    pub device: &'a wgpu::Device,
    pub output: &'a wgpu::TextureView,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub queue: &'a wgpu::Queue,
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
