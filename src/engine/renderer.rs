pub trait Renderer<T> {
    fn should_render(&self, _time: f64) -> bool {
        true
    }
    fn update(&mut self, _context: &UpdateContext<T>) {}
    fn render(&self, context: &mut RenderingContext<T>);
}

pub struct UpdateContext<'a, T> {
    pub time: &'a f64,
    pub device: &'a wgpu::Device,
    pub state: &'a T,
    pub screen_size: &'a winit::dpi::PhysicalSize<u32>,
}

pub struct RenderingContext<'a, T> {
    pub device: &'a wgpu::Device,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub output: &'a wgpu::TextureView,
    pub state: &'a T,
    pub screen_size: &'a winit::dpi::PhysicalSize<u32>,
}
