mod shader;
mod program;
mod opengl_backend;
use std::ffi::{CStr, CString};
use gl::types::GLuint;
use opengl_backend::VideoBuffer;
use sdl2::event::Event as SdlEvent;

pub const RESOLUTION: (u32, u32) = (900, 700);
pub const GRID_SIZE: (u32, u32) = (45, 35);
pub const QUAD_SIZE: (f32, f32) = (
    (RESOLUTION.0/GRID_SIZE.0 * 4) as f32,
    (RESOLUTION.1/GRID_SIZE.1 * 4) as f32
    ); 

fn main() {
    println!("size: {:?}", QUAD_SIZE);
    let mut gl_context = opengl_backend::init_sdl().unwrap();
    let mut video_buffer = VideoBuffer::new();

    let color_one = (255, 0, 0);
    let color_two = (0, 255, 0);

    let mut current_color = color_one;
    let mut current_color_bool = false;
    for x in 0..=GRID_SIZE.0 + 1 {
        for y in 0..=GRID_SIZE.1 + 1 {
            video_buffer.add_quad(
                (x as f32 * QUAD_SIZE.0, y as f32 * QUAD_SIZE.1),
                current_color
                );
            if current_color_bool {
                current_color = color_one; 
            }
            else {
                current_color = color_two;
            }
            current_color_bool = !current_color_bool;
        }
    }

    video_buffer.add_quad((0.0 * QUAD_SIZE.0, 0.0 * QUAD_SIZE.1), 
                          (255, 255, 255));


    'run_loop: loop {
        let vao_id = opengl_backend::bind_video_buffer(&video_buffer).unwrap();

        for event in gl_context.event_pump.poll_iter() {
            match event {
                SdlEvent::Quit {..} => break 'run_loop,
                _ => {}
            }
        }
        opengl_backend::render_buffer(
            &mut gl_context,
            vao_id,
            (video_buffer.vertices.len() / 6).try_into().unwrap());
    }
}
