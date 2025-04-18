mod emulator;

use emulator::Emulator;
use std::sync::Arc;
use tracing::info;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalPosition},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

#[derive(Default)]
struct App {
    state: Option<Emulator>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = LogicalSize::new(32 * 10 * 2, 32 * 10 * 2);
        let monitor_size = event_loop
            .primary_monitor()
            .or_else(|| event_loop.available_monitors().next())
            .unwrap()
            .size();

        let position = PhysicalPosition::new(
            monitor_size.width / 2 + size.width / 2,
            monitor_size.height / 2 + size.height / 2,
        );

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Snake")
                        .with_position(position)
                        .with_inner_size(size)
                        .with_min_inner_size(size)
                        .with_resizable(false),
                )
                .unwrap(),
        );
        let state = Emulator::new(window.clone());

        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                info!("Close Requested; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.render(event_loop);
                state.window().request_redraw();
            }
            WindowEvent::Resized(size) => state.resize(size, event_loop),
            WindowEvent::KeyboardInput { event, .. } => {
                state.input(event, event_loop);
            }
            _ => {}
        }
    }
}
