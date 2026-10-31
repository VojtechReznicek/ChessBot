//! This is the main file with the main loop

mod board;
mod uci;
mod movegen;
mod movespreloader;

use crate::board::GameState;
use crate::uci::get_input;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut ascii_mode = false;
    for i in 1..args.len() {
        if args[i] == "--ascii" {
            ascii_mode = true;
        }
    }
    let mut board = GameState::new();

    loop {
        let input = get_input();

        if input.is_empty() {
            continue;
        }
        uci::parse_uci(&input, &mut board, ascii_mode);
    }
}

/*
position startpos moves e2e4 a7a6 g1f3 b7b6 f1c4 h7h6
position startpos moves d2d4 a7a6 b1c3 b7b6 c1f4 h7h6 d1d2 g7g6
*/