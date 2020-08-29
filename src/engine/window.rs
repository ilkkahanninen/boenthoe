use crate::engine::*;
use winit::{
  event::*,
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

pub struct Window {
  pub window: winit::window::Window,
  pub event_loop: winit::event_loop::EventLoop<()>,
}

impl Window {
  pub fn new() -> Self {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    Window { window, event_loop }
  }

  pub fn run(self, mut engine: Engine) {
    let Window { window, event_loop } = self;

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
            WindowEvent::Resized(physical_size) => {
              engine.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
              // new_inner_size is &mut so we have to dereference it twice
              engine.resize(**new_inner_size);
            }
            _ => {}
          }
        }
      }

      Event::RedrawRequested(_) => {
        engine.update();
        engine.render();
      }

      Event::MainEventsCleared => {
        window.request_redraw();
      }

      _ => {}
    });
  }
}
