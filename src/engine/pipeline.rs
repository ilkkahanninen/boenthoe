use crate::engine::{
    assets::{Asset, AssetType},
    texture,
};

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

// Shader compiling

pub fn shader(device: &wgpu::Device, asset: &Asset) -> Result<wgpu::ShaderModule, String> {
    let kind = match asset.asset_type {
        AssetType::GlslVertexShader => shaderc::ShaderKind::Vertex,
        AssetType::GlslFragmentShader => shaderc::ShaderKind::Fragment,
        e => return Err(format!("Unsupported asset type: {:?}", e)),
    };
    let glsl = std::str::from_utf8(&asset.data)
        .or_else(|err| Err(format!("UTF-8 error at {}", err.valid_up_to())))?;
    build_shader(device, glsl, &asset.name, kind)
}

fn build_shader(
    device: &wgpu::Device,
    glsl: &str,
    label: &str,
    kind: shaderc::ShaderKind,
) -> Result<wgpu::ShaderModule, String> {
    let mut compiler = match shaderc::Compiler::new() {
        Some(compiler) => compiler,
        None => return Err("Could not acquire shader compiler".into()),
    };
    let spirv = compiler
        .compile_into_spirv(glsl, kind, label, "main", None)
        .or_else(|err| Err(format!("Shader compilation failed: {:?}", err)))?;
    let shader_data = wgpu::util::make_spirv(spirv.as_binary_u8());
    Ok(device.create_shader_module(shader_data))
}
