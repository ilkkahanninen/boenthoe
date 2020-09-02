mod demo;
mod engine;
mod scripting;

use clap::Clap;

#[derive(Clap)]
#[clap(
    version = "0.1.0",
    author = "Ilkka Hänninen <ilkka.hanninen@gmail.com>"
)]
struct Opts {
    #[clap(short, long)]
    fullscreen: bool,
}

fn main() {
    let opts = Opts::parse();

    let mut window = engine::window::Window::new(&engine::window::WindowProperties {
        title: "My demo",
        size: winit::dpi::PhysicalSize {
            width: 1920,
            height: 1080,
        },
        fullscreen: opts.fullscreen,
    });

    let engine = demo::init(&mut window.window);
    window.run(engine);
}
