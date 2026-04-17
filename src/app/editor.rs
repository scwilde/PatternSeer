use crate::camera::Camera;
use crate::pattern::Pattern;
use crate::renderer::{self, RenderContext};
use eframe::{egui, egui_wgpu};

pub fn show(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    camera: &mut Camera,
    pattern:&mut Pattern,
) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        let frame_timer = renderer::frame_timer::start();
        // ! This is horrible. Rework the Graphics API so we don't have to do this
        let mut wgpu_renderer = frame.wgpu_render_state().unwrap().renderer.write();
        let render_context = wgpu_renderer.callback_resources.get_mut::<RenderContext>().unwrap();

        // Create our rendering canvas filling all available space
        let (canvas_rect, canvas_response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
        camera.resize(canvas_rect.width(), canvas_rect.height());

        // Camera pan and zoom controls
        if canvas_response.dragged_by(egui::PointerButton::Secondary) {
            camera.pan(canvas_response.drag_delta().x, canvas_response.drag_delta().y);
        }
        if canvas_response.hovered() {
            let scroll_delta = ui.input(|i| i.smooth_scroll_delta);
            if scroll_delta.y != 0.0 {
                camera.zoom(scroll_delta.y);
            }
        }

        // Stop camera from zooming too far in/out or wandering too far from pattern
        camera.limit_pan(&pattern);
        camera.limit_zoom(&pattern);

        // Render the canvas
        render_context.rendered_mesh.clear();
        let grid = renderer::grid_renderer::render(render_context, &camera, &pattern);
        let grid_painter = egui_wgpu::Callback::new_paint_callback(canvas_rect, grid);
        ui.painter().add(grid_painter);

        // * Keep this callback as the last one registered. This resolves the frame_timer.
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(canvas_rect, frame_timer));
    });
}
