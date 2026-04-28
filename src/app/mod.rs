use crate::app::forms::FormTSM;
use crate::app::pattern_creation_form::PatternFormEvent;
use crate::pattern::{pattern_file, PatternDraft};
use crate::{app::menubar::MenubarEvent, pattern::Pattern};
use crate::app::editor::{Editor, EditorCommands};
use eframe::egui;


mod menubar;
mod editor;
pub mod forms;
mod pattern_creation_form;


/// An instance of the application.
pub struct PatternSeer {
    /// Pattern editing panel.
    editor: Editor,
    /// Currently opened pattern.
    pattern: Option<Pattern>,
    /// Typestate machine containing the form used for creating a new pattern.
    pattern_creation_form: FormTSM<PatternDraft>,
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
            pattern_creation_form: FormTSM::new(),
        }
    }
}
impl eframe::App for PatternSeer {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame){
        match menubar::show(ui, frame, &mut self.pattern) {
            // File events
            MenubarEvent::CreatePattern => {
                self.pattern_creation_form.transition_with(|state| {
                    match state {
                        FormTSM::Closed(inner) => inner.open(PatternDraft::new()).save_state(),
                        _ => state
                    }
                });
                self.editor.queue_cmd(EditorCommands::FitToPattern);
            },
            MenubarEvent::OpenPattern(path) => {
                self.pattern = match pattern_file::load(path.clone()) {
                    Ok(pattern) => {
                        self.editor.queue_cmd(EditorCommands::FitToPattern);
                        Some(pattern)
                    },
                    Err(e) => {
                        //TODO Bad UX
                        println!("Issue opening '{}': {}", path.display(), e);
                        None
                    },
                }
            },
            MenubarEvent::SavePattern => {
                if let Some(pattern) = &self.pattern && let Some(path) = &pattern.path {
                    pattern_file::save(&path, &pattern).unwrap_or_else(|e| {
                        println!("Issue saving pattern: {}", e);
                    })
                }
            },
            MenubarEvent::SavePatternAs(path) => {
                if let Some(pattern) = &mut self.pattern {
                    pattern.path = Some(path.clone());
                    pattern_file::save(&path, &pattern).unwrap_or_else(|e| {
                        println!("Issue saving pattern: {}", e);
                    })
                }
            }
            MenubarEvent::CloseWindow => ui.send_viewport_cmd(egui::ViewportCommand::Close),
            // View events
            MenubarEvent::FitToPattern => self.editor.queue_cmd(EditorCommands::FitToPattern),
            // Other events
            MenubarEvent::DoNothing => {}
        }

        self.pattern_creation_form.transition_with(|state| {
            match state {
                FormTSM::Closed(inner) => inner.save_state(), // Do nothing
                FormTSM::Pending(inner) => {
                    let (inner, draft) = inner.take_for_edits();
                    match pattern_creation_form::show(ui, draft) {
                        PatternFormEvent::FormUpdated(draft) => inner.submit_draft(draft).save_state(),
                        PatternFormEvent::FormComplete(pattern) => {
                            self.pattern = Some(pattern);
                            self.editor.queue_cmd(EditorCommands::FitToPattern);
                            inner.close_as_done().save_state()
                        },
                        PatternFormEvent::FormCancelled => inner.close_as_cancelled().save_state(),
                        PatternFormEvent::NothingHappened => panic!("Something went wrong with the pattern creation form.")
                    }
                },
                FormTSM::Transitioning(_) => unreachable!(),
            }
        });

        if let Some(pattern) = &mut self.pattern {
            self.editor.show(ui, frame, pattern);
        }
    }
}
