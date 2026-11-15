//! This module implements the Negamax search algorithm.
//! Negamax is used instead of standard Minimax because they are mathematically equivalent in zero-sum games,
//! but Negamax simplifies the implementation into a single maximizing loop.
//! Returning values are simply negated between plies (what is great for White is equally terrible for Black).

use crate::board::{GameState, SideToMove, k, K};
use crate::eval::evaluate;
use crate::movegen::{generate_moves, Move};

fn inner_negamax(board: &mut GameState, depth: i8) -> i32 {
    if depth == 0 {
        evaluate(board)
    }
    else {
        let moves = generate_moves(board);
        let mut count_of_moves = moves.len();
        let mut best: i32 = -10_000_000;

        let is_white = if board.side_to_move == SideToMove::White { true }  else {false};
        for mov in moves {
            let undo = board.make_move(mov);
            let king_index = if is_white {
                board.bitboards[K].trailing_zeros() as u8
            }
            else {board.bitboards[k].trailing_zeros() as u8};

            if board.is_attacked(king_index, !is_white) {
                board.unmake_move(mov, undo);
                count_of_moves -= 1;
                continue;
            }

            let candidate = -(inner_negamax(board, depth - 1));
            if candidate > best {
                best = candidate;
            }
            board.unmake_move(mov, undo);
        }
        if count_of_moves == 0 {
            let king_index = if is_white {
                board.bitboards[K].trailing_zeros() as u8
            }
            else {board.bitboards[k].trailing_zeros() as u8};

            if board.is_attacked(king_index, !is_white) { // checkmate
                return -100_000 - depth as i32;
            }
            else { return 0; } // stalemate
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
        let mut best_mov: Option<Move> = None;

        let is_white = if board.side_to_move == SideToMove::White { true }  else {false};
        for mov in moves {
            let undo = board.make_move(mov);

            let king_index = if is_white {
                board.bitboards[K].trailing_zeros() as u8
            }
            else {board.bitboards[k].trailing_zeros() as u8};

            if board.is_attacked(king_index, !is_white) {
                board.unmake_move(mov, undo);
                continue;
            }

            let candidate = -(inner_negamax(board, depth - 1));
            if candidate > best {
                best = candidate;
                best_mov = Some(mov);
            }
            board.unmake_move(mov, undo);
        }
        best_mov
    }
}