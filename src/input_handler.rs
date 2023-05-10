use sdl2::keyboard::Keycode;

use crate::pieces;

pub fn handle_key_event(key: Option<Keycode>) -> Option<pieces::Move> {
    use pieces::Move::*;
    if let Some(key) = key {
        match key {
            Keycode::Left => return Some(LEFT),
            Keycode::Right => return Some(RIGHT),
            Keycode::Down => return Some(DOWN),
            _ => None,
        }
    } else {
        return None;
    }
}
