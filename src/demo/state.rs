use crate::engine::{pipeline::PipelineBuilder, state::State, texture::TextureBuilder};
use futures::executor::block_on;

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

pub fn create_state(window: &winit::window::Window) -> State {
  let mut state = block_on(State::new(window));

  let vertex_buffer = state
    .device
    .create_buffer_with_data(bytemuck::cast_slice(VERTICES), wgpu::BufferUsage::VERTEX);

  let index_buffer = state
    .device
    .create_buffer_with_data(bytemuck::cast_slice(INDICES), wgpu::BufferUsage::INDEX);

  let mut texture_builder = TextureBuilder::new(&mut state);
  let texture = texture_builder.diffuse(include_bytes!("images/jml.png"), "happytree");
  let textures = texture_builder.finish();

  PipelineBuilder::new(&mut state)
    .vertex_shader(include_str!("shaders/shader.vert"), "shader.vert")
    .fragment_shader(include_str!("shaders/shader.frag"), "shader.frag")
    .describe_vertex_buffer(Vertex::desc())
    .textures(textures)
    .render(Box::new(move |a| {
      let mut render_pass = a.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
          attachment: a.target,
          resolve_target: None,
          load_op: wgpu::LoadOp::Clear,
          store_op: wgpu::StoreOp::Store,
          clear_color: wgpu::Color {
            r: 0.9,
            g: 0.7,
            b: 0.1,
            a: 1.0,
          },
        }],
        depth_stencil_attachment: None,
      });

      render_pass.set_pipeline(&a.pipeline);
      render_pass.set_bind_group(0, &texture, &[]);
      render_pass.set_vertex_buffer(0, &vertex_buffer, 0, 0);
      render_pass.set_index_buffer(&index_buffer, 0, 0);
      render_pass.draw_indexed(0..(INDICES.len() as u32), 0, 0..1);
    }))
    .build();

  state
}
