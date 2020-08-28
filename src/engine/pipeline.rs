use crate::engine::{shaders, state, texture};

pub struct PipelineBuilder<'a> {
  state: &'a mut state::State,
  vertex_shader: Option<wgpu::ShaderModule>,
  fragment_shader: Option<wgpu::ShaderModule>,
  vertex_buffer_descriptors: Vec<wgpu::VertexBufferDescriptor<'a>>,
  pipeline_textures: Vec<texture::PipelineTexture>,
  render: Option<Box<dyn Fn(state::RenderArgs) -> ()>>,
}

impl<'a> PipelineBuilder<'a> {
  pub fn new(state: &'a mut state::State) -> Self {
    PipelineBuilder {
      state,
      vertex_shader: None,
      fragment_shader: None,
      vertex_buffer_descriptors: vec![],
      pipeline_textures: vec![],
      render: None,
    }
  }

  pub fn vertex_shader(mut self, glsl_source: &str, name: &str) -> Self {
    self.vertex_shader = Some(shaders::load_vertex_shader(
      &self.state.device,
      glsl_source,
      name,
    ));
    self
  }

  pub fn fragment_shader(mut self, glsl_source: &str, name: &str) -> Self {
    self.fragment_shader = Some(shaders::load_fragment_shader(
      &self.state.device,
      glsl_source,
      name,
    ));
    self
  }

  pub fn describe_vertex_buffer(mut self, descriptor: wgpu::VertexBufferDescriptor<'a>) -> Self {
    self.vertex_buffer_descriptors.push(descriptor);
    self
  }

  pub fn textures(mut self, pipeline_textures: Vec<texture::PipelineTexture>) -> Self {
    self.pipeline_textures = pipeline_textures;
    self
  }

  pub fn render(mut self, render_fn: Box<dyn Fn(state::RenderArgs) -> ()>) -> Self {
    self.render = Some(render_fn);
    self
  }

  pub fn build(self) -> Self {
    let PipelineBuilder {
      state,
      vertex_shader,
      fragment_shader,
      vertex_buffer_descriptors,
      pipeline_textures,
      render,
    } = self;

    println!("Load {} textures", pipeline_textures.len());

    let render_pipeline_layout = {
      let bind_group_layouts: Vec<&wgpu::BindGroupLayout> = pipeline_textures
        .iter()
        .map(|t| &t.bind_group_layout)
        .collect();

      state
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
          bind_group_layouts: &bind_group_layouts,
        })
    };

    // Load textures to GPU
    {
      let command_buffers: Vec<wgpu::CommandBuffer> = pipeline_textures
        .into_iter()
        .map(|t| t.command_buffer)
        .collect();
      state.queue.submit(&command_buffers);
    }

    let render_pipeline = state
      .device
      .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        // Pipeline layout
        layout: &render_pipeline_layout,

        // Vertex stage
        vertex_stage: wgpu::ProgrammableStageDescriptor {
          module: &vertex_shader.expect("Vertex shader is undefined"),
          entry_point: "main",
        },

        // Fragment state
        fragment_stage: match fragment_shader {
          Some(ref fs_module) => Some(wgpu::ProgrammableStageDescriptor {
            module: fs_module,
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
          format: state.sc_desc.format,
          color_blend: wgpu::BlendDescriptor::REPLACE,
          alpha_blend: wgpu::BlendDescriptor::REPLACE,
          write_mask: wgpu::ColorWrite::ALL,
        }],

        // Vertex state
        vertex_state: wgpu::VertexStateDescriptor {
          index_format: wgpu::IndexFormat::Uint16,
          vertex_buffers: &vertex_buffer_descriptors,
        },

        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        depth_stencil_state: None,
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
      });

    state.rendering_contexts.push(state::RenderingContext {
      pipeline: render_pipeline,
      render: render.expect("Rendering function is not defined"),
    });

    PipelineBuilder::new(state)
  }
}
