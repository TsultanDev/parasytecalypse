use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{self, EventLoop},
    window::{Window, WindowAttributes},
};

#[derive(Default)]
pub struct Application {
    main_window: Option<Arc<Window>>,
}
impl Application {
    pub fn initialize() {
        let event_loop = EventLoop::new().expect("App: Failed to create event loop!!");

        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let mut app = Application::default();

        let result = event_loop.run_app(&mut app);
        match result {
            Ok(_) => println!("App Exit Successfully!!"),
            Err(e) => println!("App Exit Unsuccessfully!!. Error: {}", e),
        };
    }
}
impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        self.main_window = Some(Arc::new(
            event_loop
                .create_window(WindowAttributes::default().with_title("Luxarust"))
                .expect("Failed to create window!!"),
        ));
    }
    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.main_window.as_mut().unwrap().request_redraw();
            }
            _ => (),
        }
    }
    fn exiting(&mut self, event_loop: &event_loop::ActiveEventLoop) {}
}
