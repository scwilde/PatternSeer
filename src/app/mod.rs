use crate::camera::Camera;
use crate::pattern::Pattern;
use crate::renderer::{self, RenderContext};
use glam::Vec2;
use std::path::PathBuf;
use eframe::{egui, egui_wgpu};


mod menu_bar;
mod editor;


/// An instance of the application.
pub struct PatternSeer {
    /// The camera used for rendering.
    camera: Camera,
    /// Container for the pattern we are currently working on.
    pattern: Option<Pattern>,
}
impl PatternSeer {
    /// Creates a new instance of PatternSeer.
    ///
    /// # Parameters
    ///
    /// * 'cc' - `CreationContext` provided by something like `eframe::run_native()`.
    pub fn new(cc: &eframe::CreationContext) -> Self {
        renderer::init(cc.wgpu_render_state.as_ref().unwrap());

        PatternSeer {
            camera: Camera::default(),
            pattern: None,
        }
    }
}
impl eframe::App for PatternSeer {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame){
        menu_bar::show(ui, frame, &mut self.pattern);

        if let Some(pattern) = &mut self.pattern {
            editor::show(ui, frame, &mut self.camera, pattern);
        }
    }
}
