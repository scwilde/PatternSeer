use eframe::{egui, egui_wgpu};
use crate::camera::Camera;
use crate::pattern_renderer::PatternRenderer;
use crate::utils::Triangle;
use crate::utils::Volatile::{self, *};

mod pattern_renderer;
mod utils;
mod camera;


struct PatternSeer {
    triangle: utils::Triangle,
    camera: Camera,
}

impl PatternSeer {
    fn new(cc: &eframe::CreationContext) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();
        PatternRenderer::init(wgpu_render_state);

        PatternSeer {
            triangle: Triangle{vertices: [
                utils::Vertex { position: [0.0, 0.5], color: [1.0, 0.0, 0.0] },
                utils::Vertex { position: [-1.0, -0.5], color: [0.0, 1.0, 0.0] },
                utils::Vertex { position: [1.0, -0.5], color: [0.0, 0.0, 1.0] },
            ]},
            camera: Camera {
                position: [0.0, 0.0],
                viewport: [0.0, 0.0],
                zoom: 20.0,
            },
        }
    }
}

impl eframe::App for PatternSeer {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ui, |ui| {
            let (canvas_rect, canvas_response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
            self.camera.viewport = [canvas_rect.width(), canvas_rect.height()];

            if canvas_response.dragged_by(egui::PointerButton::Secondary) {
                self.camera.pan(canvas_response.drag_delta().x, canvas_response.drag_delta().y)
            }
            if canvas_response.hovered() { 
                let scroll_delta = ui.input(|i| i.smooth_scroll_delta);
                self.camera.zoom(scroll_delta.y);
            }

            let render_callback = PatternRenderer::render(&self.triangle, &self.camera, frame);
            let callback_shape = egui_wgpu::Callback::new_paint_callback(canvas_rect, render_callback);

            ui.painter().add(callback_shape);
        });
    }
}

fn main() -> anyhow::Result<()>{
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("PatternSeer"),
        renderer: eframe::Renderer::Wgpu,
        wgpu_options: eframe::egui_wgpu::WgpuConfiguration::default(),
        ..Default::default()
    };
    eframe::run_native(
        "PatternSeer",
        native_options, Box::new(|cc| { Ok(Box::new(PatternSeer::new(cc))) }
    ))?;
    
    Ok(())
}
