mod shader;
mod program;
mod opengl_backend;
use std::ffi::{CStr, CString};
use gl::types::GLuint;
use sdl2::event::Event as SdlEvent;

fn main() {
    let mut gl_context = opengl_backend::init_sdl().unwrap();
    let vao_id = opengl_backend::set_video_buffers().unwrap();
    'run_loop: loop {
        for event in gl_context.event_pump.poll_iter() {
            match event {
                SdlEvent::Quit {..} => break 'run_loop,
                _ => {}
            }
        }
        opengl_backend::render_buffer(&mut gl_context, vao_id);
    }
}
