pub trait Renderer<T> {
    fn should_render(&self, _time: f64) -> bool {
        true
    }
    fn render(&mut self, context: &mut RenderingContext<T>);
}

pub struct RenderingContext<'a, T> {
    pub time: &'a f64, // TODO: Deprecate this, state can hold the time
    pub device: &'a wgpu::Device,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub output: &'a wgpu::TextureView,
    pub state: &'a T,
    pub screen_size: &'a winit::dpi::PhysicalSize<u32>,
}
