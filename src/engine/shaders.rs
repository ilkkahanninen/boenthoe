use crate::engine::{assets::*, EngineError};

pub fn build(device: &wgpu::Device, asset: &Asset) -> Result<wgpu::ShaderModule, EngineError> {
    let kind = match asset.get_type() {
        AssetType::GlslVertexShader => shaderc::ShaderKind::Vertex,
        AssetType::GlslFragmentShader => shaderc::ShaderKind::Fragment,
        _ => {
            return Err(EngineError::UnsupportedAssetType {
                path: asset.path().clone(),
                expected: "Vertex or fragment shader (GLSL)".into(),
            })
        }
    };

    let path = asset.path();
    let glsl = asset.to_utf8()?;

    compile_into_spirv(
        device,
        glsl,
        &path.file_name().unwrap().to_string_lossy(),
        kind,
    )
    .or_else(|error| {
        Err(EngineError::AssetParseError {
            path: path.clone(),
            message: error,
        })
    })
}

fn compile_into_spirv(
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
