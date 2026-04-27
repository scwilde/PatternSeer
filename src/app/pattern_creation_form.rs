use crate::{
    app::forms::FormTSM,
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
