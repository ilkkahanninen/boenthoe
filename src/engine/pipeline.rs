use crate::engine::{shaders, state};

pub struct PipelineBuilder<'a> {
  state: &'a mut state::State,
  vertex_shader: Option<wgpu::ShaderModule>,
  fragment_shader: Option<wgpu::ShaderModule>,
}

impl<'a> PipelineBuilder<'a> {
  pub fn new(state: &'a mut state::State) -> Self {
    PipelineBuilder {
      state,
      vertex_shader: None,
      fragment_shader: None,
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

  pub fn build(self) -> Self {
    let PipelineBuilder {
      state,
      vertex_shader,
      fragment_shader,
    } = self;

    let render_pipeline_layout =
      state
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
          bind_group_layouts: &[],
        });

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

        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
          index_format: wgpu::IndexFormat::Uint16,
          vertex_buffers: &[],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
      });

    state.render_pipelines.push(render_pipeline);

    PipelineBuilder::new(state)
  }
}
