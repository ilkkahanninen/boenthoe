mod demo;
mod engine;

fn main() {
    let window = engine::window::Window::new();
    let engine = demo::init(&window.window);
    window.run(engine);
}
