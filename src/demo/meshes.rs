use crate::demo::state::State;
use crate::engine::model::Vertex;
use crate::engine::object::Object;
use crate::engine::*;
use crate::include_resources;
use std::rc::Rc;

pub struct Meshes {
    pipeline: wgpu::RenderPipeline,
    models: Vec<model::Model>,
    depth_buffer: texture::Texture,
    view: view::ViewObject,
    instances: instances::InstanceListObject,
    light: light::LightObject,
    environment_map: Rc<texture::Texture>,
    output: Rc<texture::Texture>,
}

impl Meshes {
    pub fn new<T>(
        engine: &engine::Engine<T>,
        environment_map: Rc<texture::Texture>,
        output: Rc<texture::Texture>,
    ) -> Box<Self> {
        let device = &engine.device;

        let resources = include_resources!(
            "assets/viulu.mtl",
            "assets/acordion.mtl",
            "assets/puukko.mtl",
            "assets/jussipaita.mtl",
            "assets/talla.mtl"
        );

        let view = view::ViewObject::new(
            device,
            view::ViewModel {
                camera: camera::Camera {
                    eye: (0.0, -1.0, 0.0).into(),
                    target: (0.0, 0.0, 0.0).into(),
                    up: cgmath::Vector3::unit_z() * -1.0,
                    aspect: engine.size.width as f32 / engine.size.height as f32,
                    fovy: 45.0,
                    znear: 0.1,
                    zfar: 100.0,
                },
            },
        );

        let mut texture_builder = texture::TextureBuilder::new(engine);

        let accordion = model::Model::load_obj_buf(
            device,
            include_bytes!("assets/acordion.obj"),
            &resources,
            &mut texture_builder,
        )
        .expect("Could not load accordion");

        let viulu = model::Model::load_obj_buf(
            device,
            include_bytes!("assets/viulu.obj"),
            &resources,
            &mut texture_builder,
        )
        .expect("Could not load viulu");

        let puukko = model::Model::load_obj_buf(
            device,
            include_bytes!("assets/puukko.obj"),
            &resources,
            &mut texture_builder,
        )
        .expect("Could not load puukko");

        let talla = model::Model::load_obj_buf(
            device,
            include_bytes!("assets/talla.obj"),
            &resources,
            &mut texture_builder,
        )
        .expect("Could not load talla");

        let jussipaita = model::Model::load_obj_buf(
            device,
            include_bytes!("assets/jussipaita.obj"),
            &resources,
            &mut texture_builder,
        )
        .expect("Could not load jussipaita");

        let depth_buffer = texture_builder.depth_stencil_buffer("depth_buffer");

        let instances =
            instances::InstanceListObject::new(device, vec![instances::InstanceModel::new(); 1]);

        let light = light::LightObject::new(device, light::LightModel::default());

        let pipeline = pipeline::PipelineBuilder::new()
            .enable_depth_stencil_buffer()
            .vertex_shader(include_str!("shaders/shader.vert"), "shader.vert")
            .fragment_shader(include_str!("shaders/shader.frag"), "shader.frag")
            .add_vertex_buffer_descriptor(model::ModelVertex::desc())
            .bind_objects(&[&view, environment_map.as_ref(), &instances, &light])
            .add_command_buffers(texture_builder.command_buffers)
            .set_cull_mode(wgpu::CullMode::None)
            .build(engine);

        Box::new(Self {
            pipeline,
            models: vec![accordion, viulu, talla, puukko, jussipaita],
            view,
            instances,
            depth_buffer,
            light,
            output,
            environment_map,
        })
    }
}

impl renderer::Renderer<State> for Meshes {
    fn should_render(&self, context: &renderer::RenderingContext<State>) -> bool {
        let part = context.state.part;
        part >= 5.0 && part < 17.0
    }

    fn update(&mut self, ctx: &mut renderer::RenderingContext<State>) {
        let time = ctx.state.time as f32;

        self.view.update(ctx.device, ctx.encoder);

        self.light.model.position.x = (time * 3.0).sin() * 10.0;
        self.light.model.position.y = -5.0;
        self.light.model.position.z = (time * 2.0).cos() * 10.0;
        self.light.update(ctx.device, ctx.encoder);

        for (_index, instance) in self.instances.models.iter_mut().enumerate() {
            instance.transform = transform::Transform::new()
                .rotate(time.sin(), 0.0, 8.0, cgmath::Rad(ctx.state.rotation))
                .scale(0.2 + ctx.state.mesh_pumping_factor * ctx.state.strobe)
        }
        self.instances.update(ctx.device, ctx.encoder);
    }

    fn render(&mut self, ctx: &mut renderer::RenderingContext<State>) {
        let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &self.output.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
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

        let mesh = &self.models[ctx.state.part as usize % self.models.len()].meshes[0];

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, self.view.get_bind_group(), &[]);
        render_pass.set_bind_group(1, self.environment_map.get_bind_group(), &[]);
        render_pass.set_bind_group(2, self.instances.get_bind_group(), &[]);
        render_pass.set_bind_group(3, self.light.get_bind_group(), &[]);
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(mesh.index_buffer.slice(..));
        render_pass.draw_indexed(0..mesh.num_elements, 0, self.instances.all());
    }
}