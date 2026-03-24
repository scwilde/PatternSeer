use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId}
};
use std::sync::Arc;

use crate::gpu_state::GpuState;


#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub gpu: Option<GpuState>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
        let gpu = GpuState::new(window.clone());

        self.window = Some(window);
        self.gpu = Some(gpu);
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: WindowId,
            event: WindowEvent,
        ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close button pressed!");
                event_loop.exit();
            },
            WindowEvent::Resized(size) => {
                // ! why are we checking this?
                if let Some(gpu) = &mut self.gpu {
                    gpu.resize(size);
                }
            },
            WindowEvent::RedrawRequested => {
                if let Some(gpu) = &mut self.gpu {
                    gpu.render();
                }
            },
            _ => ()
        }
    }
}