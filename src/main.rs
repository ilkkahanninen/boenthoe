mod demo;
mod engine;
mod scripting;

fn main() {
    let mut window = engine::window::Window::new(&engine::window::WindowProperties {
        title: "My demo",
        size: winit::dpi::PhysicalSize {
            width: 1920,
            height: 1080,
        },
        fullscreen: false,
    });

    let engine = demo::init(&mut window.window);
    window.run(engine);
}
