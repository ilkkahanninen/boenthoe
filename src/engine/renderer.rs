pub trait Renderer<T> {
    fn should_render(&self, context: &RenderingContext<T>) -> bool {
        true
    }
    fn update(&mut self, context: &mut RenderingContext<T>);
    fn render(&mut self, context: &mut RenderingContext<T>);
}

pub struct RenderingContext<'a, T> {
    pub device: &'a wgpu::Device,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub output: &'a wgpu::TextureView,
    pub state: &'a T,
    pub screen_size: &'a winit::dpi::PhysicalSize<u32>,
}
