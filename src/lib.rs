use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
mod wgpu_render;
mod mdl;
use crate::{wgpu_render::State, mdl::MDLFile};



pub async fn run() {
    
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("RSDKv5 MDL Render")
    .build(&event_loop)
    .unwrap();
    let mut state = State::new(&window).await;
    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        },
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => if !state.input(event) { match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::DroppedFile(out_file)=>{
                let mdl_file = MDLFile::open(&out_file);
                match mdl_file {
                    Err(error) => {
                        println!("Failed to Load {}",&out_file.display())
                    },
                    Ok(model)=>{
                        println!("Successful to Load {}",&out_file.display())
                    },
                }
            },
            WindowEvent::Resized(physical_size) => {
                state.resize(*physical_size);
            },
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &&mut so we have to dereference it twice
                state.resize(**new_inner_size);
            },
            _ => {}
        }}
        _ => {}
    });
}

