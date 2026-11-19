//! This module implements the Negamax search algorithm.
//! Negamax is used instead of standard Minimax because they are mathematically equivalent in zero-sum games,
//! but Negamax simplifies the implementation into a single maximizing loop.
//! Returning values are simply negated between plies (what is great for White is equally terrible for Black).

use crate::board::{GameState, SideToMove, k, K};
use crate::eval::{evaluate, PIECE_VALUES};
use crate::movegen::{generate_moves, Move, Flag};

const BASIC_CAPTURE_BONUS: i32 = 1000;


fn inner_negamax(board: &mut GameState, depth: i8, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        evaluate(board)
    } else {
        let moves = generate_moves(board);
        let mut legal_moves = 0;
        let mut best: i32 = -10_000_000;

        let is_white = board.side_to_move == SideToMove::White;
        for mov in moves {
            let undo = board.make_move(mov);
            let king_index = if is_white {
                board.bitboards[K].trailing_zeros() as u8
            } else {
                board.bitboards[k].trailing_zeros() as u8
            };

            if board.is_attacked(king_index, !is_white) { // In a check
                board.unmake_move(mov, undo);
                continue;
            }

            legal_moves += 1;

            let candidate = -(inner_negamax(board, depth - 1, -beta, -alpha));
            board.unmake_move(mov, undo);

            if candidate > best {
                best = candidate;
                if best > alpha {
                    alpha = best;
                }
            }

            if alpha >= beta {
                break;
            }
        }

        if legal_moves == 0 {
            let king_index = if is_white {
                board.bitboards[K].trailing_zeros() as u8
            } else {
                board.bitboards[k].trailing_zeros() as u8
            };

            return if board.is_attacked(king_index, !is_white) {
                -100_000 - depth as i32 // Checkmate
            } else {
                0 // Stalemate
            }
        }

        best
    }
}

pub fn root_negamax(board: &mut GameState, depth: i8) -> Option<Move> {
    let moves = generate_moves(board);
    if moves.is_empty() {
        return None;
    }

    let mut best: i32 = -10_000_000;
    let mut alpha: i32 = -10_000_000;
    let mut best_mov: Option<Move> = None;

    let is_white = board.side_to_move == SideToMove::White;
    for mov in moves {
        let undo = board.make_move(mov);

        let king_index = if is_white {
            board.bitboards[K].trailing_zeros() as u8
        } else {
            board.bitboards[k].trailing_zeros() as u8
        };

        if board.is_attacked(king_index, !is_white) {
            board.unmake_move(mov, undo);
            continue;
        }

        let candidate = -(inner_negamax(board, depth - 1, -10_000_000, -alpha));
        board.unmake_move(mov, undo);

        if candidate > best {
            best = candidate;
            alpha = best;
            best_mov = Some(mov);
        }
    }

    best_mov
}

fn priority_score(board: &GameState, mov: Move) -> i32 {
    let mut score: i32 = 0;
    if mov.flag == Flag::Capture{
        let bonus = 1000;
        let attacker_val = PIECE_VALUES[board.index_pos_to_piece_type(mov.from)].strict_abs();
        let victim_val = PIECE_VALUES[board.index_pos_to_piece_type(mov.to)].strict_abs();

        score += victim_val * 10 - attacker_val + bonus;
    }

    // More will be added in the future

    score
}

