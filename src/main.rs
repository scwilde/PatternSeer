use std::path::PathBuf;

use eframe::{egui, egui_wgpu};
use crate::pattern::{Pattern, stitch_buffer::StitchBuffer, stitch_palette::StitchPaletteIndex};


mod app;
mod pattern;
mod utils;

fn main() -> anyhow::Result<()>{
    // let native_options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default()
    //         .with_inner_size([1280.0, 720.0])
    //         .with_title("PatternSeer"),
    //     renderer: eframe::Renderer::Wgpu,
    //     wgpu_options: egui_wgpu::WgpuConfiguration::default(),
    //     ..Default::default()
    // };
    // eframe::run_native(
    //     "PatternSeer",
    //     native_options, Box::new(|cc| { Ok(Box::new(app::PatternSeer::new(cc))) }
    // ))?;

    // Ok(())

    let (width, height) = (5, 5);
    let test_grid = StitchBuffer::with_size(width, height);
    let mut test_save_pattern = Pattern::new(width, height, test_grid);
    *test_save_pattern.primary_grid.get_mut(13).unwrap() = StitchPaletteIndex::new(5);

    println!("{:?}", test_save_pattern);


    let mut path: Option<PathBuf> = None;
    if let None = path {
        path = rfd::FileDialog::new()
            .add_filter("PatternSeer Pattern", &["psp"])
            .save_file();
    }
    let path = match path {
        Some(path) => path,
        None => return Ok(()),
    };

    pattern::io::save(&path, &test_save_pattern).expect("File save error");

    let test_read_pattern = pattern::io::load(&path).expect("File read error");
    println!("{:?}", test_read_pattern);

    assert_eq!(test_save_pattern, test_read_pattern);
    
    Ok(())
}
