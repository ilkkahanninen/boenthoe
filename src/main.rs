#![feature(vec_into_raw_parts)]

#[macro_use]
extern crate typed_builder;

mod demo;
mod engine;

struct Args {
    window: bool,
    print_fps: bool,
}

fn main() {
    let mut args = pico_args::Arguments::from_env();
    let args = Args {
        window: args.contains(["-w", "--window"]),
        print_fps: args.contains(["-f", "--fps"]),
    };

    let mut window = engine::window::Window::new(&engine::window::WindowProperties {
        title: "Boenthoe 0.2.0",
        size: winit::dpi::PhysicalSize {
            width: 1280,
            height: 720,
        },
        fullscreen: !args.window,
    });

    match demo::init(&mut window.window) {
        Ok(engine) => window.run(
            engine,
            engine::window::RunOptions {
                print_fps: args.print_fps,
            },
        ),
        Err(err) => panic!("Demo initialization failed:\n\n{:#?}", err),
    }
}
