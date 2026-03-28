use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, ElementState, MouseButton, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowId}
};
use std::sync::Arc;

use crate::{
    camera::{self, Camera},
    canvas::Canvas,
    gpu_state::GpuState
};

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub gpu: Option<GpuState>,
    pub canvas: Option<Canvas>,
    pub camera: Option<Camera>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
        let gpu = GpuState::new(window.clone());
        let camera = Camera::new(window.inner_size().width, window.inner_size().height);

        self.window = Some(window);
        self.gpu = Some(gpu);
        self.camera = Some(camera);
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: WindowId,
            event: WindowEvent,
        ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close button pressed");
                event_loop.exit();
            },
            WindowEvent::Resized(size) => {
                if let Some(gpu) = &mut self.gpu {
                    gpu.resize(size);
                }
            },
            WindowEvent::RedrawRequested => {
                println!("Frame");
                if let Some(gpu) = &mut self.gpu {
                    gpu.render();
                }
            },

            WindowEvent::MouseInput { button, state, .. } => {
                use MouseButton::*;
                use ElementState::*;
                match (button, state) {
                    (Right, Pressed) => {
                        if let Some(camera) = &mut self.camera { camera.panning = true }
                        println!("Started panning camera");
                    },
                    (Right, Released) => {
                        if let Some(camera) = &mut self.camera { camera.panning = false }
                        println!("Stopped panning camera");
                    },
                    _ => ()
                }
            }
            _ => ()
        }
    }

    fn device_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            device_id: winit::event::DeviceId,
            event: DeviceEvent,
        ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                if let Some(camera) = &mut self.camera {
                    if camera.panning {
                        camera.pan(delta.0, delta.1);
                        if let Some(window) = &mut self.window {
                            window.request_redraw();
                        }
                    }
                }
            }
            _ => ()
        }
    }
}