use crate::{app::menubar::MenubarEvent, pattern::Pattern};
use crate::app::editor::{Editor, EditorCommand};
use eframe::egui;


mod menubar;
mod editor;


/// An instance of the application.
pub struct PatternSeer {
    /// The editor panel.
    editor: Editor,
    /// Container for the pattern we are currently working on.
    pattern: Option<Pattern>,
}
impl PatternSeer {
    /// Creates a new instance of PatternSeer.
    ///
    /// # Parameters
    ///
    /// - 'cc': `CreationContext` provided by something like `eframe::run_native()`.
    pub fn new(cc: &eframe::CreationContext) -> Self {
        editor::renderer::init(cc.wgpu_render_state.as_ref().unwrap());

        PatternSeer {
            editor: Editor::new(),
            pattern: None,
        }
    }
}
impl eframe::App for PatternSeer {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame){
        match menubar::show(ui, frame) {
            // File events
            MenubarEvent::CreatePattern => println!("TODO: Create pattern"),
            MenubarEvent::OpenPattern { path } => {
                self.pattern = match Pattern::open_sync(path.as_str()) {
                    Ok(pattern) => {
                        self.editor.queue_cmd(EditorCommand::FitToPattern);
                        Some(pattern)
                    },
                    Err(e) => {
                        println!("Issue opening '{}': {}", path, e);
                        None
                    },
                }
            },
            MenubarEvent::CloseWindow => ui.send_viewport_cmd(egui::ViewportCommand::Close),

            // View events
            MenubarEvent::FitToPattern => self.editor.queue_cmd(EditorCommand::FitToPattern),

            MenubarEvent::DoNothing => {}
        }

        if let Some(pattern) = &mut self.pattern {
            self.editor.show(ui, frame, pattern);
        }
    }
}
