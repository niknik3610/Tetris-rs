mod shader;
use sdl2::{event::Event as SdlEvent, video::gl_attr};
use std::ffi::{CStr, CString};
use gl::types::GLuint;

fn main() {
    let sdl = sdl2::init().expect("Failed to init SDL");    
    let video_subsystem = sdl.video().expect("Failed to init video");
    
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4,5);

    let window = video_subsystem
        .window("Tetris", 800, 400)
        .opengl()
        .build()
        .expect("Failed to start window");
    let gl_context = window.gl_create_context().expect("Failed to create Context");
    let gl = gl::load_with (|f|
        video_subsystem.gl_get_proc_address(f) as *const std::os::raw::c_void
        );
    
    unsafe { 
        gl::Viewport(0, 0, 800, 400);
        gl::ClearColor(0.0, 0.5, 1.0, 1.0);
    }
    let mut event_pump = sdl.event_pump().expect("Failed to init event pump");
    'run_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                SdlEvent::Quit {..} => break 'run_loop,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.gl_swap_window();
    }
}
