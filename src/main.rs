mod error_handler;
mod game_board;
mod input_handler;
mod opengl_backend;
mod pieces;

use game_board::normalize_screen_pos;
use game_board::{check_legal_move, Board};
use gl::types::GLuint;
use opengl_backend::VideoBuffer;
use sdl2::event::Event as SdlEvent;
use std::thread::sleep;
use std::{
    ffi::{CStr, CString},
    time::{Duration, Instant},
};

use crate::error_handler::Error;

//TODO: Change this to change per level
pub const TICK_RATE: Duration = Duration::from_millis(8);

pub const RESOLUTION: (u32, u32) = (800, 800);
pub const GRID_SIZE: (u32, u32) = (RESOLUTION.0 / 10, RESOLUTION.1 / 10);
pub const SIZE_MULTIPLIER: u32 = 4;
pub const QUAD_SIZE: f32 = (RESOLUTION.0 / GRID_SIZE.0 * SIZE_MULTIPLIER) as f32;

pub const BOARD_START: [u32; 2] = [GRID_SIZE.0 / (SIZE_MULTIPLIER * 2) - 5, 0];
pub const BOARD_END: [u32; 2] = [GRID_SIZE.0 / (SIZE_MULTIPLIER * 2) + 5, 20];

fn main() {
    let mut gl_context = opengl_backend::init_sdl().unwrap();

    let mut video_buffer = VideoBuffer::new();

    draw_background(&mut video_buffer);
    let bglen = video_buffer.bg_verts.len() / 6;

    let mut game_board = Board::new();
    game_board.add_block((0.0, 0.0), (255, 255, 255)).unwrap();
    game_board.add_block((1.0, 0.0), (255, 255, 255)).unwrap();

    let mut curr_piece = pieces::PIECES[3];

    let mut moves_per_second = 7;
    let mut move_time = Duration::from_millis(1000 / moves_per_second);

    let mut last_tick = Instant::now();
    let mut last_move = Instant::now();
    'run_loop: loop {
        //moves block according to move rate and checks for block rules
        if Instant::now().duration_since(last_move) > move_time {
            curr_piece.mv(pieces::Move::DOWN);
            if game_board.check_collisions(curr_piece) {
                curr_piece.blocks.iter().for_each(|block| {
                    game_board
                        .add_block(
                            normalize_screen_pos((
                                block.0 + curr_piece.coordinates.0,
                                block.1 + curr_piece.coordinates.1,
                            )),
                            curr_piece.color,
                        )
                        .unwrap();
                });
                curr_piece = pieces::PIECES[0];
            }

            last_move = Instant::now();
        }

        if TICK_RATE > Instant::now().duration_since(last_tick) {
            sleep(TICK_RATE - Instant::now().duration_since(last_tick));
        }
        last_tick = Instant::now();

        video_buffer.clear_fg();

        game_board.add_to_video_buff(&mut video_buffer);
        curr_piece.blocks.iter().for_each(|block| {
            video_buffer.add_quad_fg(
                (
                    curr_piece.coordinates.0 + block.0,
                    curr_piece.coordinates.1 + block.1,
                ),
                curr_piece.color,
            );
        });

        let vao_id = opengl_backend::bind_video_buffer(&video_buffer).unwrap();
        for event in gl_context.event_pump.poll_iter() {
            match event {
                SdlEvent::Quit { .. } => break 'run_loop,
                SdlEvent::KeyDown {
                    timestamp,
                    window_id,
                    keycode,
                    ..
                } => {
                    if let Some(action) = input_handler::handle_key_event(keycode) {
                        if check_legal_move(curr_piece, action) {
                            curr_piece.mv(action);
                        }
                    }
                }
                _ => {}
            }
        }

        opengl_backend::render_buffer(
            &mut gl_context,
            vao_id,
            (bglen + (video_buffer.fg_verts.len() / 6))
                .try_into()
                .unwrap(),
        )
        .unwrap();
    }
}

fn draw_background(video_buffer: &mut VideoBuffer) {
    let color_one = (115, 34, 122);
    let color_two = (87, 12, 94);

    let mut current_color = color_one;
    let mut current_color_bool = false;

    for x in BOARD_START[0]..BOARD_END[0] {
        for y in BOARD_START[1]..=BOARD_END[1] {
            video_buffer.add_quad_bg((x as f32 * QUAD_SIZE, y as f32 * QUAD_SIZE), current_color);
            if current_color_bool {
                current_color = color_one;
            } else {
                current_color = color_two;
            }
            current_color_bool = !current_color_bool;
        }
    }
}
