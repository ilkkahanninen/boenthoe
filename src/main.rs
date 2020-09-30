mod demo;
mod engine;
mod scripting;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "6.6.6", author = "Jumalauta - Money and more")]
struct Opts {
    #[clap(short, long)]
    window: bool,
}

fn main() {
    let opts = Opts::parse();

    let mut window = engine::window::Window::new(&engine::window::WindowProperties {
        title: "Jumalauta Folk Music Committee - Haermaen tappelupolkka",
        size: winit::dpi::PhysicalSize {
            width: 1920,
            height: 1080,
        },
        fullscreen: !opts.window,
    });

    let engine = demo::init(&mut window.window);
    window.run(engine);
}
