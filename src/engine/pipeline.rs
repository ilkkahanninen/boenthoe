use crate::engine::*;

pub struct PipelineBuilder<'a> {
  engine: &'a mut Engine,
  vertex_shader: Option<wgpu::ShaderModule>,
  fragment_shader: Option<wgpu::ShaderModule>,
  vertex_buffer_descriptors: Vec<wgpu::VertexBufferDescriptor<'a>>,
  pipeline_textures: Vec<texture::PipelineTexture>,
  camera: camera::Camera,
  uniforms: uniforms::Uniforms,
  render: Option<Box<dyn Fn(RenderFnContext, f64) -> ()>>,
}

impl<'a> PipelineBuilder<'a> {
  pub fn new(engine: &'a mut Engine) -> Self {
    let aspect = engine.get_aspect_ratio();

    PipelineBuilder {
      engine,
      vertex_shader: None,
      fragment_shader: None,
      vertex_buffer_descriptors: vec![],
      pipeline_textures: vec![],
      camera: camera::Camera {
        eye: (0.0, 1.0, 2.0).into(),
        target: (0.0, 0.0, 0.0).into(),
        up: cgmath::Vector3::unit_y(),
        aspect,
        fovy: 45.0,
        znear: 0.1,
        zfar: 100.0,
      },
      uniforms: uniforms::Uniforms::new(),
      render: None,
    }
  }

  pub fn vertex_shader(mut self, glsl_source: &str, name: &str) -> Self {
    self.vertex_shader = Some(shaders::load_vertex_shader(
      &self.engine.device,
      glsl_source,
      name,
    ));
    self
  }

  pub fn fragment_shader(mut self, glsl_source: &str, name: &str) -> Self {
    self.fragment_shader = Some(shaders::load_fragment_shader(
      &self.engine.device,
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

  pub fn render(mut self, render_fn: Box<dyn Fn(RenderFnContext, f64) -> ()>) -> Self {
    self.render = Some(render_fn);
    self
  }

  pub fn build(self) -> Self {
    let PipelineBuilder {
      engine,
      vertex_shader,
      fragment_shader,
      vertex_buffer_descriptors,
      pipeline_textures,
      camera,
      mut uniforms,
      render,
    } = self;

    // Uniforms

    uniforms.update_view_proj(&camera);
    let (uniform_bind_group_layout, uniform_bind_group) = uniforms.create_bind_group(engine);

    // Create render pipeline layout

    let render_pipeline_layout = {
      // Default uniforms get position 0
      let mut bind_group_layouts: Vec<&wgpu::BindGroupLayout> = vec![&uniform_bind_group_layout];

      // Textures will get positions 1..
      for t in pipeline_textures.iter() {
        bind_group_layouts.push(&t.bind_group_layout);
      }

      engine
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
      engine.queue.submit(&command_buffers);
    }

    // Create pipeline

    let render_pipeline = engine
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
          format: engine.sc_desc.format,
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

    engine.rendering_contexts.push(RenderingContext {
      pipeline: render_pipeline,
      uniform_bind_group,
      render: render.expect("Rendering function is not defined"),
    });

    PipelineBuilder::new(engine)
  }
}
