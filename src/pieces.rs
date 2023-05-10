use crate::{BOARD_START, QUAD_SIZE};

pub const PIECES: [Piece; 1] = [Piece::new(
    [
        (0.0 * QUAD_SIZE, 0.0),
        (1.0 * QUAD_SIZE, 0.0),
        (2.0 * QUAD_SIZE, 0.0),
        (3.0 * QUAD_SIZE, 0.0),
    ],
    ((BOARD_START[0] as f32 + 3.0) * QUAD_SIZE, 20.0 * QUAD_SIZE),
    (10, 220, 240),
)];

#[derive(Copy, Clone)]
pub enum Move {
    LEFT,
    RIGHT,
    DOWN,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub blocks: [(f32, f32); 4],
    pub coordinates: (f32, f32),
    pub color: (u8, u8, u8),
}

impl Piece {
    const fn new(blocks: [(f32, f32); 4], coordinates: (f32, f32), color: (u8, u8, u8)) -> Piece {
        return Piece {
            blocks,
            coordinates,
            color,
        };
    }
    pub fn mv(&mut self, mv: Move) {
        match mv {
            Move::LEFT => {
                self.coordinates.0 -= 1.0 * QUAD_SIZE;
            }
            Move::RIGHT => {
                self.coordinates.0 += 1.0 * QUAD_SIZE;
            }
            Move::DOWN => {
                self.coordinates.1 -= 1.0 * QUAD_SIZE;
            }
        }
    }
}
