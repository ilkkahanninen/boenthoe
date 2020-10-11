#[macro_use]
extern crate typed_builder;

mod demo;
mod engine;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.2.0", author = "Ilkka Hänninen")]
struct Opts {
    #[clap(short, long)]
    window: bool,
}

fn main() {
    let opts = Opts::parse();

    let mut window = engine::window::Window::new(&engine::window::WindowProperties {
        title: "Boenthoe 0.2.0",
        size: winit::dpi::PhysicalSize {
            width: 1920,
            height: 1080,
        },
        fullscreen: !opts.window,
    });

    match demo::init(&mut window.window) {
        Ok(engine) => window.run(engine),
        Err(err) => panic!("Demo initialization failed:\n\n{:?}", err),
    }
}
