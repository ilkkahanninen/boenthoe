use crate::demo::state::State;
use crate::engine::object::Object;
use crate::engine::*;
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct Background {
    pipeline: wgpu::RenderPipeline,
    uniforms: BackgroundObject,
    images: [texture::Texture; 9],
    output: Rc<texture::Texture>,
}

impl Background {
    pub fn new<T>(engine: &engine::Engine<T>, output: Rc<texture::Texture>) -> Box<Self> {
        let mut texture_builder = texture::TextureBuilder::new(engine);
        let images: [texture::Texture; 9] = [
            texture_builder.diffuse(include_bytes!("assets/kuosi1.jpg"), "kuosi1.jpg"),
            texture_builder.diffuse(include_bytes!("assets/kuosi2.jpg"), "kuosi2.jpg"),
            texture_builder.diffuse(include_bytes!("assets/kuosi3.jpg"), "kuosi3.jpg"),
            texture_builder.diffuse(include_bytes!("assets/kuosi4.jpg"), "kuosi4.jpg"),
            texture_builder.diffuse(include_bytes!("assets/kuosi5.jpg"), "kuosi5.jpg"),
            texture_builder.diffuse(include_bytes!("assets/kuosi6.jpg"), "kuosi6.jpg"),
            texture_builder.diffuse(include_bytes!("assets/kuosi7.jpg"), "kuosi7.jpg"),
            texture_builder.diffuse(include_bytes!("assets/kuosi8.jpg"), "kuosi8.jpg"),
            texture_builder.diffuse(include_bytes!("assets/kuosi9.jpg"), "kuosi9.jpg"),
        ];
        let uniforms = BackgroundObject::new(
            &engine.device,
            BackgroundModel {
                zoom: 0.5,
                brightness: 0.5,
                lsd: 0.0,
                fade: 0.0,
            },
        );

        let pipeline = pipeline::PipelineBuilder::new()
            .vertex_shader(include_str!("shaders/background.vert"), "background.vert")
            .fragment_shader(include_str!("shaders/background.frag"), "background.frag")
            .bind_objects(&[&images[0], &uniforms])
            .add_command_buffers(texture_builder.command_buffers)
            .build(engine);

        Box::new(Self {
            pipeline,
            uniforms,
            images,
            output,
        })
    }
}

impl renderer::Renderer<State> for Background {
    fn update(&mut self, ctx: &mut renderer::RenderingContext<State>) {
        self.uniforms.model.zoom = ctx.state.time.sin() as f32 * 0.25 + 0.25;
        self.uniforms.model.lsd = ctx.state.speed.powf(2.0) - 1.0;
        self.uniforms.model.fade = ctx.state.fade;
        if ctx.state.part >= 5.0 && ctx.state.part < 17.0 {
            self.uniforms.model.brightness = ctx.state.strobe;
        }
        self.uniforms.update(ctx.device, ctx.encoder);
    }

    fn render(&mut self, ctx: &mut renderer::RenderingContext<State>) {
        let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &self.output.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        let index = ctx.state.part as usize % self.images.len();

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.images[index].bind_group, &[]);
        render_pass.set_bind_group(1, &self.uniforms.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct BackgroundModel {
    zoom: f32,
    brightness: f32,
    lsd: f32,
    fade: f32,
}

unsafe impl bytemuck::Pod for BackgroundModel {}
unsafe impl bytemuck::Zeroable for BackgroundModel {}

struct BackgroundObject {
    pub model: BackgroundModel,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl BackgroundObject {
    pub fn new(device: &wgpu::Device, model: BackgroundModel) -> Self {
        let buffer = Self::create_buffer(device, model, true);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size: wgpu::BufferSize::new(
                        std::mem::size_of::<BackgroundModel>() as _
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
            std::mem::size_of::<BackgroundModel>() as wgpu::BufferAddress,
        );
    }

    fn create_buffer(
        device: &wgpu::Device,
        model: BackgroundModel,
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

impl Object for BackgroundObject {
    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
