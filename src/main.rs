mod demo;
mod engine;

fn main() {
    let window = engine::window::Window::new();
    let state = demo::state::create_state(&window.window);
    window.run(state);
}
