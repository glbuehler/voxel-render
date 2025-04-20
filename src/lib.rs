#![allow(dead_code, unused)]

mod background;
mod camera;
mod lattice;
mod state;
mod vertex;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowAttributes,
};

pub struct AppHandler<'a> {
    state: Option<state::State<'a>>,
}

impl AppHandler<'_> {
    pub fn new() -> Self {
        Self { state: None }
    }
}

impl ApplicationHandler for AppHandler<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attributes = WindowAttributes::default();
        let window = event_loop
            .create_window(attributes)
            .expect("Failed to create window");
        window.set_cursor_visible(false);
        window.set_cursor_grab(winit::window::CursorGrabMode::Confined);
        self.state = Some(pollster::block_on(state::State::new(window)));
    }

    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        let state = self
            .state
            .as_mut()
            .expect("Invalid state: window event on uninitialized window");

        state.device_input(event);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = self
            .state
            .as_mut()
            .expect("Invalid state: window event on uninitialized window");

        if window_id != state.window().id() {
            return;
        }

        match event {
            WindowEvent::Focused(focused) => {
                let window = state.window();
                if focused {
                    window.set_cursor_visible(false);
                    window.set_cursor_grab(winit::window::CursorGrabMode::Confined);
                } else {
                    window.set_cursor_visible(true);
                    window.set_cursor_grab(winit::window::CursorGrabMode::None);
                }
            }
            WindowEvent::Resized(size) => state.resize(size),
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(winit::dpi::PhysicalSize {
                        width: state.width(),
                        height: state.height(),
                    }),
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => state.window_input(event),
        };
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = self.state.as_ref().unwrap().window();
        window.request_redraw();
    }
}
