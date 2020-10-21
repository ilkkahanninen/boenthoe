use crate::engine::prelude::*;

pub fn build(engine: &Engine, asset: &Asset) -> Result<wgpu::ShaderModule, EngineError> {
    let kind = match asset.get_type() {
        AssetType::GlslVertexShader => shaderc::ShaderKind::Vertex,
        AssetType::GlslFragmentShader => shaderc::ShaderKind::Fragment,
        _ => {
            return Err(EngineError::unsupported_asset_format(
                asset,
                "Vertex or fragment shader (GLSL)",
            ))
        }
    };

    let path = asset.path();
    let glsl = asset.to_utf8()?;

    compile_into_spirv(engine, glsl, path, kind).or_else(|error| {
        Err(EngineError::AssetParseError {
            path: path.clone(),
            message: error,
        })
    })
}

fn compile_into_spirv(
    engine: &Engine,
    glsl: &str,
    path: &PathBuf,
    kind: shaderc::ShaderKind,
) -> Result<wgpu::ShaderModule, String> {
    // Acquire compiler
    let mut compiler = match shaderc::Compiler::new() {
        Some(compiler) => compiler,
        None => return Err("Could not acquire shader compiler".into()),
    };

    // Set compile options
    let mut options = match shaderc::CompileOptions::new() {
        Some(options) => options,
        None => return Err("Could not initialize compile options".into()),
    };

    options.set_include_callback(|filename, _, _, _| {
        // TODO: Get rid of this ugly path mangling and implement load_child_asset()
        let mut path = path.clone();
        path.pop();
        let asset = engine.load_asset(&path.join(filename));
        Ok(shaderc::ResolvedInclude {
            content: asset
                .to_utf8()
                .or_else(|error| Err(format!("{:?}", error)))?
                .into(),
            resolved_name: asset.path().to_string_lossy().to_string(),
        })
    });

    // Compile
    let spirv = compiler
        .compile_into_spirv(
            glsl,
            kind,
            &path.file_name().unwrap().to_string_lossy(),
            "main",
            Some(&options),
        )
        .or_else(|err| Err(format!("Shader compilation failed: {:?}", err)))?;
    let shader_data = wgpu::util::make_spirv(spirv.as_binary_u8());
    Ok(engine.device.create_shader_module(shader_data))
}
