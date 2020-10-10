use crate::engine::{model::Vertex, object::Object, transform::Transform, *};
use std::path::Path;

pub struct TestEffect {
    pipeline: wgpu::RenderPipeline,
    model: model::Model,
    depth_buffer: textures::Texture,
    view: view::ViewObject,
    instances: storagebuffer::StorageVecObject<InstanceModel>,
    light: storagebuffer::StorageObject<LightModel>,
    script: scripts::Script,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct InstanceModel {
    pub transform: Transform,
}

unsafe impl bytemuck::Zeroable for InstanceModel {}
unsafe impl bytemuck::Pod for InstanceModel {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LightModel {
    pub position: cgmath::Vector3<f32>,
    pub _padding: u32,
    pub color: cgmath::Vector3<f32>,
}

impl Default for LightModel {
    fn default() -> Self {
        Self {
            position: cgmath::Vector3::new(0.0, 10.0, 0.0),
            color: cgmath::Vector3::new(1.0, 1.0, 1.0),
            _padding: 0,
        }
    }
}

unsafe impl bytemuck::Zeroable for LightModel {}
unsafe impl bytemuck::Pod for LightModel {}

impl TestEffect {
    pub fn attach(engine: &engine::Engine) -> Result<(), EngineError> {
        let device = &engine.device;

        let view = view::ViewObject::new(device);
        let instances = storagebuffer::StorageVecObject::new(device, 20);
        let light = storagebuffer::StorageObject::default(device);

        let model =
            model::Model::load_obj_buf(engine, &engine.load_asset(&Path::new("assets/cube.obj")))?;

        let depth_buffer = textures::depth_buffer(engine);

        let vertex_shader = shaders::build(
            device,
            &engine.load_asset(&Path::new("shaders/shader.vert")),
        )?;
        let fragment_shader = shaders::build(
            device,
            &engine.load_asset(&Path::new("shaders/shader.frag")),
        )?;
        let script = scripts::build(&engine.load_asset(&Path::new("assets/camerajump.boe")))?;

        let layout = device.create_pipeline_layout(&pipeline::layout(&vec![
            view.get_layout(),
            model.materials[0]
                .diffuse_texture
                .as_ref()
                .unwrap()
                .get_layout(),
            instances.get_layout(),
            light.get_layout(),
        ]));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex_stage: pipeline::shader_stage(&vertex_shader),
            fragment_stage: Some(pipeline::shader_stage(&fragment_shader)),
            rasterization_state: pipeline::rasterization_state(wgpu::CullMode::Back),
            color_states: &pipeline::color_state(
                engine.swap_chain_descriptor.format,
                pipeline::BlendMode::default(),
            ),
            depth_stencil_state: pipeline::depth_stencil_state(),
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint32,
                vertex_buffers: &[model::ModelVertex::desc()],
            },
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        engine.add_renderer(Box::new(Self {
            pipeline,
            model,
            view,
            instances,
            depth_buffer,
            light,
            script,
        }));

        Ok(())
    }
}

impl renderer::Renderer for TestEffect {
    fn reload_assets(&mut self, assets: &assets::AssetLibrary) -> Result<(), EngineError> {
        if let Some(script) = assets.changed("camerajump.boe") {
            println!("TestEffect: reload script");
            self.script = scripts::build(&script)?;
        }

        Ok(())
    }

    fn update(&mut self, ctx: &mut renderer::RenderingContext) {
        let time = ctx.time as f32;

        self.script.set_time(ctx.time);

        self.view.model.camera.eye = (
            self.script.get("eye_x").to_f() as f32,
            self.script.get("eye_y").to_f() as f32,
            self.script.get("eye_z").to_f() as f32,
        )
            .into();
        self.view.copy_to_gpu(ctx.device, ctx.encoder);

        self.light.data.position.x = (time).sin() * 10.0;
        self.light.data.position.y = 15.0 + (time * 1.3).sin() * 10.0;
        self.light.data.position.z = (time * 1.2).cos() * 10.0;
        self.light.copy_to_gpu(ctx.device, ctx.encoder);

        for (index, instance) in self.instances.data.iter_mut().enumerate() {
            let a = index as f32 + 1.0;
            instance.transform = transform::Transform::new()
                .translate((a * 1.2).sin(), (a * 1.3).cos(), (a * 0.7).sin() - a.cos())
                .rotate(
                    a.sin(),
                    a.cos(),
                    a.sin() - a.cos(),
                    cgmath::Rad(a * time * 0.2),
                )
        }
        self.instances.copy_to_gpu(ctx.device, ctx.encoder);
    }

    fn render(&mut self, ctx: &mut renderer::RenderingContext) {
        let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &ctx.output,
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
        let diffuse_texture = material.diffuse_texture.as_ref().unwrap();

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, self.view.get_bind_group(), &[]);
        render_pass.set_bind_group(1, &diffuse_texture.bind_group, &[]);
        render_pass.set_bind_group(2, self.instances.get_bind_group(), &[]);
        render_pass.set_bind_group(3, self.light.get_bind_group(), &[]);
        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(mesh.index_buffer.slice(..));
        render_pass.draw_indexed(0..mesh.num_elements, 0, self.instances.all());
    }
}
