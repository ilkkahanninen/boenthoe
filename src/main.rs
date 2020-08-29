mod demo;
mod engine;
mod scripting;

fn main() {
    let window = engine::window::Window::new();
    let engine = demo::init(&window.window);
    window.run(engine);
}
