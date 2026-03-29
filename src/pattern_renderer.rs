use eframe::{egui, egui_wgpu::{self, wgpu}};


pub struct PatternRendererResources {
    frame_count: u32
}

#[derive(Clone, Copy)]
pub struct PatternRenderer {}

impl PatternRenderer {
    pub fn new(callback_resources: &mut egui_wgpu::CallbackResources) -> Self {
        callback_resources.insert(PatternRendererResources { frame_count: 0 });

        Self { }
    }
}

impl egui_wgpu::CallbackTrait for PatternRenderer {
    fn prepare(
            &self,
            _device: &wgpu::Device,
            _queue: &wgpu::Queue,
            _screen_descriptor: &egui_wgpu::ScreenDescriptor,
            _egui_encoder: &mut wgpu::CommandEncoder,
            callback_resources: &mut egui_wgpu::CallbackResources,
        ) -> Vec<wgpu::CommandBuffer> {
        if let Some(resources) = callback_resources.get_mut::<PatternRendererResources>() {
            resources.frame_count += 1;
        }

        Vec::new()
    }

    fn paint(
            &self,
            _info: egui::PaintCallbackInfo,
            _render_pass: &mut wgpu::RenderPass<'static>,
            callback_resources: &egui_wgpu::CallbackResources
        ) {
        if let Some(resources) = callback_resources.get::<PatternRendererResources>() {
            println!("Test renderer callback: frame {}", resources.frame_count);
        }
    }
}