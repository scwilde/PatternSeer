use eframe::{egui, egui_wgpu};
use crate::pattern_renderer::PatternRenderer;
use crate::utils::Volatile::{self, *};

mod pattern_renderer;
mod utils;


struct PatternSeer {
    hello_triangle: Volatile<Vec<utils::Vertex>>
}

impl PatternSeer {
    fn new(cc: &eframe::CreationContext) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();
        PatternRenderer::init(wgpu_render_state);

        PatternSeer {
            hello_triangle: Dirty(vec![
                utils::Vertex { position: [0.0, 0.5], color: [1.0, 0.0, 0.0] },
                utils::Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                utils::Vertex { position: [0.5, -0.5], color: [0.0, 0.0, 1.0] }
            ])
        }
    }
}

impl eframe::App for PatternSeer {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ui, |ui| {
            println!("New frame");
            let desired_size = egui::vec2(200.0, 100.0);
            let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

            self.hello_triangle.transform_with(|volatile| {
                match volatile {
                    Dirty(hello_triangle) => {
                        println!("Triangle dirty; Rerendering...");
                        PatternRenderer::update(&hello_triangle, frame);
                        Clean(hello_triangle)
                    }
                    Clean(_) => { volatile }
                }
            });
            let render_callback = PatternRenderer::render();
            let callback_shape = egui_wgpu::Callback::new_paint_callback(rect, render_callback);

            ui.painter().add(callback_shape);
        });
    }
}

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
        native_options, Box::new(|cc| { Ok(Box::new(PatternSeer::new(cc))) }
    ))?;
    
    Ok(())
}
