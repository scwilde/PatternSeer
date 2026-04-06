use eframe::{egui, egui_wgpu};
use glam::{Vec2, Vec3};
use crate::camera::Camera;
use crate::pattern::Pattern;
// use crate::grid_renderer::GridRenderer;
use crate::utils::Volatile::{self, *};


// mod grid_renderer;
mod renderer;
mod utils;
mod camera;
mod pattern;

/// An instance of the application.
struct PatternSeer {
    /// The camera used for rendering.
    camera: Volatile<Camera>,
    /// Container for the pattern we are currently working on.
    pattern: Pattern,
}

impl PatternSeer {
    /// Creates a new instance of PatternSeer.
    /// 
    /// # Parameters
    /// 
    /// * 'cc' - `CreationContext` provided by something like `eframe::run_native()`.
    fn new(cc: &eframe::CreationContext) -> Self {
        // GridRenderer::init(cc.wgpu_render_state.as_ref().unwrap());

        let pattern = Pattern { stitched_dimensions: Vec2::new(2000.0, 2000.0) };
        let camera = Camera::new(&pattern);

        PatternSeer {
            camera: Dirty(camera),
            pattern,
        }
    }
}

impl eframe::App for PatternSeer {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame){
        // Central UI pannel
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // Create our rendering canvas filling all available space
            let (canvas_rect, canvas_response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
            if self.camera.inner().viewport != Vec2::new(canvas_rect.width(), canvas_rect.height()) {
                self.camera.dirty_with(|camera| camera.resize(canvas_rect.width(), canvas_rect.height(), &self.pattern));
            }

            // Camera pan and zoom controls
            if canvas_response.dragged_by(egui::PointerButton::Secondary) {
                self.camera.dirty_with(|camera| {
                    camera.pan(canvas_response.drag_delta().x, canvas_response.drag_delta().y)
                });
            }
            if canvas_response.hovered() { 
                let scroll_delta = ui.input(|i| i.smooth_scroll_delta);
                if scroll_delta.y != 0.0 {
                    self.camera.dirty_with(|camera| camera.zoom(scroll_delta.y));
                }
            }
            
            // // Render the canvas
            // self.camera.if_dirty_clean_with(|camera| {
            //     self.camera.inner_mut().limit_pan(&self.pattern);
            //     self.camera.inner_mut().limit_zoom(&self.pattern);

            //     GridRenderer::clear_with_color(Vec3::new(1.0, 1.0, 1.0), frame);
            //     GridRenderer::render_grid(&self.pattern, camera, frame);
            // });
            // let render_callback = GridRenderer::get_render();

            // let callback_shape = egui_wgpu::Callback::new_paint_callback(canvas_rect, render_callback);
            // ui.painter().add(callback_shape);
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
