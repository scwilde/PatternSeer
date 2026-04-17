use std::path::PathBuf;
use eframe::egui;

pub enum MenubarEvent {
    OpenPattern { path: PathBuf },
    CreatePattern,
    CloseWindow,
    DoNothing,
}

pub fn show(
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
) -> MenubarEvent {
    let mut event = MenubarEvent::DoNothing;

    egui::Panel::top("Menu Bar").show_inside(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                // if ui.button("New").clicked() && let Some(path) = rfd::FileDialog::new()
                //     .add_filter("PatternSeer Pattern", &["psp"])
                //     .save_file() {
                //         println!("Opening pattern: '{}'", path.display());
                //         *pattern = match Pattern::create_sync(path.to_str().expect("Invalid path"), 30, 30) {
                //             Ok(pattern) => Some(pattern),
                //             Err(e) => { println!("Failed to create pattern: {}", e); None }
                //          };
                //         println!("Pattern dimensions: {}w x {}h", pattern.as_ref().unwrap().metadata.width, pattern.as_ref().unwrap().metadata.height);
                //     }
                // if ui.button("Open").clicked() && let Some(path) = rfd::FileDialog::new()
                //     .add_filter("PatternSeer Pattern", &["psp"])
                //     .pick_file() {
                //         println!("Opening pattern: '{}'", path.display());
                //         *pattern = match Pattern::open_sync(path.to_str().expect("Invalid path")) {
                //             Ok(pattern) => Some(pattern),
                //             Err(e) => { println!("Failed to open pattern: {}", e); None }
                //         };
                //         println!("Pattern dimensions: {}w x {}h", pattern.as_ref().unwrap().metadata.width, pattern.as_ref().unwrap().metadata.height);
                // }
                if ui.button("New").clicked() {
                    event = MenubarEvent::CreatePattern;
                }
                if ui.button("Open").clicked() && let Some(path) = rfd::FileDialog::new()
                    .add_filter("PatternSeer Pattern", &["psp"])
                    .pick_file() {
                        event = MenubarEvent::OpenPattern { path };
                }
                ui.separator();
                if ui.button("Quit").clicked() {
                    event = MenubarEvent::CloseWindow;
                }
            });
        });
    });
    event
}
