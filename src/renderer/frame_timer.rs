use std::time::Instant;
use eframe::{
    egui_wgpu::{
        self,
        wgpu
    },
    egui,
};

pub struct FrameTimerCallback {
    frame_start: Instant,
}
impl egui_wgpu::CallbackTrait for FrameTimerCallback {
    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &egui_wgpu::CallbackResources,
    ) {
        println!("{}µs per frame", self.frame_start.elapsed().as_micros());
    }
}

pub fn start() -> FrameTimerCallback {
    FrameTimerCallback { frame_start: Instant::now() }
}
