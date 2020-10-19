#[macro_use]
extern crate typed_builder;

mod demo;
mod engine;

struct Args {
    window: bool,
}

fn main() {
    let mut args = pico_args::Arguments::from_env();
    let args = Args {
        window: args.contains(["-w", "--window"]),
    };

    let mut window = engine::window::Window::new(&engine::window::WindowProperties {
        title: "Boenthoe 0.2.0",
        size: winit::dpi::PhysicalSize {
            width: 1920,
            height: 1080,
        },
        fullscreen: !args.window,
    });

    match demo::init(&mut window.window) {
        Ok(engine) => window.run(engine),
        Err(err) => panic!("Demo initialization failed:\n\n{:?}", err),
    }
}
