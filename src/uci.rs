use crate::board::Bitboards;
use crate::{index_to_algebraic, main, print_generated_moves};

pub fn parse_uci(input: &[String], board: &mut Bitboards) {
    match input[0].as_str() {
        "uci" => {
            println!("id name ChessBot");
            println!("id author VojtechReznicek");
            println!("uciok");
        },
        "isready" => println!("readyok"),
        "position" => {
            if input.len() > 1 && input[1] == "startpos" {
                *board = Bitboards::new();

                if input.len() > 3 && input[2] == "moves" {

                    board.make_moves(&input[3..]);
                }

                board.print_board();
            }
        },
        "go" => {
            let moves = crate::movegen::generate_moves(board);

            print_generated_moves(&moves);

            if let Some(first_move) = moves.first() {
                let from_sq = index_to_algebraic(first_move.from);
                let to_sq = index_to_algebraic(first_move.to);

                let flag_char = match first_move.flag {
                    crate::movegen::Flag::Promote => "q", 
                    _ => "",
                };

                println!("bestmove {}{}{}", from_sq, to_sq, flag_char);
            } 
            else {  // checkmate
                println!("bestmove (none)"); 
            }
        },
        "quit" => std::process::exit(0),
        _ => {}
    }
}
