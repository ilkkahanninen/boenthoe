use crate::demo::state::State;
use crate::engine::object::Object;
use crate::engine::*;
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct TitleLayer {
    pipeline: wgpu::RenderPipeline,
    images: Vec<texture::Texture>,
    output: Rc<texture::Texture>,
    uniforms: UniformsObject,
}

impl TitleLayer {
    pub fn new<T>(engine: &engine::Engine<T>, output: Rc<texture::Texture>) -> Box<Self> {
        let mut texture_builder = texture::TextureBuilder::new(engine);
        let images = vec![
            texture_builder.diffuse(include_bytes!("assets/jml-overlay-1.png"), "jml-overlay-1"),
            texture_builder.diffuse(include_bytes!("assets/jml-overlay-2.png"), "jml-overlay-2"),
            texture_builder.diffuse(
                include_bytes!("assets/polkka-overlay-1.png"),
                "polkka-overlay-1",
            ),
            texture_builder.diffuse(
                include_bytes!("assets/polkka-overlay-2.png"),
                "polkka-overlay-2",
            ),
        ];

        let uniforms = UniformsObject::new(
            &engine.device,
            UniformsModel {
                time: 0.0,
                scale: 0.0,
            },
        );

        let pipeline = pipeline::PipelineBuilder::new()
            .vertex_shader(include_str!("shaders/layer.vert"), "layer.vert")
            .fragment_shader(include_str!("shaders/titlelayer.frag"), "titlelayer.frag")
            .bind_objects(&[&uniforms, &images[0], &images[1]])
            .add_command_buffers(texture_builder.command_buffers)
            .set_blend_mode(pipeline::SCREEN_BLEND)
            .build(engine);

        Box::new(Self {
            pipeline,
            images,
            output,
            uniforms,
        })
    }
}

impl renderer::Renderer<State> for TitleLayer {
    fn should_render(&self, context: &renderer::RenderingContext<State>) -> bool {
        let part = context.state.part as u8;
        part == 2 || part == 4
    }

    fn update(&mut self, ctx: &mut renderer::RenderingContext<State>) {
        self.uniforms.model.time = ctx.state.time;
        self.uniforms.model.scale = ctx.state.time + 150.0;
        self.uniforms.update(ctx.device, ctx.encoder);
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
            depth_stencil_attachment: None,
        });

        let (bg1, bg2) = if ctx.state.part <= 3.0 {
            (0, 1)
        } else {
            (2, 3)
        };

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.uniforms.bind_group, &[]);
        render_pass.set_bind_group(1, &self.images[bg1].bind_group, &[]);
        render_pass.set_bind_group(2, &self.images[bg2].bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct UniformsModel {
    time: f32,
    scale: f32,
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
