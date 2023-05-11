use crate::{
    error_handler::Error,
    opengl_backend::VideoBuffer,
    pieces::{Move, Piece},
    BOARD_START, QUAD_SIZE,
};

#[derive(Copy, Clone)]
pub struct Block {
    color: (u8, u8, u8),
}
pub struct Board {
    pub blocks: [[Option<Block>; 20]; 10],
}

#[derive(Debug)]
pub enum BoardError {
    AlreadyTaken,
}
impl Error for BoardError {
    fn to_string(&self) -> String {
        use BoardError::*;
        match self {
            AlreadyTaken => return "This Block is already Occupied".to_owned(),
        }
    }
}

impl Board {
    pub fn new() -> Board {
        return Board {
            blocks: [[None; 20]; 10],
        };
    }
    pub fn add_block(&mut self, pos: (f32, f32), color: (u8, u8, u8)) -> Result<(), BoardError> {
        //Error Handling (Yuck)

        // if let None = self.blocks[pos.0 as usize][pos.1 as usize] {
        //     println!("{}, {}", pos.0, pos.1);
        //     return Ok(())
        // }
        // return Err(BoardError::AlreadyTaken)
        self.blocks[pos.0 as usize][pos.1 as usize] = Some(Block::new(color));
        return Ok(());
    }
    pub fn check_collisions(&mut self, mut piece: Piece) -> bool {
        piece.blocks.iter_mut().for_each(|block| {
            *block = normalize_screen_pos((
                block.0 + piece.coordinates.0,
                block.1 + piece.coordinates.1,
            ));
        });

        for block in piece.blocks { 
            if self.check_block((block.0, block.1 - 1.0)) {
                return true;
            } else if block.1 == 0.0 {
                return true;
            }
        }
        return false;
    }
    fn check_block(&self, pos: (f32, f32)) -> bool {
        if let Some(_) = self.blocks[pos.0 as usize][pos.1 as usize] {
            return true;
        }
        return false;
    }
    pub fn add_to_video_buff(&self, vid_buff: &mut VideoBuffer) {
        self.blocks.iter().enumerate().for_each(|(x, x_block)| {
            x_block.iter().enumerate().for_each(|(y, y_block)| {
                if let Some(block) = y_block {
                    vid_buff.add_quad_fg(
                        (
                            (BOARD_START[0] as f32 + (x as f32)) * QUAD_SIZE,
                            (BOARD_START[1] as f32 + (y as f32)) * QUAD_SIZE,
                        ),
                        block.color,
                    );
                }
            })
        })
    }
}

impl Block {
    fn new(color: (u8, u8, u8)) -> Block {
        return Block { color };
    }
}

pub fn normalize_screen_pos(coords: (f32, f32)) -> (f32, f32) {
    let coords0 = coords.0 / QUAD_SIZE - BOARD_START[0] as f32;
    let coords1 = coords.1 / QUAD_SIZE as f32;
    return (coords0, coords1);
}

pub fn check_legal_move(mut piece: Piece, mv: Move) -> bool {
    piece.mv(mv);
    let normalized_coords_1 = normalize_screen_pos((piece.coordinates.0 + piece.blocks[0].0, 0.0));
    let normalized_coords_2 = normalize_screen_pos((piece.coordinates.0 + piece.blocks[3].0, 0.0));

    if normalized_coords_1.0 < 0.0 || normalized_coords_2.0 >= 10.0 {
        return false;
    }
    return true;
}
