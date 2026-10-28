mod board;
mod uci;
mod movegen;
mod movespreloader;

use crate::board::Bitboards;
use crate::movegen::{Flag, Move};

fn main() {
    let mut board = Bitboards::new();

    loop {
        let input = get_input();

        if input.is_empty() {
            continue;
        }
        uci::parse_uci(&input, &mut board);
    }
}

pub fn get_input() -> Vec<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|s| s.to_string()).collect()
}







fn index_to_algebraic(index: u8) -> String {
    let col = index % 8;
    let row = index / 8;

    let file = (b'a' + col) as char;
    let rank = (b'1' + row) as char;

    format!("{}{}", file, rank)
}

pub fn print_generated_moves(moves: &Vec<Move>) {
    println!("--- Generated {} pseudo-legal moves ---", moves.len());

    for m in moves {
        let from_sq = index_to_algebraic(m.from);
        let to_sq = index_to_algebraic(m.to);

        let flag_suffix = match m.flag {
            Flag::Normal => "",
            Flag::Castle => " [Castle]",
            Flag::Promote => " [Promote]",
            Flag::DoublePush => " [DoublePush]",
            Flag::Capture => " [Capture]",
        };

        println!("{}{}{}", from_sq, to_sq, flag_suffix);
    }
}


/*
position startpos moves e2e4 a7a6 g1f3 b7b6 f1c4 h7h6
position startpos moves d2d4 a7a6 b1c3 b7b6 c1f4 h7h6 d1d2 g7g6
*/