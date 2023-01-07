use crate::*;

pub struct Piece {
    position: i32,
}

pub struct GameState {
    current_turn: i32,
    player_pieces: [[Piece; 4]; 4],
    win_state: i32,
}