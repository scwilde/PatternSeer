use crate::{
    app::forms::{FormDraft, FormTSM},
    pattern::{Pattern, PatternDraft},
};
use eframe::egui::{self, scroll_area::State};

pub enum PatternFormEvent {
    NothingHappened,
    FormCancelled,
    FormUpdated(PatternDraft),
    FormComplete(Pattern),
}

pub fn show(ui: &mut egui::Ui, mut draft: PatternDraft) -> PatternFormEvent {
    let mut event = PatternFormEvent::NothingHappened;

    let form_valid = draft.path.is_some();

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
                            .save_file()
                        {
                            if let Some(path_utf8) = path.to_str() {
                                draft.path = Some(path_utf8.to_owned());
                            } else {
                                // TODO Bad UX
                                println!("Path '{}' is invalid", path.display());
                            }
                        }
                    });
                    ui.end_row();
                });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                if ui.add_enabled(form_valid, egui::Button::new("Save")).clicked() {
                    match draft.finish() {
                        Ok(pattern) => event = PatternFormEvent::FormComplete(pattern),
                        Err(e) => {
                            // TODO Bad UX
                            println!("Issue saving pattern: {:?}", e);
                            event = PatternFormEvent::FormUpdated(draft);
                        }
                    }
                } else if ui.button("Cancel").clicked() {
                    event = PatternFormEvent::FormCancelled;
                } else {
                    event = PatternFormEvent::FormUpdated(draft);
                }
            });
        });

    event

}
