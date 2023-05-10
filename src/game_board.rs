use crate::{error_handler::Error, opengl_backend::VideoBuffer, QUAD_SIZE, BOARD_START};

#[derive(Copy, Clone)]
pub struct Block {
    color: (u8, u8, u8)
}
pub struct Board {
    pub blocks: [[Option<Block>; 10]; 20],
}

#[derive(Debug)]
pub enum BoardError {
    AlreadyTaken
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
        return Board {blocks: [[None; 10]; 20]} 
    }
    pub fn add_block(
        &mut self,
        pos: (f32, f32),
        color: (u8, u8, u8)
        ) -> Result<(), BoardError> {
        if let None = self.blocks[pos.0 as usize][pos.1 as usize] {
            self.blocks[pos.0 as usize][pos.1 as usize] = Some(Block::new(color));
            return Ok(())
        }
        return Err(BoardError::AlreadyTaken)
    }
    pub fn check_block(
        &self,
        pos: (f32, f32),
        ) -> bool {
        if let Some(_) = self.blocks[pos.0 as usize][pos.1 as usize] {
            return true;
        } 
        return false;
    }
    pub fn add_to_video_buff(&self, vid_buff: &mut VideoBuffer) {
        self.blocks.iter().enumerate().for_each(|(x, x_block)|{
            x_block.iter().enumerate().for_each(|(y, y_block)| {
                if let Some(block) = y_block {
                    vid_buff.add_quad_fg(
                        (
                            (BOARD_START[0] as f32 + (x as f32)) * QUAD_SIZE,
                            (BOARD_START[1] as f32 + (y as f32)) * QUAD_SIZE
                        ),
                        block.color
                        );
                }
            })
        })
    }
}

impl Block {
    fn new(color: (u8, u8, u8)) -> Block {
        return Block { color }
    }
}
