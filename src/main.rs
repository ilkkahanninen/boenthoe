mod demo;
mod engine;
mod scripting;

fn main() {
    let mut window = engine::window::Window::new();
    let engine = demo::init(&mut window.window);
    window.run(engine);
}
