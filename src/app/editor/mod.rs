use crate::{
    app::editor::{
        camera::Camera,
        renderer::EditorRenderContext,
    },
    pattern::Pattern, utils::{self, CommandBuffer},
};
use eframe::{egui, egui_wgpu};


pub mod renderer;
mod camera;


/// Commands passed to the editor from other GUI elements.
pub enum EditorCommand {
    /// Adjust camera position and zoom so the entire pattern fits in view.
    FitToPattern,
}

/// Tracker for which commands are currently active and which are inactive.
struct EditorCommandBuffer {
    /// Adjust camera position and zoom so the entire pattern fits in view.
    pub fit_to_pattern: utils::CommandSlot<()>,
}
impl utils::CommandBuffer for EditorCommandBuffer {
    type Command = EditorCommand;

    fn new() -> Self {
        Self {
            fit_to_pattern: utils::CommandSlot::Inactive,
        }
    }

    fn push(&mut self, cmd: Self::Command) -> Option<Self::Command> {
        match cmd {
            EditorCommand::FitToPattern => self.fit_to_pattern.activate(()).map(|_| EditorCommand::FitToPattern)
        }
    }
}


/// The main panel on screen where a pattern can be seen and edited.
pub struct Editor {
    camera: Camera,
    pending_cmds: EditorCommandBuffer,
}
impl Editor {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            pending_cmds: EditorCommandBuffer::new(),
        }
    }

    /// Draws the editor onto the screen for one frame.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
        pattern: &mut Pattern,
    ) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let frame_timer = renderer::frame_timer::start();

            // TODO This is horrible. Rework the Graphics API so we don't have to do this
            let mut wgpu_renderer = frame.wgpu_render_state().unwrap().renderer.write();
            let render_context = wgpu_renderer.callback_resources.get_mut::<EditorRenderContext>().unwrap();

            // Create our rendering canvas filling all available space
            let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
            self.camera.resize(rect.width(), rect.height());
            if let Some(_) = self.pending_cmds.fit_to_pattern.take() {
                self.camera.fit_to_pattern(pattern);
            }

            // Camera pan and zoom controls
            if response.dragged_by(egui::PointerButton::Secondary) {
                self.camera.pan(response.drag_delta().x, response.drag_delta().y);
            }
            if response.hovered() {
                let scroll_delta = ui.input(|i| i.smooth_scroll_delta);
                if scroll_delta.y != 0.0 {
                    self.camera.zoom(scroll_delta.y);
                }
            }

            // Stop camera from zooming too far in/out or wandering too far from pattern
            self.camera.limit_pan(&pattern);
            self.camera.limit_zoom(&pattern);

            // Render the canvas
            render_context.rendered_mesh.clear();
            let grid = renderer::grid_renderer::render(render_context, &self.camera, &pattern);
            let grid_painter = egui_wgpu::Callback::new_paint_callback(rect, grid);
            ui.painter().add(grid_painter);

            // * Keep this callback as the last one registered. This resolves the frame_timer.
            ui.painter().add(egui_wgpu::Callback::new_paint_callback(rect, frame_timer));
        });
    }

    /// Queues up a new `EditorCommands` variant for editor to act on in its next frame.
    /// Each variant will only be queued up once per frame, with the most recent call overwriting the previous.
    pub fn queue_cmd(&mut self, cmd: EditorCommand) {
        self.pending_cmds.push(cmd);
    }
}
