use eframe::{egui, egui_wgpu};
use crate::camera::Camera;
use crate::pattern_renderer::PatternRenderer;
use crate::utils::Volatile::{self, *};

mod pattern_renderer;
mod utils;
mod camera;


struct PatternSeer {
    triangles: Vec<Vec<utils::Vertex>>,
    active_triangle: Volatile<usize>,
    camera: Camera,
}

impl PatternSeer {
    fn new(cc: &eframe::CreationContext) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();
        PatternRenderer::init(wgpu_render_state);

        PatternSeer {
            triangles: vec![
                vec![
                    utils::Vertex { position: [0.0, 0.5], color: [1.0, 0.0, 0.0] },
                    utils::Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                    utils::Vertex { position: [0.5, -0.5], color: [0.0, 0.0, 1.0] }
                ],
                vec![
                    utils::Vertex { position: [0.0, 0.5], color: [0.0, 1.0, 0.0] },
                    utils::Vertex { position: [-0.5, -0.5], color: [0.0, 0.0, 1.0] },
                    utils::Vertex { position: [0.5, -0.5], color: [1.0, 0.0, 0.0] }
                ],
                vec![
                    utils::Vertex { position: [0.0, 0.5], color: [0.0, 0.0, 1.0] },
                    utils::Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
                    utils::Vertex { position: [0.5, -0.5], color: [0.0, 1.0, 0.0] }
                ],
            ],
            active_triangle: Dirty(0),
            camera: Camera::new(),
        }
    }

    pub fn rotate_right(&mut self) {
        let mut active_idx = self.active_triangle.inner().clone() + 1;
        if active_idx >= self.triangles.len() {
            active_idx = 0;
        }
        self.active_triangle = Dirty(active_idx);
    }
    pub fn rotate_left(&mut self) {
        let mut active_idx = self.active_triangle.inner().clone();
        if active_idx > 0 {
            active_idx -= 1;
        } else {
            active_idx = self.triangles.len() - 1;
        }
        self.active_triangle = Dirty(active_idx);
    }
}

impl eframe::App for PatternSeer {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ui, |ui| {
            // println!("New frame");

            if ui.input(|i| i.key_pressed(egui::Key::ArrowRight)) { self.rotate_right() }
            if ui.input(|i| i.key_pressed(egui::Key::ArrowLeft)) { self.rotate_left() }




            let (canvas_rect, canvas_response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
            if canvas_response.dragged() {
                self.camera.pan(canvas_response.drag_delta().x, canvas_response.drag_delta().y);
            }

            self.active_triangle.if_dirty_clean_with(|active_index| {
                println!("Triangle dirty; Rerendering...");
                PatternRenderer::update(&self.triangles[*active_index], frame);
            });

            let render_callback = PatternRenderer::render();
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
