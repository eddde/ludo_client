use crate::*;


pub struct GameState {
    current_turn: i32,
    player_pieces: [[i32; 4]; 4],
    win_state: i32,
}

fn string_to_game_state(string: &str) -> GameState {
    let mut pieces = [[i32::default(); 4]; 4];
    let mut win = 0;
    let mut turn = 0;
    let mut data = string.split('|');

    turn = data.next().unwrap().parse::<i32>().unwrap();
    for i in 0..4 {
        for j in 0..4 {
            pieces[i][j] = data.next().unwrap().parse::<i32>().unwrap();
        }
    }
    win = data.next().unwrap().parse::<i32>().unwrap();
    GameState { current_turn: turn, player_pieces: pieces, win_state: win }
}

fn game_state_to_string(game_state: &GameState) -> String {
    let mut string = format!("{}|", game_state.current_turn);
    let pieces = game_state.player_pieces;
    for i in 0..4 {
        for j in 0..4 {
            string += &format!("{}|", pieces[i][j]);
        }
    }
    string += &format!("{}", game_state.win_state);
    string
}