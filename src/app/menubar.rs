use std::path::PathBuf;
use eframe::egui;
use crate::pattern::Pattern;

/// Events caused by interactions with the menubar.
pub enum MenubarEvent {
    CreatePattern,
    OpenPattern(PathBuf),
    SavePattern,
    SavePatternAs(PathBuf),
    CloseWindow,
    
    FitToPattern,

    DoNothing,
}

/// Displays the menubar to the screen for one frame.
pub fn show(
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
    pattern: &mut Option<Pattern>,
) -> MenubarEvent {
    let mut event = MenubarEvent::DoNothing;

    egui::Panel::top("Menu Bar").show_inside(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() { event = MenubarEvent::CreatePattern }
                if ui.button("Open").clicked() && let Some(path) = rfd::FileDialog::new()
                    .add_filter("PatternSeer Pattern", &["psp"])
                    .pick_file() {
                        event = MenubarEvent::OpenPattern(path);
                    }
                ui.separator();
                if ui.add_enabled(pattern.is_some(), egui::Button::new("Save")).clicked()
                    && let Some(pattern) = pattern {
                        if pattern.path.is_none() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("PatternSeer Pattern", &["psp"])
                                .save_file() {
                                    pattern.path = Some(path);
                                    event = MenubarEvent::SavePattern;
                            } else {
                                event = MenubarEvent::DoNothing;
                            }
                        } else {
                            event = MenubarEvent::SavePattern;
                        }
                }
                if ui.add_enabled(pattern.is_some(), egui::Button::new("Save As...")).clicked()
                    && let Some(path) = rfd::FileDialog::new()
                    .add_filter("PatternSeer Pattern", &["psp"])
                    .save_file() {
                        event = MenubarEvent::SavePatternAs(path);
                }
                ui.separator();
                if ui.button("Quit").clicked() { event = MenubarEvent::CloseWindow }
            });
            ui.menu_button("View", |ui| {
                if ui.button("Fit to pattern").clicked() {
                    event = MenubarEvent::FitToPattern;
                }
            })
        });
    });
    event
}
