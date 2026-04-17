use std::path::PathBuf;
use eframe::{egui, egui_wgpu};
use glam::Vec2;
use crate::camera::Camera;
use crate::pattern::Pattern;
use crate::renderer::RenderContext;

mod renderer;
mod app;

mod utils;
mod camera;
mod pattern;

fn main() -> anyhow::Result<()>{
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("PatternSeer"),
        renderer: eframe::Renderer::Wgpu,
        wgpu_options: eframe::egui_wgpu::WgpuConfiguration::default(),
        ..Default::default()
    };
    eframe::run_native(
        "PatternSeer",
        native_options, Box::new(|cc| { Ok(Box::new(app::PatternSeer::new(cc))) }
    ))?;

    Ok(())
}
