mod app;
mod egui_tools;

use winit::event_loop::{ControlFlow, EventLoop};

#[unsafe(no_mangle)]
pub extern "C" fn hello_world() {
    println!("Hello, world from Rust!");
}

#[unsafe(no_mangle)]
pub extern "C" fn init_ui() {
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = app::App::new();

    event_loop.run_app(&mut app).expect("Failed to run app");
}
