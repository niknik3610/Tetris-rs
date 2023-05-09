mod shader;
mod program;
mod opengl_backend;
mod pieces;

use std::{ffi::{CStr, CString}, time::Duration};
use gl::types::GLuint;
use opengl_backend::VideoBuffer;
use sdl2::event::Event as SdlEvent;
use std::thread::sleep;

pub const RESOLUTION: (u32, u32) = (800, 800);
pub const GRID_SIZE: (u32, u32) = (RESOLUTION.0/10, RESOLUTION.1/10);
pub const SIZE_MULTIPLIER: u32 = 4;
pub const QUAD_SIZE: f32 = (RESOLUTION.0 / GRID_SIZE.0 * SIZE_MULTIPLIER) as f32;

pub const GRID_START: [u32; 2] = [
        GRID_SIZE.0 / (SIZE_MULTIPLIER * 2) - 5, 
        0
];
pub const GRID_END: [u32; 2] = [
        GRID_SIZE.0 / (SIZE_MULTIPLIER * 2) + 5,
        20
];

fn main() {
    let mut gl_context = opengl_backend::init_sdl().unwrap();
    let mut video_buffer = VideoBuffer::new();
    draw_board(&mut video_buffer);
    println!("{}", GRID_START[0]);
    
    let bglen = video_buffer.bg_verts.len() / 6;
    let mut curr_piece = pieces::PIECES[0];

    'run_loop: loop { 
        video_buffer.clear_fg();

        curr_piece.blocks.iter().for_each(|block|{
            video_buffer.add_quad_fg(
                (
                    curr_piece.coordinates.0 + block.0,
                    curr_piece.coordinates.1 + block.1
                ),   
                curr_piece.color
                );
            println!("x: {}", curr_piece.coordinates.0 + block.0);
        });
        curr_piece.mv(pieces::Move::DOWN);

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
            ((bglen + (video_buffer.fg_verts.len() / 6))).try_into().unwrap()
            ).unwrap();
        sleep(Duration::from_millis(500)) 
    }
}

fn draw_board(video_buffer: &mut VideoBuffer) {
    let color_one = (115, 34, 122);
    let color_two = (87, 12, 94);

    let mut current_color = color_one;
    let mut current_color_bool = false; 

    for x in GRID_START[0]..GRID_END[0] {
        for y in GRID_START[1]..=GRID_END[1] {
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
