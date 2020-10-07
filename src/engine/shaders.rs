use crate::engine::assets::*;

pub fn build(device: &wgpu::Device, asset: &Asset) -> Result<wgpu::ShaderModule, String> {
    let kind = match asset.get_type() {
        AssetType::GlslVertexShader => shaderc::ShaderKind::Vertex,
        AssetType::GlslFragmentShader => shaderc::ShaderKind::Fragment,
        e => return Err(format!("Unsupported asset type: {:?}", e)),
    };
    match asset {
        Asset::Ready { name, data, .. } => {
            let glsl = std::str::from_utf8(data)
                .or_else(|err| Err(format!("UTF-8 error at {}", err.valid_up_to())))?;
            compile_into_spirv(device, glsl, name, kind)
        }
        _ => Err("Asset is not ready".into()),
    }
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
