// use winit::event_loop::{ControlFlow, EventLoop};
use crate::app::PatternSeer;


mod app;
// mod gpu_context;
// mod canvas;
// mod camera;
// mod utils;


fn main() -> anyhow::Result<()>{
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("PatternSeer", native_options, Box::new(
        |cc| Ok(Box::new(PatternSeer::new(cc)))
    ))?;
    
    // env_logger::init();
    
    // let event_loop = EventLoop::new()?;
    // event_loop.set_control_flow(ControlFlow::Wait);
    // event_loop.run_app(&mut app::App::default())?;
    
    Ok(())
}
