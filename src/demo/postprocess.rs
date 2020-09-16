use crate::demo::state::State;
use crate::engine::object::Object;
use crate::engine::*;
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct PostProcess {
    pipeline: wgpu::RenderPipeline,
    input: Rc<texture::Texture>,
    vignette: texture::Texture,
    dust: texture::Texture,
    uniforms: UniformsObject,
}

impl PostProcess {
    pub fn new<T>(engine: &engine::Engine<T>, input: Rc<texture::Texture>) -> Box<Self> {
        let mut texture_builder = texture::TextureBuilder::new(engine);
        let vignette = texture_builder.diffuse(include_bytes!("assets/vignette.jpg"), "vignette");
        let dust = texture_builder.diffuse(include_bytes!("assets/dust.jpg"), "dust");

        let uniforms = UniformsObject::new(&engine.device, UniformsModel::default());

        let pipeline = pipeline::PipelineBuilder::new()
            .vertex_shader(include_str!("shaders/layer.vert"), "layer.vert")
            .fragment_shader(include_str!("shaders/postprocess.frag"), "postprocess.frag")
            .bind_objects(&[input.as_ref(), &vignette, &dust, &uniforms])
            .add_command_buffers(texture_builder.command_buffers)
            .build(engine);

        Box::new(Self {
            pipeline,
            input,
            vignette,
            dust,
            uniforms,
        })
    }
}

impl renderer::Renderer<State> for PostProcess {
    fn update(&mut self, ctx: &mut renderer::RenderingContext<State>) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        self.uniforms.model.dust_x = rng.gen_range(-1.0, 1.0);
        self.uniforms.model.dust_y = rng.gen_range(-1.0, 1.0);
        self.uniforms.model.dust_scale = rng.gen_range(-1.0, 1.0);
        self.uniforms.model.dust_opacity = rng.gen_range(0.0, 1.0);
        self.uniforms.model.fade = ctx.state.fade;
        self.uniforms.update(ctx.device, ctx.encoder);
    }

    fn render(&mut self, ctx: &mut renderer::RenderingContext<State>) {
        let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: ctx.output,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.input.bind_group, &[]);
        render_pass.set_bind_group(1, &self.vignette.bind_group, &[]);
        render_pass.set_bind_group(2, &self.dust.bind_group, &[]);
        render_pass.set_bind_group(3, &self.uniforms.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
struct UniformsModel {
    dust_x: f32,
    dust_y: f32,
    dust_scale: f32,
    dust_opacity: f32,
    fade: f32,
}

unsafe impl bytemuck::Pod for UniformsModel {}
unsafe impl bytemuck::Zeroable for UniformsModel {}

struct UniformsObject {
    pub model: UniformsModel,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl UniformsObject {
    pub fn new(device: &wgpu::Device, model: UniformsModel) -> Self {
        let buffer = Self::create_buffer(device, model, true);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size: wgpu::BufferSize::new(
                        std::mem::size_of::<UniformsModel>() as _
                    ),
                },
                count: None,
            }],
            label: Some("uniform_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.slice(..)),
            }],
            label: Some("uniform_bind_group"),
        });

        Self {
            model,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn update(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let buffer = Self::create_buffer(device, self.model, false);
        encoder.copy_buffer_to_buffer(
            &buffer,
            0,
            &self.buffer,
            0,
            std::mem::size_of::<UniformsModel>() as wgpu::BufferAddress,
        );
    }

    fn create_buffer(
        device: &wgpu::Device,
        model: UniformsModel,
        is_destination: bool,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::cast_slice(&[model]),
            usage: wgpu::BufferUsage::UNIFORM
                | if is_destination {
                    wgpu::BufferUsage::COPY_DST
                } else {
                    wgpu::BufferUsage::COPY_SRC
                },
            label: None,
        })
    }
}

impl Object for UniformsObject {
    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
