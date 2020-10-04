pub trait Renderer {
    fn should_render(&self, context: &RenderingContext) -> bool {
        true
    }
    fn update(&mut self, context: &mut RenderingContext);
    fn render(&mut self, context: &mut RenderingContext);
}

pub struct RenderingContext<'a> {
    pub device: &'a wgpu::Device,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub output: &'a wgpu::TextureView,
    pub time: f64,
    pub screen_size: &'a winit::dpi::PhysicalSize<u32>,
}
