use env_logger;
use winit::event_loop::EventLoop;

use voxel_render::AppHandler;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    let mut handler = AppHandler::new();

    let _ = event_loop.run_app(&mut handler);
}
