use crate::engine::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct WindowProperties<'a> {
    pub title: &'a str,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub fullscreen: bool,
}

pub struct Window {
    pub window: winit::window::Window,
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl Window {
    pub fn new(properties: &WindowProperties) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(properties.title)
            .with_resizable(false)
            .with_inner_size(properties.size)
            .build(&event_loop)
            .unwrap();

        if properties.fullscreen {
            window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(
                window.current_monitor(),
            )));
            window.set_cursor_visible(false);
        }

        Window {
            window,
            event_loop,
            size: properties.size,
        }
    }

    pub fn run(self, mut engine: engine::Engine, options: RunOptions) {
        let Window {
            window,
            event_loop,
            size: _,
        } = self;

        let mut options = options;

        engine.init();
        let mut previous_elapsed = 0.0;
        let mut fps_counter = WindowedAverageCounter::new();

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !engine.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode,
                                ..
                            } => match virtual_keycode {
                                Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                                Some(VirtualKeyCode::F) => options.print_fps = !options.print_fps,
                                _ => {}
                            },

                            _ => {}
                        },
                        _ => {}
                    }
                }
            }

            Event::RedrawRequested(_) => {
                // Render frame
                engine.render();

                // Calculate FPS
                if options.print_fps {
                    let elapsed = engine.elapsed();
                    if let Some(avg_time) = fps_counter.push(elapsed - previous_elapsed) {
                        println!("FPS: {}", (1.0 / avg_time).round());
                    }
                    previous_elapsed = elapsed;
                }
            }

            Event::MainEventsCleared => {
                window.request_redraw();
            }

            _ => {}
        });
    }
}

pub struct RunOptions {
    pub print_fps: bool,
}

const FPS_WINDOW_SIZE: usize = 100;
struct WindowedAverageCounter {
    values: [f64; FPS_WINDOW_SIZE],
    count: usize,
}

impl WindowedAverageCounter {
    fn new() -> Self {
        Self {
            values: [0.0; FPS_WINDOW_SIZE],
            count: 0,
        }
    }

    fn push(&mut self, value: f64) -> Option<f64> {
        self.values[self.count] = value;
        self.count += 1;

        if self.count == FPS_WINDOW_SIZE {
            self.count = 0;
            Some(self.values.iter().sum::<f64>() / FPS_WINDOW_SIZE as f64)
        } else {
            None
        }
    }
}
