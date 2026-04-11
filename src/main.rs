use eframe::{egui, egui_wgpu};
use glam::Vec2;
use crate::camera::Camera;
use crate::pattern::Pattern;
use crate::renderer::RenderContext;

// mod grid_renderer;
mod renderer;
mod utils;
mod camera;
mod pattern;

/// An instance of the application.
struct PatternSeer {
    /// The camera used for rendering.
    camera: Camera,
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
        renderer::init(cc.wgpu_render_state.as_ref().unwrap());

        let pattern = Pattern { stitched_dimensions: Vec2::new(2000.0, 2000.0) };
        let camera = Camera::new(&pattern);

        PatternSeer {
            camera,
            pattern,
        }
    }
}

impl eframe::App for PatternSeer {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame){
        // Central UI pannel
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // ! This is horrible. Rework the Graphics API so we don't have to do this
            let mut wgpu_renderer = frame.wgpu_render_state().unwrap().renderer.write();
            let render_context = wgpu_renderer.callback_resources.get_mut::<RenderContext>().unwrap();

            // Create our rendering canvas filling all available space
            let (canvas_rect, canvas_response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
            self.camera.resize(canvas_rect.width(), canvas_rect.height());

            // Camera pan and zoom controls
            if canvas_response.dragged_by(egui::PointerButton::Secondary) {
                self.camera.pan(canvas_response.drag_delta().x, canvas_response.drag_delta().y);
            }
            if canvas_response.hovered() { 
                let scroll_delta = ui.input(|i| i.smooth_scroll_delta);
                if scroll_delta.y != 0.0 {
                    self.camera.zoom(scroll_delta.y);
                }
            }

            // Stop camera from zooming too far in/out or wandering too far from pattern
            self.camera.limit_pan(&self.pattern);
            self.camera.limit_zoom(&self.pattern);
            
            // Render the canvas
            render_context.rendered_mesh.clear();
            let grid = renderer::grid_renderer::render(render_context, &self.camera, &self.pattern);
            let grid_painter = egui_wgpu::Callback::new_paint_callback(canvas_rect, grid);
            ui.painter().add(grid_painter);
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
