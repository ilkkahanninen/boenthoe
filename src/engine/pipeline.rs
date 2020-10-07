#[allow(dead_code)]
use crate::engine::texture;

// Default descriptors for pipeline creation

pub fn layout<'a>(
    bind_group_layouts: &'a Vec<&'a wgpu::BindGroupLayout>,
) -> wgpu::PipelineLayoutDescriptor<'a> {
    wgpu::PipelineLayoutDescriptor {
        label: Some("Default pipeline layout"),
        bind_group_layouts: &bind_group_layouts,
        push_constant_ranges: &[],
    }
}

pub fn shader_stage<'a>(shader: &'a wgpu::ShaderModule) -> wgpu::ProgrammableStageDescriptor<'a> {
    wgpu::ProgrammableStageDescriptor {
        module: &shader,
        entry_point: "main",
    }
}

pub fn rasterization_state(
    cull_mode: wgpu::CullMode,
) -> Option<wgpu::RasterizationStateDescriptor> {
    Some(wgpu::RasterizationStateDescriptor {
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: cull_mode,
        depth_bias: 0,
        depth_bias_slope_scale: 0.0,
        depth_bias_clamp: 0.0,
        clamp_depth: false,
    })
}

pub fn color_state(
    format: wgpu::TextureFormat,
    blend_mode: BlendMode,
) -> Vec<wgpu::ColorStateDescriptor> {
    let (color_blend, alpha_blend) = blend_mode.get_descriptors();
    vec![wgpu::ColorStateDescriptor {
        format,
        color_blend: color_blend,
        alpha_blend: alpha_blend,
        write_mask: wgpu::ColorWrite::ALL,
    }]
}

pub fn depth_stencil_state() -> Option<wgpu::DepthStencilStateDescriptor> {
    Some(wgpu::DepthStencilStateDescriptor {
        format: texture::DEPTH_FORMAT,
        depth_write_enabled: true,
        depth_compare: wgpu::CompareFunction::Less,
        stencil: wgpu::StencilStateDescriptor {
            front: wgpu::StencilStateFaceDescriptor::IGNORE,
            back: wgpu::StencilStateFaceDescriptor::IGNORE,
            read_mask: 0,
            write_mask: 0,
        },
    })
}

pub fn empty_vertex_state() -> wgpu::VertexStateDescriptor<'static> {
    wgpu::VertexStateDescriptor {
        index_format: wgpu::IndexFormat::Uint32,
        vertex_buffers: &[],
    }
}

// Color blend modes

#[derive(Debug, Copy, Clone)]
pub enum BlendMode {
    Replace,
    Alpha,
    Screen,
}

impl BlendMode {
    fn get_descriptors(&self) -> (wgpu::BlendDescriptor, wgpu::BlendDescriptor) {
        match self {
            BlendMode::Replace => (
                wgpu::BlendDescriptor::REPLACE,
                wgpu::BlendDescriptor::REPLACE,
            ),
            BlendMode::Alpha => (ALPHA_BLEND, ALPHA_BLEND),
            BlendMode::Screen => (SCREEN_BLEND, ALPHA_BLEND),
        }
    }
}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::Replace
    }
}

pub const ALPHA_BLEND: wgpu::BlendDescriptor = wgpu::BlendDescriptor {
    src_factor: wgpu::BlendFactor::SrcAlpha,
    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
    operation: wgpu::BlendOperation::Add,
};

pub const SCREEN_BLEND: wgpu::BlendDescriptor = wgpu::BlendDescriptor {
    src_factor: wgpu::BlendFactor::One,
    dst_factor: wgpu::BlendFactor::OneMinusSrcColor,
    operation: wgpu::BlendOperation::Add,
};
