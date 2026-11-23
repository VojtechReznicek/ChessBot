//! This module implements the Negamax search algorithm.
//! Negamax is used instead of standard Minimax because they are mathematically equivalent in zero-sum games,
//! but Negamax simplifies the implementation into a single maximizing loop.
//! Returning values are simply negated between plies (what is great for White is equally terrible for Black).

use std::time::{Duration, Instant};
use crate::board::{GameState, SideToMove, k, K};
use crate::eval::{evaluate, PIECE_VALUES};
use crate::movegen::{generate_moves, Move, Flag};

const BASIC_CAPTURE_BONUS: i32 = 1000;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TimeManagement {
    limit: Duration,
    start: Instant,
    nodes_passed: u64,
    stopped: bool,
}

fn inner_negamax(board: &mut GameState, depth: i8, mut alpha: i32, beta: i32, time: &mut TimeManagement) -> i32 {
    if depth == 0 {
        evaluate(board)
    } else {
        let mut moves = generate_moves(board);
        sort_moves(&board, &mut moves);
        let mut legal_moves = 0;
        let mut best: i32 = -10_000_000;

        let is_white = board.side_to_move == SideToMove::White;
        for mov in moves {
            time.nodes_passed += 1;
            if time.nodes_passed % 2048 == 0 { // Check time

                if time.start.elapsed() >= time.limit {
                    time.stopped = true;
                    return 0
                }
            }


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

            let candidate = -(inner_negamax(board, depth - 1, -beta, -alpha, time));
            board.unmake_move(mov, undo);
            if time.stopped{ return 0 }

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

pub fn root_negamax(board: &mut GameState, depth: Option<i8>, limit: u64, increment: u64) -> Option<Move> {

    let max_safe_time = limit.saturating_sub(40).max(10);
    let allocated = ((limit / 20) + ((increment / 5) * 3))
        .saturating_sub(40)
        .clamp(10, max_safe_time);

    let mut time: TimeManagement = TimeManagement {
        limit: Duration::from_millis(allocated),
        start: Instant::now(),
        nodes_passed: 0,
        stopped: false,
    };


    let mut moves = generate_moves(board);
    if depth.is_some() {
        time.limit = Duration::MAX;
    }
    let depth = depth.unwrap_or(127);

    if moves.is_empty() {
        return None;
    }
    sort_moves(&board, &mut moves);

    let mut best: i32;
    let mut alpha: i32;
    let mut best_mov: Option<Move> = None;

    let mut best_depth_move: Option<Move>;

    for i in 1..=depth {
        best = -10_000_000;
        alpha = -10_000_000;
        best_depth_move = None;
        
        let is_white = board.side_to_move == SideToMove::White;
        for mov in &moves {
            let undo = board.make_move(*mov);

            let king_index = if is_white {
                board.bitboards[K].trailing_zeros() as u8
            } else {
                board.bitboards[k].trailing_zeros() as u8
            };

            if board.is_attacked(king_index, !is_white) {
                board.unmake_move(*mov, undo);
                continue
            }
            if best_mov.is_none() {
                best_mov = Some(*mov);
            }

            let candidate = -(inner_negamax(board, i - 1, -10_000_000, -alpha, &mut time));
            board.unmake_move(*mov, undo);

            if time.stopped{ break }
            if candidate > best {
                best = candidate;
                alpha = best;
                best_depth_move = Some(*mov);
            }
        }

        if time.stopped { break }
        best_mov = best_depth_move;
    }

    best_mov
}


fn priority_score(board: &GameState, mov: &Move) -> i32 {
    let mut score: i32 = 0;
    if mov.flag == Flag::Capture{
        let attacker_val = PIECE_VALUES[board.index_pos_to_piece_type(mov.from)].strict_abs();
        let victim_val = PIECE_VALUES[board.index_pos_to_piece_type(mov.to)].strict_abs();

        score += victim_val * 10 - attacker_val + BASIC_CAPTURE_BONUS;
    }

    // More flags (En Passant, Promotes, Promotes with Captures) to be added in the future

    score
}

fn sort_moves(board: &GameState, moves: &mut [Move]) {
    moves.sort_unstable_by_key(|mov| -priority_score(board, mov))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench_move_ordering() {
        let positions = [
            ("Startpos", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            ("Kiwipete (tactical)", "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"),
            ("Middlegame", "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1"),
        ];

        let depth = 5;
        let mut total_nodes: u64 = 0;
        let total_start = Instant::now();

        println!("\n--- Benchmark (depth {}) ---", depth);
        for (name, fen) in positions {
            let mut board = GameState::fen(fen);
            let mut time = TimeManagement {
                limit: Duration::MAX,
                start: Instant::now(),
                nodes_passed: 0,
                stopped: false,
            };

            let score = inner_negamax(&mut board, depth, -10_000_000, 10_000_000, &mut time);
            let ms = time.start.elapsed().as_millis();
            total_nodes += time.nodes_passed;

            println!(
                "{:<20} | nodes: {:>10} | time: {:>5} ms | score: {}",
                name, time.nodes_passed, ms, score
            );
        }

        let total_ms = total_start.elapsed().as_millis().max(1);
        let nps = (total_nodes as u128 * 1000) / total_ms;
        println!("---------------------------------------------------------------");
        println!(
            "TOTAL                | nodes: {:>10} | time: {:>5} ms | {} NPS\n",
            total_nodes, total_ms, nps
        );
    }
}