//! This module handles Zobrist hashing features. The keys are pre-generated in the compile time as const numbers.
//! Zobrist hashing is good for identifying positions that were evaluated before to avoid losing time on them.

use crate::board::{CastlingRights, GameState};
use crate::board::SideToMove::White;

pub struct ZobristKeys {
    pub pieces: [[u64; 64]; 12],
    pub side_to_move: u64,
    pub castling: [u64; 16],
    pub en_passant: [u64; 8]
}
const SEED: u64 = 94498698040556907;
pub const KEYS: ZobristKeys = generate_keys(SEED);

const fn generate_key(seed: &mut u64) -> u64 {
    let mut res = *seed;
    res ^= res << 5;
    res ^= res >> 13;
    res ^= res << 10;
    res ^= *seed;
    res
}

const fn generate_keys(seed: u64) -> ZobristKeys {
    let mut res = ZobristKeys{
        pieces: [[0; 64]; 12],
        side_to_move: 0,
        castling: [0; 16],
        en_passant: [0; 8]
    };
    let mut state: u64 = seed;
    let mut i = 0;
    while i < res.pieces.len() {
        let mut j = 0;
        while j < res.pieces[i].len(){
            res.pieces[i][j] = generate_key(&mut state);
            change_seed_state(&mut state);
            j += 1;
        }
        i += 1
    }

    res.side_to_move = generate_key(&mut state);
    change_seed_state(&mut state);

    i = 0;
    while i < res.castling.len(){
        res.castling[i] = generate_key(&mut state);
        change_seed_state(&mut state);
        i += 1;
    }

    i = 0;
    while i < res.en_passant.len(){
        res.en_passant[i] = generate_key(&mut state);
        change_seed_state(&mut state);
        i += 1;
    }

    res
}

const fn change_seed_state(seed: &mut u64) {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
}


pub fn hash_position(board: &GameState) -> u64 {
    let mut res = 0;
    for (i, bitboard) in board.bitboards.iter().enumerate() {
        let mut bb = *bitboard;
        while bb != 0 {
            let sq = bb.trailing_zeros() as usize;
            res ^= KEYS.pieces[i][sq];

            bb &= bb - 1;
        }

    }

    if board.side_to_move == White {
        res ^= KEYS.side_to_move;
    }

    if board.en_passant_mask != 0 {
        let i = (board.en_passant_mask.trailing_zeros() % 8) as usize;
        res ^= KEYS.en_passant[i];
    }

    let mut castling_index = 0;
    if board.castling_rights.black_kingside { castling_index += 1; }
    if board.castling_rights.white_kingside { castling_index += 2; }
    if board.castling_rights.black_queenside { castling_index += 4; }
    if board.castling_rights.white_queenside { castling_index += 8; }

    res ^= KEYS.castling[castling_index];


    res
}

pub fn castling_key(rights: CastlingRights) -> u64 {
    let mut i = 0;
    if rights.black_kingside { i += 1; }
    if rights.white_kingside { i += 2; }
    if rights.black_queenside { i += 4; }
    if rights.white_queenside { i += 8; }
    KEYS.castling[i]
}
pub fn en_passant_key(ep_mask: u64) -> u64 {
    if ep_mask == 0 { 0 }
    else {
        let col = (ep_mask.trailing_zeros() % 8) as usize;
        KEYS.en_passant[col]
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transposition_gives_same_hash() {
        let mut board_a = GameState::new();
        let moves1: Vec<String> = ["g1f3", "g8f6", "b1c3", "b8c6"]
            .iter()
            .map(|m| m.to_string())
            .collect();
        board_a.make_uci_moves(&moves1);

        let mut board_b = GameState::new();
        let moves2: Vec<String> = ["b1c3", "g8f6", "g1f3", "b8c6"]
            .iter()
            .map(|m| m.to_string())
            .collect();

        board_b.make_uci_moves(&moves2);

        assert_eq!(hash_position(&board_a), hash_position(&board_b));
    }

    #[test]
    fn different_state_flags_change_hash() {
        let b_white = GameState::fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let b_black = GameState::fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
        let b_no_castle = GameState::fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1");
        let b_ep = GameState::fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");

        let h_white = hash_position(&b_white);
        assert_ne!(h_white, hash_position(&b_black));
        assert_ne!(h_white, hash_position(&b_no_castle));
        assert_ne!(h_white, hash_position(&b_ep));
    }

}