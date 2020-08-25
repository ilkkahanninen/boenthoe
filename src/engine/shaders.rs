pub fn load_vertex_shader(device: &wgpu::Device, source: &str, name: &str) -> wgpu::ShaderModule {
  load_shader(device, source, name, shaderc::ShaderKind::Vertex)
}

pub fn load_fragment_shader(device: &wgpu::Device, source: &str, name: &str) -> wgpu::ShaderModule {
  load_shader(device, source, name, shaderc::ShaderKind::Fragment)
}

fn load_shader(
  device: &wgpu::Device,
  source: &str,
  name: &str,
  shader_type: shaderc::ShaderKind,
) -> wgpu::ShaderModule {
  let mut compiler = shaderc::Compiler::new().unwrap();
  let spirv = compiler
    .compile_into_spirv(source, shader_type, name, "main", None)
    .unwrap();
  let shader_data = wgpu::read_spirv(std::io::Cursor::new(spirv.as_binary_u8())).unwrap();
  device.create_shader_module(&shader_data)
}
