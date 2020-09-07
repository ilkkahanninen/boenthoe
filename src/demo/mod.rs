use crate::create_state;
use crate::engine::model::Vertex;
use crate::engine::object::Object;
use crate::engine::*;
use crate::include_resources;
use crate::scripting::*;
use futures::executor::block_on;

pub struct State {
    time: f64,
    cam_x: f64,
    cam_y: f64,
    cam_z: f64,
}

#[allow(dead_code)]
struct TestEffect {
    pipeline: wgpu::RenderPipeline,
    model: model::Model,
    depth_buffer: texture::Texture,
    view: view::ViewObject,
    instances: instances::InstanceListObject,
    light: light::LightObject,
}

impl TestEffect {
    fn new<T>(engine: &engine::Engine<T>) -> Box<Self> {
        let device = &engine.device;

        let resources = include_resources!("assets/cube.mtl", "assets/cube-diffuse.jpg");

        let view = view::ViewObject::new(
            device,
            view::ViewModel {
                camera: camera::Camera {
                    eye: (0.0, 0.0, -10.0).into(),
                    target: (0.0, 0.0, 0.0).into(),
                    up: cgmath::Vector3::unit_y(),
                    aspect: engine.size.width as f32 / engine.size.height as f32,
                    fovy: 45.0,
                    znear: 0.1,
                    zfar: 100.0,
                },
            },
        );

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
        let instances =
            instances::InstanceListObject::new(device, vec![instances::InstanceModel::new(); 3]);

        let light = light::LightObject::new(device, light::LightModel::default());

        let pipeline = pipeline::PipelineBuilder::new()
            .enable_depth_stencil_buffer()
            .vertex_shader(include_str!("shaders/shader.vert"), "shader.vert")
            .fragment_shader(include_str!("shaders/shader.frag"), "shader.frag")
            .add_vertex_buffer_descriptor(model::ModelVertex::desc())
            .bind_objects(&[
                &view,
                &model.materials[0].diffuse_texture,
                &instances,
                &light,
            ])
            .add_command_buffers(texture_builder.command_buffers)
            .build(engine);

        Box::new(Self {
            pipeline,
            model,
            view,
            instances,
            depth_buffer,
            light,
        })
    }
}

impl renderer::Renderer<State> for TestEffect {
    fn update(&mut self, ctx: &mut renderer::RenderingContext<State>) {
        let time = ctx.state.time as f32;

        self.view.model.camera.eye = (
            ctx.state.cam_x as f32,
            ctx.state.cam_y as f32,
            ctx.state.cam_z as f32,
        )
            .into();
        self.view.update(ctx.device, ctx.encoder);

        self.light.model.position.x = (time * 0.1).sin() * 10.0;
        self.light.model.position.y = (time * 0.13).sin() * 10.0;
        self.light.model.position.z = (time * 0.12).cos() * 10.0;
        self.light.update(ctx.device, ctx.encoder);

        for (index, instance) in self.instances.models.iter_mut().enumerate() {
            let a = index as f32 + 0.1;
            instance.transform = transform::Transform::new()
                .translate((a * 1.2).sin(), (a * 1.3).cos(), (a * 0.7).sin() - a.cos())
                .rotate(
                    a.sin(),
                    a.cos(),
                    a.sin() - a.cos(),
                    cgmath::Rad(a * time * 0.2),
                )
        }
        self.instances.update(ctx.device, ctx.encoder);
    }

    fn render(&mut self, ctx: &mut renderer::RenderingContext<State>) {
        let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: ctx.output,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.01,
                        g: 0.01,
                        b: 0.01,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: &self.depth_buffer.view,
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
        render_pass.set_bind_group(0, self.view.get_bind_group(), &[]);
        render_pass.set_bind_group(1, &material.diffuse_texture.bind_group, &[]);
        render_pass.set_bind_group(2, self.instances.get_bind_group(), &[]);
        render_pass.set_bind_group(3, self.light.get_bind_group(), &[]);
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(mesh.index_buffer.slice(..));
        render_pass.draw_indexed(0..mesh.num_elements, 0, self.instances.all());
    }
}

pub fn init(window: &mut winit::window::Window) -> engine::Engine<State> {
    let state = create_state!(State {
        time => Envelope::time(),
        cam_x => Envelope::infinite(Envelope::linear(12.0, 4.0, 2.0)),
        cam_y => Envelope::infinite(Envelope::linear(8.0, 6.0, 2.0)),
        cam_z => Envelope::infinite(Envelope::linear(15.0, 7.0, 2.0))
    });

    let mut engine = block_on(engine::Engine::new(window, Box::new(state)));
    engine.add_renderer(TestEffect::new(&engine));

    engine.set_music(include_bytes!("assets/musa.mp3"));

    engine
}
