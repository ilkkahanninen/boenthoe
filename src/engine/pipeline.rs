#[allow(dead_code)]
use crate::engine::{engine, textures};

#[derive(TypedBuilder)]
pub struct PipelineDescriptor<'a> {
    vertex_shader: &'a wgpu::ShaderModule,

    #[builder(default, setter(strip_option))]
    label: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    fragment_shader: Option<&'a wgpu::ShaderModule>,
    #[builder(default)]
    cull_mode: wgpu::CullMode,
    #[builder(default, setter(strip_option))]
    primitive_topology: Option<wgpu::PrimitiveTopology>,
    #[builder(default)]
    color_format: Option<wgpu::TextureFormat>,
    #[builder(default)]
    blend_mode: BlendMode,
    #[builder(default, setter(strip_option))]
    depth_stencil_state: Option<wgpu::DepthStencilStateDescriptor>,
    #[builder(default)]
    vertex_buffers: &'a [wgpu::VertexBufferDescriptor<'a>],
    #[builder(default = 1)]
    sample_count: u32,
    #[builder(default = 0xffffffff)]
    sample_mask: u32,
    #[builder(default)]
    enable_alpha_to_coverage: bool,
    #[builder(default)]
    enable_depth_buffer: bool,
    #[builder(default)]
    bind_group_layouts: &'a [&'a wgpu::BindGroupLayout],
}

pub fn build_pipeline<'a>(
    engine: &engine::Engine,
    descriptor: PipelineDescriptor<'a>,
) -> wgpu::RenderPipeline {
    let pipeline_layout = engine
        .device
        .create_pipeline_layout(&layout(descriptor.bind_group_layouts));

    engine
        .device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: descriptor.label.or(Some("Render pipeline")),
            layout: Some(&pipeline_layout),
            vertex_stage: shader_stage(&descriptor.vertex_shader),
            fragment_stage: match descriptor.fragment_shader {
                Some(ref shader) => Some(shader_stage(shader)),
                None => None,
            },
            rasterization_state: rasterization_state(descriptor.cull_mode),
            primitive_topology: descriptor
                .primitive_topology
                .unwrap_or(wgpu::PrimitiveTopology::TriangleList),
            color_states: &color_state(
                descriptor
                    .color_format
                    .unwrap_or(engine.swap_chain_descriptor.format),
                descriptor.blend_mode,
            ),
            depth_stencil_state: if descriptor.enable_depth_buffer {
                depth_stencil_state()
            } else {
                None
            },
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &descriptor.vertex_buffers,
            },
            sample_count: descriptor.sample_count,
            sample_mask: descriptor.sample_mask,
            alpha_to_coverage_enabled: descriptor.enable_alpha_to_coverage,
        })
}

// Default descriptors for pipeline creation

pub fn layout<'a>(
    bind_group_layouts: &'a [&'a wgpu::BindGroupLayout],
) -> wgpu::PipelineLayoutDescriptor<'a> {
    wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline layout"),
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
        color_blend,
        alpha_blend,
        write_mask: wgpu::ColorWrite::ALL,
    }]
}

pub fn depth_stencil_state() -> Option<wgpu::DepthStencilStateDescriptor> {
    Some(wgpu::DepthStencilStateDescriptor {
        format: textures::DEPTH_FORMAT,
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
