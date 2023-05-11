use crate::{BOARD_START, QUAD_SIZE};

pub const PIECES: [Piece; 7] = [
    Piece::new(     //I
        [
            //Blocks
            (0.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (1.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (2.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (3.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
        ],
        ((BOARD_START[0] as f32 + 3.0) * QUAD_SIZE, 20.0 * QUAD_SIZE), //Coords
        (10, 220, 240),                                                //Color
    ),
    Piece::new(     //T
        [
            (0.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (1.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (1.0 * QUAD_SIZE, 1.0 * QUAD_SIZE),
            (2.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
        ],
        ((BOARD_START[0] as f32 + 3.0) * QUAD_SIZE, 20.0 * QUAD_SIZE),
        (220, 80, 200),
    ),
    Piece::new(     //L Left Side
        [
            (0.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (0.0 * QUAD_SIZE, 1.0 * QUAD_SIZE),
            (1.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (2.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
        ],
        ((BOARD_START[0] as f32 + 3.0) * QUAD_SIZE, 20.0 * QUAD_SIZE),
        (0, 0, 240),
    ),
    Piece::new(     //L Right Side
        [
            (0.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (1.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (2.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (2.0 * QUAD_SIZE, 1.0 * QUAD_SIZE),
        ],
        ((BOARD_START[0] as f32 + 3.0) * QUAD_SIZE, 20.0 * QUAD_SIZE),
        (250, 150, 30),
    ),
    Piece::new(     //O
        [
            (0.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (0.0 * QUAD_SIZE, 1.0 * QUAD_SIZE),
            (1.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (1.0 * QUAD_SIZE, 1.0 * QUAD_SIZE),
        ],
        ((BOARD_START[0] as f32 + 3.0) * QUAD_SIZE, 20.0 * QUAD_SIZE),
        (255, 230, 30),
    ),
    Piece::new(     //Z right side
        [
            (0.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (1.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (1.0 * QUAD_SIZE, 1.0 * QUAD_SIZE),
            (2.0 * QUAD_SIZE, 1.0 * QUAD_SIZE),
        ],
        ((BOARD_START[0] as f32 + 3.0) * QUAD_SIZE, 20.0 * QUAD_SIZE),
        (30, 215, 30),
    ),
    Piece::new(     //Z left side
        [
            (0.0 * QUAD_SIZE, 1.0 * QUAD_SIZE),
            (1.0 * QUAD_SIZE, 1.0 * QUAD_SIZE),
            (1.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
            (2.0 * QUAD_SIZE, 0.0 * QUAD_SIZE),
        ],
        ((BOARD_START[0] as f32 + 3.0) * QUAD_SIZE, 20.0 * QUAD_SIZE),
        (215, 0, 30),
    ),
];

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
