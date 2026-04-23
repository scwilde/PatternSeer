use eframe::egui;

/// Events caused by interactions with the menubar.
pub enum MenubarEvent {
    CreatePattern,
    OpenPattern { path: String },
    CloseWindow,
    FitToPattern,

    DoNothing,
}

/// Displays the menubar to the screen for one frame.
pub fn show(
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
) -> MenubarEvent {
    let mut event = MenubarEvent::DoNothing;

    egui::Panel::top("Menu Bar").show_inside(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    event = MenubarEvent::CreatePattern;
                }
                if ui.button("Open").clicked() && let Some(path) = rfd::FileDialog::new()
                    .add_filter("PatternSeer Pattern", &["psp"])
                    .pick_file() {
                        if let Some(path_utf8) = path.to_str() {
                            event = MenubarEvent::OpenPattern { path: path_utf8.to_owned() };
                        } else {
                            println!("Path '{}' invalid", path.display());
                            event = MenubarEvent::DoNothing;
                        }
                }
                ui.separator();
                if ui.button("Quit").clicked() {
                    event = MenubarEvent::CloseWindow;
                }
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
