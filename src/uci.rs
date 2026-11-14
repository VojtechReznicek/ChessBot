//! This module handles all the UCI (universal chess interface) communications.
//! There are also debug functions like printing the board to the console. They launch only when --ascii argument is given

use crate::board::{index_to_algebraic, GameState, SideToMove};
use crate::eval::evaluate;
use crate::movegen::{Flag, Move};
use crate::search::root_negamax;

pub fn parse_uci(input: &[String], board: &mut GameState, ascii_mode: bool) {
    if input.is_empty() {
        return;
    }
    match input[0].as_str() {
        "uci" => {
            println!("id name ChessBot");
            println!("id author VojtechReznicek");
            println!("uciok");
        },
        "isready" => println!("readyok"),
        "position" => {
            *board = GameState::new();
            for (i, txt) in input.iter().enumerate() {
                if txt == "moves" {
                    board.make_uci_moves(&input[i + 1..]);
                    if ascii_mode {
                        print_board(board);
                    }
                    break;
                }

            }

        },
        "go" => {


            if ascii_mode {
                let moves = crate::movegen::generate_moves(board);
                println!("Position's value for the player on the move: {}", evaluate(board));
                if board.side_to_move == SideToMove::White {
                    println!("(it's a white move)\n");
                }
                else { println!("(it's a black move)\n"); }

                print_generated_moves(&moves);
            }


            if let Some(best_move) = root_negamax(board, 4) {
                let from_sq = index_to_algebraic(best_move.from);
                let to_sq = index_to_algebraic(best_move.to);

                let flag_char = match best_move.flag {
                    crate::movegen::Flag::PromoteQueen | crate::movegen::Flag::PromoteQueenCapture => "q",
                    crate::movegen::Flag::PromoteRook | crate::movegen::Flag::PromoteRookCapture => "r",
                    crate::movegen::Flag::PromoteBishop | crate::movegen::Flag::PromoteBishopCapture => "b",
                    crate::movegen::Flag::PromoteKnight | crate::movegen::Flag::PromoteKnightCapture => "n",
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

pub fn get_input() -> Vec<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|s| s.to_string()).collect()
}



pub fn print_generated_moves(moves: &Vec<Move>) {
    println!("--- Generated {} pseudo-legal moves ---", moves.len());
    for m in moves {
        let from_sq = index_to_algebraic(m.from);
        let to_sq = index_to_algebraic(m.to);

        let flag_suffix = match m.flag {
            Flag::Normal => "",
            Flag::CastleKingside => " [CastleKingside]",
            Flag::CastleQueenside => " [CastleQueenside]",
            Flag::DoublePush => " [DoublePush]",
            Flag::Capture => " [Capture]",
            Flag::PromoteQueen => " [PromoteQueen]",
            Flag::PromoteRook => " [PromoteRook]",
            Flag::PromoteBishop => " [PromoteBishop]",
            Flag::PromoteKnight => " [PromoteKnight]",
            Flag::PromoteQueenCapture => " [PromoteQueenCapture]",
            Flag::PromoteRookCapture => " [PromoteRookCapture]",
            Flag::PromoteBishopCapture => " [PromoteBishopCapture]",
            Flag::PromoteKnightCapture => " [PromoteKnightCapture]",
            Flag::EnPassant => " [EnPassant]",
        };

        println!("{}{}{}", from_sq, to_sq, flag_suffix);
    }
}


pub fn print_board(board: &GameState) {
    println!("  a b c d e f g h");

    let piece_chars = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'];

    for row in (0..8).rev() {
        print!("{} ", row + 1);
        for col in 0..8 {
            let index = row * 8 + col;
            let mask = 1u64 << index;
            let mut current_piece = '.';

            for i in 0..12 {
                if (board.bitboards[i] & mask) != 0 {
                    current_piece = piece_chars[i];
                    break;
                }
            }
            print!("{} ", current_piece);
        }
        println!();
    }
    println!();
}