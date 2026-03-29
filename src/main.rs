use eframe::{egui, egui_wgpu};

mod pattern_renderer;
use crate::pattern_renderer::PatternRenderer;


struct PatternSeer {
    renderer: PatternRenderer
    // renderer: Renderer
    // canvas: Canvas,
    // camera: Camera,
    // renderer: Renderer
}

impl PatternSeer {
    fn new(cc: &eframe::CreationContext) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().unwrap();

        PatternSeer {
            renderer: PatternRenderer::new(&mut wgpu_render_state.renderer.write().callback_resources)
        }
    }
}

impl eframe::App for PatternSeer {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ui, |ui| {
            let desired_size = egui::vec2(200.0, 100.0);
            let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

            let callback_shape = egui_wgpu::Callback::new_paint_callback(rect, self.renderer);

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
