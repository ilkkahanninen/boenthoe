use crate::engine::*;

#[derive(Default)]
pub struct PipelineBuilder<'a> {
  vertex_shader: Option<ShaderScript<'a>>,
  fragment_shader: Option<ShaderScript<'a>>,
  vertex_buffer_descriptors: Vec<wgpu::VertexBufferDescriptor<'a>>,
  command_buffers: Vec<wgpu::CommandBuffer>,
  bind_group_layouts: Vec<wgpu::BindGroupLayout>,
}

#[derive(Copy, Clone)]
pub struct ShaderScript<'a> {
  glsl: &'a str,
  label: &'a str,
  kind: shaderc::ShaderKind,
}

impl<'a> PipelineBuilder<'a> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn vertex_shader(mut self, glsl: &'a str, label: &'a str) -> Self {
    self.vertex_shader = Some(ShaderScript {
      glsl,
      label,
      kind: shaderc::ShaderKind::Vertex,
    });
    self
  }

  pub fn fragment_shader(mut self, glsl: &'a str, label: &'a str) -> Self {
    self.fragment_shader = Some(ShaderScript {
      glsl,
      label,
      kind: shaderc::ShaderKind::Fragment,
    });
    self
  }

  pub fn add_vertex_buffer_descriptor(
    mut self,
    vertex_buffer_descriptor: wgpu::VertexBufferDescriptor<'a>,
  ) -> Self {
    self
      .vertex_buffer_descriptors
      .push(vertex_buffer_descriptor);
    self
  }

  pub fn add_command_buffer(mut self, command_buffer: wgpu::CommandBuffer) -> Self {
    self.command_buffers.push(command_buffer);
    self
  }

  pub fn add_command_buffers(mut self, command_buffers: Vec<wgpu::CommandBuffer>) -> Self {
    for command_buffer in command_buffers.into_iter() {
      self.command_buffers.push(command_buffer);
    }
    self
  }

  pub fn add_bind_group_layout(mut self, bind_group_layout: wgpu::BindGroupLayout) -> Self {
    self.bind_group_layouts.push(bind_group_layout);
    self
  }

  pub fn build<T>(self, engine: &engine::Engine<T>) -> wgpu::RenderPipeline {
    let device = &engine.device;

    // Run command buffers (e.g. load textures to gpu)
    println!("Submit command buffers");
    engine.queue.submit(&self.command_buffers);

    // Create pipeline layout and attach bind group layouts to it
    println!("Create layout");
    let render_pipeline_layout = {
      let bind_group_layouts: Vec<&wgpu::BindGroupLayout> =
        self.bind_group_layouts.iter().collect();
      device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &bind_group_layouts,
      })
    };

    println!("Build vertex shader");
    let vertex_shader = self.build_shader(device, &self.vertex_shader);
    println!("Build fragment shader");
    let fragment_shader = self.build_shader(device, &self.vertex_shader);

    // Create pipeline
    let descriptor = wgpu::RenderPipelineDescriptor {
      // Pipeline layout
      layout: &render_pipeline_layout,

      // Vertex stage
      vertex_stage: wgpu::ProgrammableStageDescriptor {
        module: &vertex_shader.expect("Cannot build a pipeline without vertex shader"),
        entry_point: "main",
      },

      // Fragment state
      fragment_stage: match fragment_shader {
        Some(ref module) => Some(wgpu::ProgrammableStageDescriptor {
          module: &module,
          entry_point: "main",
        }),
        None => None,
      },

      // Rasterization stage
      rasterization_state: Some(wgpu::RasterizationStateDescriptor {
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: wgpu::CullMode::Back,
        depth_bias: 0,
        depth_bias_slope_scale: 0.0,
        depth_bias_clamp: 0.0,
      }),

      // Color states
      color_states: &[wgpu::ColorStateDescriptor {
        format: engine.swap_chain_descriptor.format,
        color_blend: wgpu::BlendDescriptor::REPLACE,
        alpha_blend: wgpu::BlendDescriptor::REPLACE,
        write_mask: wgpu::ColorWrite::ALL,
      }],

      // Vertex state
      vertex_state: wgpu::VertexStateDescriptor {
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &self.vertex_buffer_descriptors,
      },

      primitive_topology: wgpu::PrimitiveTopology::TriangleList,
      depth_stencil_state: None,
      sample_count: 1,
      sample_mask: !0,
      alpha_to_coverage_enabled: false,
    };
    println!("Create pipeline {:?}", descriptor);
    let pipeline = device.create_render_pipeline(&descriptor);

    println!("Pipekuedf ");

    pipeline
  }

  fn build_shader(
    &self,
    device: &wgpu::Device,
    shader: &Option<ShaderScript>,
  ) -> Option<wgpu::ShaderModule> {
    shader.map(|s| {
      let mut compiler = shaderc::Compiler::new().expect("Could not acquire shader compiler");
      let spirv = compiler
        .compile_into_spirv(s.glsl, s.kind, s.label, "main", None)
        .expect("Compiling to SPIR-V failed");
      let shader_data = wgpu::read_spirv(std::io::Cursor::new(spirv.as_binary_u8()))
        .expect("Could not convert SPIR-V to shader data");
      device.create_shader_module(&shader_data)
    })
  }
}
