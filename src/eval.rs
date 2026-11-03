//! This module contains the logic for evaluating chess positions.
//! It always returns a value from the perspective of the player whose turn it is.
//! If the position is good, the value is positive, otherwise the value is negative

use crate::board::{GameState, P, p, N, n, B, b, R, r, Q, q, K, k, SideToMove};
use crate::movegen::{FILE_A, FILE_H};

const PIECE_VALUES: [i32; 12] = [
    100, 310, 320, 500, 900, 0,
    -100, -310, -320, -500, -900, -0,
];

pub fn evaluate(board: &GameState) -> i32 {
    let white_active_pieces = board.bitboards[P] | board.bitboards[N] | board.bitboards[B] | board.bitboards[R] | board.bitboards[Q];
    let black_active_pieces = board.bitboards[p] | board.bitboards[n] | board.bitboards[b] | board.bitboards[r] | board.bitboards[q];
    let mut value: i32 = 0;
    value += king_alive(board);
    value += count_pure_material_value(board);
    value += pawns_covering_bonus(board);
    value += aggression_bonus(white_active_pieces, black_active_pieces);

    if board.side_to_move == SideToMove::Black { value *= -1; }
    value
}

fn king_alive(board: &GameState) -> i32 {
    let mut value: i32 = 0;
    value += if board.bitboards[K] != 0 { 30_000 }
        else { 0 };

    value -= if board.bitboards[k] != 0 { 30_000 }
        else { 0 };

    value
}

fn count_pure_material_value(board: &GameState) -> i32 {
    let mut value: i32 = 0;
    for i in 0..12 {
        value += board.bitboards[i].count_ones() as i32 * PIECE_VALUES[i];
    }
    value
}

fn pawns_covering_bonus(board: &GameState) -> i32 {
    let mut value: i32 = 0;
    let white_pawns = board.bitboards[P];
    let black_pawns = board.bitboards[p];

    let white_attacks_left = (white_pawns & !FILE_A) << 7;
    let white_attacks_right = (white_pawns & !FILE_H) << 9;
    let white_defended_pawns = white_pawns & (white_attacks_left | white_attacks_right);
    value += white_defended_pawns.count_ones() as i32 * 10;

    let black_attacks_left = (black_pawns & !FILE_H) >> 7;
    let black_attacks_right = (black_pawns & !FILE_A) >> 9;
    let black_defended_pawns = black_pawns & (black_attacks_left | black_attacks_right);
    value -= black_defended_pawns.count_ones() as i32 * 10;

    value
}

/// Bonuses all pieces in opponent's side by 5 points
fn aggression_bonus(white_pieces: u64, black_pieces: u64) -> i32 {
    let mut value: i32 = 0;
    let white_half: u64 = 0x0000_0000_FFFF_FFFF;
    let black_half: u64 = 0xFFFF_FFFF_0000_0000;
    value += (white_pieces & black_half).count_ones() as i32 * 5;
    value -= (black_pieces & white_half).count_ones() as i32 * 5;
    value
}