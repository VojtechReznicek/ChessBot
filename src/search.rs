//! This module implements the Negamax search algorithm.
//! Negamax is used instead of standard Minimax because they are mathematically equivalent in zero-sum games,
//! but Negamax simplifies the implementation into a single maximizing loop.
//! Returning values are simply negated between plies (what is great for White is equally terrible for Black).

use crate::board::GameState;
use crate::eval::evaluate;
use crate::movegen::{generate_moves, Move};

fn inner_negamax(board: &mut GameState, depth: i8) -> i32 {
    if depth == 0 {
        evaluate(board)
    }
    else {
        let moves = generate_moves(board);
        let mut best: i32 = -10_000_000;

        for mov in moves {
            let undo = board.make_move(mov);
            let candidate = -(inner_negamax(board, depth - 1));
            if candidate > best {
                best = candidate;
            }
            board.unmake_move(mov, undo);
        }
        best
    }
}


pub fn root_negamax(board: &mut GameState, depth: i8) -> Option<Move> {
    let moves = generate_moves(board);
    if moves.is_empty() {
        return None;
    }
    else {
        let mut best: i32 = -10_000_000;
        let mut best_mov: Move = moves[0];
        for mov in moves {
            let undo = board.make_move(mov);
            let candidate = -(inner_negamax(board, depth - 1));
            if candidate > best {
                best = candidate;
                best_mov = mov;
            }
            board.unmake_move(mov, undo);
        }
        Some(best_mov)
    }

}