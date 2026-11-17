use crate::app::App;
use winit::event_loop::EventLoop;

mod app;
mod state;

fn main() -> color_eyre::Result<()> {
    env_logger::init();

    let mut app = App::new();

    let event_loop = EventLoop::with_user_event().build()?;
    event_loop.run_app(&mut app)?;

    Ok(())
}
