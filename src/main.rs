use winit::event_loop::{ControlFlow, EventLoop};


mod app;
mod gpu_state;


fn main() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut app::App::default())?;

    Ok(())
}
