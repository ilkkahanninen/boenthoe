pub trait Renderer<T> {
  fn should_render(&self, time: f64) -> bool {
    true
  }
  fn update(&mut self, context: &UpdateContext<T>) {}
  fn render(&self, context: &mut RenderingContext<T>);
}

pub struct UpdateContext<'a, T> {
  pub time: &'a f64,
  pub device: &'a wgpu::Device,
  pub state: &'a T,
}

pub struct RenderingContext<'a, T> {
  pub device: &'a wgpu::Device,
  pub encoder: &'a mut wgpu::CommandEncoder,
  pub output: &'a wgpu::TextureView,
  pub state: &'a T,
}
