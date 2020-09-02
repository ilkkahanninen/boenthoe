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
        }

        Window {
            window,
            event_loop,
            size: properties.size,
        }
    }

    pub fn run<T: 'static>(self, mut engine: engine::Engine<T>) {
        let Window {
            window,
            event_loop,
            size: _,
        } = self;

        engine.init();
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
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,

                            _ => {}
                        },
                        _ => {}
                    }
                }
            }

            Event::RedrawRequested(_) => {
                engine.render();
            }

            Event::MainEventsCleared => {
                window.request_redraw();
            }

            _ => {}
        });
    }
}
