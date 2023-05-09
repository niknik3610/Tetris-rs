mod shader;
mod program;
mod opengl_backend;
use std::ffi::{CStr, CString};
use gl::types::GLuint;
use opengl_backend::VideoBuffer;
use sdl2::event::Event as SdlEvent;

pub const RESOLUTION: (u32, u32) = (600, 600);
pub const GRID_SIZE: (u32, u32) = (60, 60);
pub const QUAD_SIZE: f32 = (RESOLUTION.0 / GRID_SIZE.0) as f32;
fn main() {
    let mut gl_context = opengl_backend::init_sdl().unwrap();
    let mut video_buffer = VideoBuffer::new();
    create_bg(&mut video_buffer);
    video_buffer.add_quad_bg((1.0 * QUAD_SIZE, 1.0 * QUAD_SIZE), 
                             (0, 255, 255));

    let bglen = video_buffer.bg_verts.len() / 6;
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
            ((bglen + video_buffer.fg_verts.len()) / 6).try_into().unwrap()
            ).unwrap();
    }
}

fn create_bg(video_buffer: &mut VideoBuffer) {
    let color_one = (115, 34, 122);
    let color_two = (87, 12, 94);

    let mut current_color = color_one;
    let mut current_color_bool = false; 

    let grid_start = 0;
    let grid_end = GRID_SIZE.0;

    for x in grid_start..=grid_end {
        for y in 0..=GRID_SIZE.1 {
            video_buffer.add_quad_bg(
                (x as f32 * QUAD_SIZE, y as f32 * QUAD_SIZE),
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
}
