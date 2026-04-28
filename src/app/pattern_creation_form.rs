use crate::pattern::{Pattern, PatternDraft};
use eframe::egui;

pub enum PatternFormEvent {
    /// Nothing of note happened in the form.
    NothingHappened,
    /// Form's "Cancel" button was pressed.
    FormCancelled,
    /// Form was edited and was neither saved nor cancelled.
    FormUpdated(PatternDraft),
    /// Form's "Save" button was pressed and the draft was successfully finalized.
    FormComplete(Pattern),
}

/// Shows the pattern creation form to the screen for one frame
/// 
/// # Parameters
/// 
/// - `ui`: Egui UI to display the form to.
/// - `draft`: Draft version of the new pattern.
/// 
/// # Returns
/// 
/// `PatternFormEvent` which can be
/// - `NothingHappened`: When no event happened in the form; usually means somethign went wrong.
/// - `FormUpdated(updated_draft)`: When the form was neither saved nor cancelled.
/// - `FormComplete(pattern)`: When the "Save" button was pressed and the draft was successfully finalized
/// into an actual Pattern.
/// - `FormCancelled`: When the "Cancel" button was pressed.
pub fn show(ui: &mut egui::Ui, mut draft: PatternDraft) -> PatternFormEvent {
    let mut event = PatternFormEvent::NothingHappened;

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
                });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                if ui.button("Save").clicked() {
                    event = PatternFormEvent::FormComplete(Pattern::from_draft(&draft));
                } else if ui.button("Cancel").clicked() {
                    event = PatternFormEvent::FormCancelled;
                } else {
                    event = PatternFormEvent::FormUpdated(draft);
                }
            });
        });

    event

}
