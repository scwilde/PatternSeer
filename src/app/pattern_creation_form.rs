use eframe::egui;
use crate::pattern::{Pattern, PatternDraft};


pub enum PatternCreationForm {
    Closed,
    PendingEdits(PatternDraft),
    TakenForEdit,
    Done(Pattern),
}
impl PatternCreationForm {
    // TODO Make it so every state change happens with a dedicated function
    pub fn open(&mut self) {
        *self = PatternCreationForm::PendingEdits(PatternDraft::new());
    }

    pub fn close(&mut self) {
        *self = PatternCreationForm::Closed;
    }

    pub fn take_finished_pattern(&mut self) -> Pattern {
        if matches!(self, PatternCreationForm::Done(_)) {
            let old = std::mem::replace(self, PatternCreationForm::Closed);
            return match old {
                PatternCreationForm::Done(pattern) => pattern,
                _ => unreachable!(),
            }
        } else {
            panic!("Form is unfinished!");
        }
    }

    pub fn take_draft_for_edit(&mut self) -> PatternDraft {
        if matches!(self, PatternCreationForm::PendingEdits(_)) {
            let old = std::mem::replace(self, PatternCreationForm::TakenForEdit);
            return match old {
                PatternCreationForm::PendingEdits(draft) => draft,
                _ => unreachable!(),
            }
        } else {
            panic!("Form is not currently open for edits!");
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let mut draft = self.take_draft_for_edit();
        let form_complete = draft.path.is_some();

        egui::Window::new("New Pattern")
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(ui.content_rect().center())
            .default_width(300.0)
            .resizable(false)
            .collapsible(false)
            .order(egui::Order::Foreground)
            .show(ui, |ui| {
                egui::Grid::new("Main Grid")
                    .num_columns(2)
                    .spacing(egui::vec2(10.0, 10.0))
                    .show(ui, |ui| {
                        ui.label("Dimensions");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut draft.width));
                            ui.label(" x ");
                            ui.add(egui::DragValue::new(&mut draft.height));
                        });
                        ui.end_row();

                        ui.label("Save Location");
                        ui.horizontal(|ui| {
                            let no_path = String::from("No path selected...");
                            let mut path_str = draft.path.as_ref().unwrap_or(&no_path).as_str();

                            ui.add(egui::TextEdit::singleline(&mut path_str));
                            if ui.button("...").clicked() && let Some(path) = rfd::FileDialog::new()
                                .add_filter("PatternSeer Pattern", &["psp"])
                                .save_file() {
                                    if let Some(path_utf8) = path.to_str() {
                                        draft.path = Some(path_utf8.to_owned());
                                    } else {
                                        println!("Path '{}' is invalid", path.display());
                                    }
                                }
                        });
                        ui.end_row();
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui.add_enabled(form_complete, egui::Button::new("Save")).clicked() {
                        if let Ok(pattern) = Pattern::from_draft(draft) {
                            *self = PatternCreationForm::Done(pattern);
                        }
                    } else if ui.button("Cancel").clicked() {
                        *self = PatternCreationForm::Closed;
                    } else {
                        *self = PatternCreationForm::PendingEdits(draft);
                    }
                });
            });
    }
}
