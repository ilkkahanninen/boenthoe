use crate::create_state;
use crate::engine::model::Vertex;
use crate::engine::*;
use crate::scripting::*;
use futures::executor::block_on;
use wgpu::util::DeviceExt;

pub struct State {
    cam_x: f64,
    cam_y: f64,
    cam_z: f64,
}

#[allow(dead_code)]
struct TestEffect {
    pipeline: wgpu::RenderPipeline,
    model: model::Model,
    uniforms: uniforms::Uniforms,
    uniforms_bind_group: wgpu::BindGroup,
    instances: Vec<transform::Transform>,
    instance_buffer: wgpu::Buffer,
    instance_bind_group: wgpu::BindGroup,
    depth_buffer: wgpu::TextureView,
}

impl TestEffect {
    fn new<T>(engine: &engine::Engine<T>) -> Box<Self> {
        let device = &engine.device;

        // TODO: Nice macro to map all resources
        let mut resources = model::Resources::new();
        resources.insert(
            String::from("cube.mtl"),
            include_bytes!("assets/cube.mtl").to_vec(),
        );
        resources.insert(
            String::from("cube-diffuse.jpg"),
            include_bytes!("assets/cube-diffuse.jpg").to_vec(),
        );

        let uniforms = uniforms::Uniforms::new();

        let mut texture_builder = texture::TextureBuilder::new(engine);
        let model = model::Model::load_obj_buf(
            device,
            include_bytes!("assets/cube.obj"),
            &resources,
            &mut texture_builder,
        )
        .expect("Could not load model");

        let depth_buffer = texture_builder.depth_stencil_buffer("depth_buffer");

        // Instance buffer
        let instances = vec![
            transform::Transform::new(),
            transform::Transform::new().translate(5.1, 5.2, 5.3),
            transform::Transform::new().translate(-5.6, -5.5, -5.4),
            transform::Transform::new().translate(5.7, -5.8, 2.2),
            transform::Transform::new().translate(-4.9, 5.9, 1.1),
        ];
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsage::STORAGE,
            label: None,
        });
        let instance_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("instance_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::StorageBuffer {
                        dynamic: false,
                        readonly: true,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let instance_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("instance_bind_group"),
            layout: &instance_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(instance_buffer.slice(..)),
            }],
        });

        let pipeline = pipeline::PipelineBuilder::new()
            .enable_depth_stencil_buffer()
            .vertex_shader(include_str!("shaders/shader.vert"), "shader.vert")
            .fragment_shader(include_str!("shaders/shader.frag"), "shader.frag")
            .add_vertex_buffer_descriptor(model::ModelVertex::desc())
            .add_bind_group_layout(uniforms::Uniforms::create_bind_group_layout(device))
            .add_bind_group_layout(texture_builder.diffuse_bind_group_layout())
            .add_bind_group_layout(instance_bind_group_layout)
            .add_command_buffers(texture_builder.command_buffers)
            .build(engine);

        Box::new(Self {
            pipeline,
            model,
            uniforms,
            uniforms_bind_group: uniforms.create_bind_group(device),
            instances,
            instance_buffer,
            instance_bind_group,
            depth_buffer,
        })
    }
}

impl renderer::Renderer<State> for TestEffect {
    fn update(&mut self, ctx: &renderer::UpdateContext<State>) {
        self.uniforms.update_view_proj(&camera::Camera {
            eye: (
                ctx.state.cam_x as f32,
                ctx.state.cam_y as f32,
                ctx.state.cam_z as f32,
            )
                .into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: ctx.screen_size.width as f32 / ctx.screen_size.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        });
        self.uniforms_bind_group = self.uniforms.create_bind_group(ctx.device);
    }

    fn render(&self, ctx: &mut renderer::RenderingContext<State>) {
        let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: ctx.output,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: &self.depth_buffer,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        let mesh = &self.model.meshes[0];
        let material = &self.model.materials[0];

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.uniforms_bind_group, &[]);
        render_pass.set_bind_group(1, &material.diffuse_texture.bind_group, &[]);
        render_pass.set_bind_group(2, &self.instance_bind_group, &[]);
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(mesh.index_buffer.slice(..));
        render_pass.draw_indexed(0..mesh.num_elements, 0, 0..self.instances.len() as _);
    }
}

pub fn init(window: &mut winit::window::Window) -> engine::Engine<State> {
    let state = create_state!(State {
      cam_x => Envelope::infinite(Envelope::linear(12.0, 4.0, 2.0)),
      cam_y => Envelope::infinite(Envelope::linear(8.0, 6.0, 2.0)),
      cam_z => Envelope::infinite(Envelope::linear(15.0, 7.0, 2.0))
    });

    let mut engine = block_on(engine::Engine::new(window, Box::new(state)));
    engine.add_renderer(TestEffect::new(&engine));

    engine
}
