use std::time::Instant;
use eframe::{
    egui_wgpu::{
        self,
        wgpu
    },
    egui,
};

/// Faux `PaintCallback` which just keeps an internal timer and then prints its duration during the `paint()` phase.
/// It's meant to be created at the very start of the frame, but the last `PaintCallback` to be registered with egui
/// to ensure its `paint()` phase is the very last one of the frame. Also, to elegantly handle egui's reactive mode,
/// it should be created only when a new frame begins processing.
pub struct FrameTimerCallback {
    frame_start: Instant,
}
impl egui_wgpu::CallbackTrait for FrameTimerCallback {
    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        _render_pass: &mut wgpu::RenderPass<'static>,
        _callback_resources: &egui_wgpu::CallbackResources,
    ) {
        println!("{}µs per frame", self.frame_start.elapsed().as_micros());
    }
}

/// Starts a new frame timer.
///
/// # Returns
///
/// `FrameTimerCallback` which, when registered with wgpu as a `PaintCallback`, will print the time between
/// this function call and its `paint()` phase.
pub fn start() -> FrameTimerCallback {
    FrameTimerCallback { frame_start: Instant::now() }
}
