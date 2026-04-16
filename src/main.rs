use std::path::PathBuf;
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
    /// Path to the currently opened file.
    open_file: Option<PathBuf>
}
impl PatternSeer {
    /// Creates a new instance of PatternSeer.
    ///
    /// # Parameters
    ///
    /// * 'cc' - `CreationContext` provided by something like `eframe::run_native()`.
    fn new(cc: &eframe::CreationContext) -> Self {
        renderer::init(cc.wgpu_render_state.as_ref().unwrap());

        let pattern = Pattern { stitched_dimensions: Vec2::new(20000.0, 20000.0) };
        let camera = Camera::new(&pattern);

        PatternSeer {
            camera,
            pattern,
            open_file: None,
        }
    }
}
impl eframe::App for PatternSeer {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame){
        egui::Panel::top("Menu Bar").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() && let Some(path) = rfd::FileDialog::new()
                        .add_filter("PatternSeer Pattern", &["psp"])
                        .save_file() {
                            self.open_file = Some(path);
                            println!("Saving new file: {:?}", self.open_file);
                        }
                    if ui.button("Open").clicked() && let Some(path) = rfd::FileDialog::new()
                        .add_filter("PatternSeer Pattern", &["psp"])
                        .pick_file() {
                            self.open_file = Some(path);
                            println!("Opening file: {:?}", self.open_file);
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        // Central UI pannel
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let frame_timer = renderer::frame_timer::start();
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

            // * Keep this callback as the last one registered. This resolves the frame_timer.
            ui.painter().add(egui_wgpu::Callback::new_paint_callback(canvas_rect, frame_timer));
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
