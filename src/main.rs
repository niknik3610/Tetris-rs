mod shader;
mod program;
mod opengl_backend;
use std::ffi::{CStr, CString};
use gl::types::GLuint;
use opengl_backend::VideoBuffer;
use sdl2::event::Event as SdlEvent;

pub const RESOLUTION: (u32, u32) = (900, 700);
pub const GRID_SIZE: (u32, u32) = (90, 70);
pub const QUAD_SIZE: (f32, f32) = (
    (RESOLUTION.0/GRID_SIZE.0) as f32,
    (RESOLUTION.1/GRID_SIZE.1) as f32
    ); 

fn main() {
    let mut gl_context = opengl_backend::init_sdl().unwrap();
    let mut video_buffer = VideoBuffer::new();

    video_buffer.add_quad((450.0, 350.0), (255, 255, 0));
    let vao_id = opengl_backend::bind_video_buffer(video_buffer).unwrap();

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
