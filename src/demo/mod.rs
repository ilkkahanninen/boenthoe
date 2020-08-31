use crate::create_state;
use crate::engine::*;
use crate::scripting::*;
use futures::executor::block_on;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
  position: [f32; 3],
  tex_coords: [f32; 2],
}

impl Vertex {
  fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
    use std::mem;
    wgpu::VertexBufferDescriptor {
      stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
      step_mode: wgpu::InputStepMode::Vertex,
      attributes: &[
        wgpu::VertexAttributeDescriptor {
          offset: 0,
          shader_location: 0,
          format: wgpu::VertexFormat::Float3,
        },
        wgpu::VertexAttributeDescriptor {
          offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
          shader_location: 1,
          format: wgpu::VertexFormat::Float2,
        },
      ],
    }
  }
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

const VERTICES: &[Vertex] = &[
  Vertex {
    position: [-0.0868241, 0.49240386, 0.0],
    tex_coords: [0.4131759, 0.99240386],
  }, // A
  Vertex {
    position: [-0.49513406, 0.06958647, 0.0],
    tex_coords: [0.0048659444, 0.56958646],
  }, // B
  Vertex {
    position: [-0.21918549, -0.44939706, 0.0],
    tex_coords: [0.28081453, 0.050602943],
  }, // C
  Vertex {
    position: [0.35966998, -0.3473291, 0.0],
    tex_coords: [0.85967, 0.15267089],
  }, // D
  Vertex {
    position: [0.44147372, 0.2347359, 0.0],
    tex_coords: [0.9414737, 0.7347359],
  }, // E
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

pub struct State {
  x: f64,
  y: f64,
}

struct TestEffect {
  pipeline: wgpu::RenderPipeline,
  vertex_buffer: wgpu::Buffer,
  index_buffer: wgpu::Buffer,
  // texture: wgpu::BindGroup,
  // uniforms: uniforms::Uniforms,
  // uniforms_bind_group: wgpu::BindGroup,
}

impl TestEffect {
  fn new<T>(engine: &engine::Engine<T>) -> Box<Self> {
    let device = &engine.device;

    // let uniforms = uniforms::Uniforms::new();

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      contents: bytemuck::cast_slice(VERTICES),
      usage: wgpu::BufferUsage::VERTEX,
      label: Some("vertex_buffer"),
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      contents: bytemuck::cast_slice(INDICES),
      usage: wgpu::BufferUsage::INDEX,
      label: Some("index_buffer"),
    });

    // let mut texture_builder = texture::TextureBuilder::new(engine);
    // let texture = texture_builder.diffuse(include_bytes!("images/jml.png"), "happytree");

    let pipeline = pipeline::PipelineBuilder::new()
      .vertex_shader(include_str!("shaders/shader.vert"), "shader.vert")
      .fragment_shader(include_str!("shaders/shader.frag"), "shader.frag")
      .add_vertex_buffer_descriptor(Vertex::desc())
      // .add_bind_group_layout(uniforms::Uniforms::create_bind_group_layout(device))
      // .add_bind_group_layout(texture_builder.diffuse_bind_group_layout())
      // .add_command_buffers(texture_builder.command_buffers)
      .build(engine);

    Box::new(Self {
      pipeline,
      vertex_buffer,
      index_buffer,
      // texture,
      // uniforms,
      // uniforms_bind_group: uniforms.create_bind_group(device),
    })
  }
}

impl renderer::Renderer<State> for TestEffect {
  fn update(&mut self, ctx: &renderer::UpdateContext<State>) {
    // self.uniforms_bind_group = self.uniforms.create_bind_group(ctx.device);
  }

  fn render(&self, ctx: &mut renderer::RenderingContext<State>) {
    // let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //   color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
    //     attachment: ctx.output,
    //     resolve_target: None,
    //     load_op: wgpu::LoadOp::Clear,
    //     store_op: wgpu::StoreOp::Store,
    //     clear_color: wgpu::Color {
    //       r: 0.1,
    //       g: 0.15,
    //       b: 0.2,
    //       a: 1.0,
    //     },
    //   }],
    //   depth_stencil_attachment: None,
    // });

    // render_pass.set_pipeline(&self.pipeline);
    // render_pass.set_bind_group(0, &self.uniforms_bind_group, &[]);
    // render_pass.set_bind_group(1, &self.texture, &[]);
    // render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
    // render_pass.set_index_buffer(&self.index_buffer, 0, 0);
    // render_pass.draw_indexed(0..(INDICES.len() as u32), 0, 0..1);
  }
}

pub fn init(window: &winit::window::Window) -> engine::Engine<State> {
  let state = create_state!(State {
    x => Envelope::linear(64.0, 0.0, 1.0),
    y => Envelope::linear(64.0, 1.0, 0.0)
  });

  let mut engine = block_on(engine::Engine::new(window, Box::new(state)));
  engine.add_renderer(TestEffect::new(&engine));

  engine
}
