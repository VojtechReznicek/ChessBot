//! This module manages all the information about the current game state (chess board and some data for specific chess rules like castling, en passant etc.)
//! and also the functions for making changes on this structure.
//! The chess board is represented as 12 bitboards each for one of the 6 types of pieces for both black and white.
//! Bitboard is an u64 number where each bit is one square on the board and 1 means the square is occupied by the specific piece, otherwise it's empty.
//! This lets me use masks and superfast bitwise operations for moving the pieces (deleting and/or appending them on the board)

#![allow(non_upper_case_globals)]

use crate::board::SideToMove::{Black, White};
use crate::movegen::{Flag, Move};

pub const P: usize = 0;
pub const N: usize = 1;
pub const B: usize = 2;
pub const R: usize = 3;
pub const Q: usize = 4;
pub const K: usize = 5;
pub const p: usize = 6;
pub const n: usize = 7;
pub const b: usize = 8;
pub const r: usize = 9;
pub const q: usize = 10;
pub const k: usize = 11;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
/// The main structure containing all important information about the current game state
pub struct GameState {
    /// The current board position
    pub bitboards: [u64; 12],
    /// The square where to go to take En Passant (NOT where the pawn actually is)
    pub en_passant_mask: u64,
    pub side_to_move: SideToMove,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u8,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SideToMove {
    Black,
    White,
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}
#[derive(Clone, Copy, Debug)]
pub struct Undo {
    pub captured_piece: Option<usize>,
    pub castling_rights: CastlingRights,
    pub en_passant_mask: u64,
    pub halfmove_clock: u8,
}

impl GameState {
    pub fn new() -> Self {
        let mut pieces = [0; 12];

        pieces[P] = 0x0000_0000_0000_ff00;
        pieces[N] = 0x0000_0000_0000_0042;
        pieces[B] = 0x0000_0000_0000_0024;
        pieces[R] = 0x0000_0000_0000_0081;
        pieces[Q] = 0x0000_0000_0000_0008;
        pieces[K] = 0x0000_0000_0000_0010;

        pieces[p] = 0x00ff_0000_0000_0000;
        pieces[n] = 0x4200_0000_0000_0000;
        pieces[b] = 0x2400_0000_0000_0000;
        pieces[r] = 0x8100_0000_0000_0000;
        pieces[q] = 0x0800_0000_0000_0000;
        pieces[k] = 0x1000_0000_0000_0000;

        let side_to_move = SideToMove::White;
        let castling_rights= CastlingRights{
            white_queenside: true,
            white_kingside: true,
            black_queenside: true,
            black_kingside: true,
        };
        let halfmove_clock = 0;
        let en_passant_mask = 0;
        Self { bitboards: pieces, side_to_move, castling_rights, halfmove_clock, en_passant_mask }
    }


    pub fn make_uci_moves(&mut self, moves: &[String]) {
        if moves.len() % 2 == 0 {
            self.side_to_move = SideToMove::White;
        }
        else {
            self.side_to_move = SideToMove::Black;
        }
        for m in moves {

            let bytes = m.as_bytes();
            if m.len() != 4 && m.len() != 5 {
                eprintln!("Invalid move skipped: {}", m);
                continue;
            }
            else if !((b'a'..=b'h').contains(&bytes[0]) && (b'1'..=b'8').contains(&bytes[1]) && (b'a'..=b'h').contains(&bytes[2]) && (b'1'..=b'8').contains(&bytes[3])) {
                eprintln!("Invalid move skipped: {}", m);
                continue;
            }

            let from_mask = square_to_mask(&m[0..2]);
            let to_mask = square_to_mask(&m[2..4]);
            let mut captured_a_piece = false;

            let move_mask = from_mask | to_mask;

            self.en_passant_mask = 0;

            if m.len() == 5 {  // Promoting
                for i in 0..12 {
                    if (self.bitboards[i] & to_mask) != 0 {   // Capture
                        self.bitboards[i] &= !to_mask;
                        break;
                    }
                }
                if (self.bitboards[P] & from_mask) != 0 { // White pawn promoting
                    self.bitboards[P] ^= from_mask;
                    let promoting_type = match m.as_bytes()[4] {
                        b'n' => N,
                        b'b' => B,
                        b'r' => R,
                        b'q' => Q,
                        _ => { eprintln!("Invalid move skipped: {}", m); continue; }
                    };
                    self.bitboards[promoting_type] ^= to_mask;
                }
                else {    // Black pawn promoting
                    self.bitboards[p] ^= from_mask;
                    let promoting_type = match m.as_bytes()[4] {
                        b'n' => n,
                        b'b' => b,
                        b'r' => r,
                        b'q' => q,
                        _ => { eprintln!("Invalid move skipped: {}", m); continue; }
                    };
                    self.bitboards[promoting_type] ^= to_mask;
                }
                self.halfmove_clock = 0;
                continue;
            }
            else {  // Normal move, castle, En Passant
                let is_pawn_moving = (self.bitboards[P] & from_mask) != 0 || (self.bitboards[p] & from_mask) != 0;

                for i in 0..12 {
                    if (self.bitboards[i] & to_mask) != 0 {   // Usual capture
                        self.bitboards[i] &= !to_mask;
                        captured_a_piece = true;
                        self.halfmove_clock = 0;
                        break;
                    }
                }

                if is_pawn_moving && m.as_bytes()[0] != m.as_bytes()[2] && !captured_a_piece { // En Passant
                    let col = m.as_bytes()[2] - b'a';
                    let row = m.as_bytes()[1] - b'1';
                    let index = row * 8 + col;
                    let en_passant_capture_mask = 1u64 << index;

                    self.bitboards[p] &= !en_passant_capture_mask;
                    self.bitboards[P] &= !en_passant_capture_mask;
                }

                for i in 0..12 { // The moNotve
                    if (self.bitboards[i] & from_mask) != 0 {
                        self.bitboards[i] ^= move_mask;
                        if (i == P || i == p) && from_mask.trailing_zeros().abs_diff(to_mask.trailing_zeros()) == 16 {
                            if i == p {
                                self.en_passant_mask = to_mask << 8;
                            }
                            else {
                                self.en_passant_mask = to_mask >> 8;
                            }
                        }
                        break;
                    }
                }

                match m.as_ref() {  // Castling
                    "e1g1" => if (self.bitboards[K] & to_mask) != 0 { self.make_simple_uci_move("h1f1") },
                    "e1c1" => if (self.bitboards[K] & to_mask) != 0 { self.make_simple_uci_move("a1d1") },
                    "e8g8" => if (self.bitboards[k] & to_mask) != 0 { self.make_simple_uci_move("h8f8") },
                    "e8c8" => if (self.bitboards[k] & to_mask) != 0 { self.make_simple_uci_move("a8d8") },
                    _ => {}
                }
                if is_pawn_moving || captured_a_piece {
                    self.halfmove_clock = 0;
                }
                else { // nothing reset the halfmove clock
                    self.halfmove_clock += 1;
                }
            }

            match &m[0..2] { // Rook or King moved and lost castling rights
                "a1" => self.castling_rights.white_queenside = false,
                "h1" => self.castling_rights.white_kingside = false,
                "e1" => {
                    self.castling_rights.white_queenside = false;
                    self.castling_rights.white_kingside = false;
                },
                "a8" => self.castling_rights.black_queenside = false,
                "h8" => self.castling_rights.black_kingside = false,
                "e8" => {
                    self.castling_rights.black_queenside = false;
                    self.castling_rights.black_kingside = false;
                }
                _ => {}
            }
            match &m[2..4] { // Rook was captured (so it can't castle)
                "a1" => self.castling_rights.white_queenside = false,
                "h1" => self.castling_rights.white_kingside = false,
                "a8" => self.castling_rights.black_queenside = false,
                "h8" => self.castling_rights.black_kingside = false,
                _ => {}
            }
        }
    }

    /// Assist function for simply moving one piece (without capturing)
    fn make_simple_uci_move(&mut self, m: &str) {
        let from_mask = square_to_mask(&m[0..2]);
        let to_mask = square_to_mask(&m[2..4]);
        let move_mask = from_mask | to_mask;
        for i in 0..12 {
            if (self.bitboards[i] & from_mask) != 0 {  // The move
                self.bitboards[i] ^= move_mask;
                break;
            }
        }
    }

    pub fn make_move(&mut self, mov: Move) -> Undo {
        let from_mask: u64 = 1u64 << mov.from;
        let to_mask: u64 = 1u64 << mov.to;
        let move_mask = from_mask | to_mask;
        let friendly_pieces_indexes: [usize; 6];
        let enemy_pieces_indexes: [usize; 6];
        let mut was_pawn_move = false;

        // For unmake:
        let old_castling_rights = self.castling_rights;
        let old_en_passant_mask = self.en_passant_mask;
        let old_halfmove_clock = self.halfmove_clock;
        let mut captured_piece = None;
        //

        self.en_passant_mask = 0;
        if self.side_to_move == White {
            friendly_pieces_indexes = [0, 1, 2, 3, 4, 5];
            enemy_pieces_indexes = [6, 7, 8, 9, 10, 11];
        } else {
            friendly_pieces_indexes = [6, 7, 8, 9, 10, 11];
            enemy_pieces_indexes = [0, 1, 2, 3, 4, 5];
        };

        if mov.flag == Flag::Normal {
            for i in friendly_pieces_indexes {
                if (self.bitboards[i] & from_mask) != 0 {
                    self.bitboards[i] ^= move_mask;
                    if i == P || i == p { was_pawn_move = true;}
                    break;
                }
            }
            self.halfmove_clock += 1;
            if was_pawn_move {self.halfmove_clock = 0;}
        }
        else if mov.flag == Flag::DoublePush { // Pawns only
            if self.side_to_move == White {
                self.bitboards[P] ^= move_mask;
                self.en_passant_mask = to_mask >> 8;
            } else {
                self.bitboards[p] ^= move_mask;
                self.en_passant_mask = to_mask << 8;
            }
            self.halfmove_clock = 0;
        }
        else if mov.flag == Flag::Capture {
            for i in enemy_pieces_indexes { // Capture
                if (self.bitboards[i] & to_mask) != 0 {
                    self.bitboards[i] ^= to_mask;
                    captured_piece = Some(i);
                    break;
                }
            }

            if captured_piece.is_none() {
                panic!("Fatal: Move from {} to {} is flagged as Capture, but there is no enemy on the square!", mov.from, mov.to);
            }

            for i in friendly_pieces_indexes { // Move
                if (self.bitboards[i] & from_mask) != 0 {
                    self.bitboards[i] ^= move_mask;
                    break;
                }
            }
            self.halfmove_clock = 0;
        }
        else if mov.flag == Flag::EnPassant {
            if self.side_to_move == White {
                self.bitboards[P] ^= move_mask;
                self.bitboards[p] ^= to_mask >> 8;
            } else {
                self.bitboards[p] ^= move_mask;
                self.bitboards[P] ^= to_mask << 8;
            }
            self.halfmove_clock = 0;
        }
        else if mov.flag == Flag::CastleKingside {
            if self.side_to_move == White {
                let white_kingside_castle_mask: u64 = 0xA0;
                self.bitboards[K] ^= move_mask;
                self.bitboards[R] ^= white_kingside_castle_mask;
                self.castling_rights.white_kingside = false;
            } else {
                let black_kingside_castle_mask: u64 = 0xA000_0000_0000_0000;
                self.bitboards[k] ^= move_mask;
                self.bitboards[r] ^= black_kingside_castle_mask;
                self.castling_rights.black_kingside = false;
            }
            self.halfmove_clock += 1;
        }
        else if mov.flag == Flag::CastleQueenside {
            if self.side_to_move == White {
                let white_queenside_castle_mask: u64 = 0x09;
                self.bitboards[K] ^= move_mask;
                self.bitboards[R] ^= white_queenside_castle_mask;
                self.castling_rights.white_queenside = false;
            } else {
                let black_queenside_castle_mask: u64 = 0x0900_0000_0000_0000;
                self.bitboards[k] ^= move_mask;
                self.bitboards[r] ^= black_queenside_castle_mask;
                self.castling_rights.black_queenside = false;
            }
            self.halfmove_clock += 1;
        }
        else if mov.flag == Flag::PromoteBishop || mov.flag == Flag::PromoteKnight || mov.flag == Flag::PromoteRook || mov.flag == Flag::PromoteQueen
            || mov.flag == Flag::PromoteBishopCapture || mov.flag == Flag::PromoteKnightCapture || mov.flag == Flag::PromoteRookCapture || mov.flag == Flag::PromoteQueenCapture
        {
            if self.side_to_move == White {
                if mov.flag == Flag::PromoteBishopCapture || mov.flag == Flag::PromoteKnightCapture || mov.flag == Flag::PromoteRookCapture || mov.flag == Flag::PromoteQueenCapture {
                    for i in enemy_pieces_indexes {
                        if (self.bitboards[i] & to_mask) != 0 {
                            self.bitboards[i] ^= to_mask;
                            captured_piece = Some(i);
                            break;
                        }
                    }
                }
                self.bitboards[P] ^= from_mask;
                let i = match mov.flag {
                    Flag::PromoteBishop => B,
                    Flag::PromoteBishopCapture => B,
                    Flag::PromoteKnight => N,
                    Flag::PromoteKnightCapture => N,
                    Flag::PromoteRook => R,
                    Flag::PromoteRookCapture => R,
                    Flag::PromoteQueen => Q,
                    Flag::PromoteQueenCapture => Q,
                    _ => unreachable!(),
                };
                self.bitboards[i] ^= to_mask;
            }
            else {
                if mov.flag == Flag::PromoteBishopCapture || mov.flag == Flag::PromoteKnightCapture || mov.flag == Flag::PromoteRookCapture || mov.flag == Flag::PromoteQueenCapture {
                    for i in enemy_pieces_indexes { // Capture
                        if (self.bitboards[i] & to_mask) != 0 {
                            self.bitboards[i] ^= to_mask;
                            captured_piece = Some(i);
                            break;
                        }
                    }
                }
                self.bitboards[p] ^= from_mask;
                let i = match mov.flag {
                    Flag::PromoteBishop => b,
                    Flag::PromoteBishopCapture => b,
                    Flag::PromoteKnight => n,
                    Flag::PromoteKnightCapture => n,
                    Flag::PromoteRook => r,
                    Flag::PromoteRookCapture => r,
                    Flag::PromoteQueen => q,
                    Flag::PromoteQueenCapture => q,
                    _ => unreachable!(),
                };
                self.bitboards[i] ^= to_mask;
            }
            self.halfmove_clock = 0;
        }

        match mov.from {
            4 => { self.castling_rights.white_kingside = false; self.castling_rights.white_queenside = false; }
            60 => { self.castling_rights.black_kingside = false; self.castling_rights.black_queenside = false; }
            0 => self.castling_rights.white_queenside = false,
            7 => self.castling_rights.white_kingside = false,
            56 => self.castling_rights.black_queenside = false,
            63 => self.castling_rights.black_kingside = false,
            _ => {}
        }
        match mov.to {
            0 => self.castling_rights.white_queenside = false,
            7 => self.castling_rights.white_kingside = false,
            56 => self.castling_rights.black_queenside = false,
            63 => self.castling_rights.black_kingside = false,
            _ => {}
        }

        if self.side_to_move == White {
            self.side_to_move = SideToMove::Black;
        }
        else {
            self.side_to_move = White;
        }
        Undo{
            captured_piece,
            castling_rights: old_castling_rights,
            en_passant_mask: old_en_passant_mask,
            halfmove_clock: old_halfmove_clock,
        }
    }
    pub fn unmake_move(&mut self, mov: Move, undo: Undo) {
        if self.side_to_move == White {
            self.side_to_move = Black;
        }
        else {
            self.side_to_move = White;
        }
        self.castling_rights = undo.castling_rights;
        self.en_passant_mask = undo.en_passant_mask;
        self.halfmove_clock = undo.halfmove_clock;
        let from_mask = 1u64 << mov.from;
        let to_mask = 1u64 << mov.to;
        let move_mask = from_mask | to_mask;
        let friendly_pieces_indexes: [usize; 6];



        if self.side_to_move == White {
            friendly_pieces_indexes = [0, 1, 2, 3, 4, 5];
        } else {
            friendly_pieces_indexes = [6, 7, 8, 9, 10, 11];
        };

        if mov.flag == Flag::Normal {
            for i in friendly_pieces_indexes {
                if (self.bitboards[i] & to_mask) != 0 {
                    self.bitboards[i] ^= move_mask;
                    break;
                }
            }
        }
        else if mov.flag == Flag::DoublePush { // Pawns only
            if self.side_to_move == White {
                self.bitboards[P] ^= move_mask;
            } else {
                self.bitboards[p] ^= move_mask;
            }
        }
        else if mov.flag == Flag::Capture {
            let captured = undo.captured_piece;

            for i in friendly_pieces_indexes { // Move
                if (self.bitboards[i] & to_mask) != 0 {
                    self.bitboards[i] ^= move_mask;
                    break;
                }
            }
            self.bitboards[captured.unwrap()] ^= to_mask;
        }
        else if mov.flag == Flag::EnPassant {
            for i in friendly_pieces_indexes {
                if (self.bitboards[i] & to_mask) != 0 {
                    self.bitboards[i] ^= move_mask;
                }
            }
            if self.side_to_move == White {
                self.bitboards[p] ^= to_mask >> 8;
            }
            else {
                self.bitboards[P] ^= to_mask << 8;
            }
        }
        else if mov.flag == Flag::CastleKingside {
            if self.side_to_move == White {
                let white_kingside_castle_mask: u64 = 0xA0;
                self.bitboards[K] ^= move_mask;
                self.bitboards[R] ^= white_kingside_castle_mask;
            } else {
                let black_kingside_castle_mask: u64 = 0xA000_0000_0000_0000;
                self.bitboards[k] ^= move_mask;
                self.bitboards[r] ^= black_kingside_castle_mask;
            }
        }
        else if mov.flag == Flag::CastleQueenside {
            if self.side_to_move == White {
                let white_queenside_castle_mask: u64 = 0x09;
                self.bitboards[K] ^= move_mask;
                self.bitboards[R] ^= white_queenside_castle_mask;
            } else {
                let black_queenside_castle_mask: u64 = 0x0900_0000_0000_0000;
                self.bitboards[k] ^= move_mask;
                self.bitboards[r] ^= black_queenside_castle_mask;
            }
        }
        else if mov.flag == Flag::PromoteBishop || mov.flag == Flag::PromoteKnight || mov.flag == Flag::PromoteRook || mov.flag == Flag::PromoteQueen
        || mov.flag == Flag::PromoteBishopCapture || mov.flag == Flag::PromoteKnightCapture || mov.flag == Flag::PromoteRookCapture || mov.flag == Flag::PromoteQueenCapture {
            if self.side_to_move == White {
                self.bitboards[P] ^= from_mask;
            } else {
                self.bitboards[p] ^= from_mask;
            }

            for i in friendly_pieces_indexes {
                if (self.bitboards[i] & to_mask) != 0 {
                    self.bitboards[i] ^= to_mask;
                    break;
                }
            }
            if mov.flag == Flag::PromoteBishopCapture || mov.flag == Flag::PromoteKnightCapture || mov.flag == Flag::PromoteRookCapture || mov.flag == Flag::PromoteQueenCapture {
                let captured = undo.captured_piece;
                self.bitboards[captured.unwrap()] ^= to_mask;
            }
        }
    }
}



/// Makes a bitboard with a single bit from given coordinates
pub fn square_to_mask(square: &str) -> u64 {
    let bytes = square.as_bytes();
    let col = bytes[0] - b'a';
    let row = bytes[1] - b'1';
    let index = row * 8 + col;
    1u64 << index
}


/// Converts index of a square to proper chess notation
pub fn index_to_algebraic(index: u8) -> String {
    let col = index % 8;
    let row = index / 8;

    let file = (b'a' + col) as char;
    let rank = (b'1' + row) as char;

    format!("{}{}", file, rank)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_and_index_conversions() {
        assert_eq!(square_to_mask("a1"), 1u64);
        assert_eq!(square_to_mask("h8"), 1u64 << 63);
        assert_eq!(index_to_algebraic(0), "a1");
        assert_eq!(index_to_algebraic(63), "h8");

        for index in 0..64 {
            let notation = index_to_algebraic(index);
            let mask = square_to_mask(&notation);
            assert_eq!(mask, 1u64 << index);
        }
    }

    #[test]
    fn test_make_and_unmake_move() {
        let moves: Vec<Move> = vec![
        Move{
            from: 12,
            to: 28,
            flag: Flag::DoublePush
        },
        Move{
            from: 11,
            to: 27,
            flag: Flag::DoublePush
        },
        Move{
            from: 1,
            to: 18,
            flag: Flag::Normal
        },
        Move{
            from: 6,
            to: 21,
            flag: Flag::Normal
        },
        Move{
            from: 6,
            to: 23,
            flag: Flag::Normal
        },
        ];


        let board2 = GameState::new();
        for mov in moves {
            let mut board1 = GameState::new();
            let undo =board1.make_move(mov);
            board1.unmake_move(mov, undo);
            assert_eq!(board1, board2);
        }

    }
    #[test]
    fn black_kingside_castle_moves_h8_rook_to_f8() {
        let mut g = GameState::new();
        g.bitboards[b] &= !(1u64 << 61); // clear f8
        g.bitboards[n] &= !(1u64 << 62); // clear g8
        g.side_to_move = SideToMove::Black;

        let castle = crate::movegen::generate_moves(&g)
            .into_iter()
            .find(|m| m.from == 60 && m.to == 62)
            .expect("castling to g8 should be generated");
        g.make_move(castle);

        assert_ne!(g.bitboards[r] & (1u64 << 61), 0, "rook should be on f8");
        assert_ne!(g.bitboards[r] & (1u64 << 56), 0, "a8 rook should not move");
    }

    #[test]
    fn king_move_make_unmake_restores_castling_rights() {
        let mut g = GameState::new();
        g.bitboards[P] &= !(1u64 << 12); // clear e2 so e1e2 is possible
        let before = g;

        let mov = Move { from: 4, to: 12, flag: Flag::Normal };
        let undo = g.make_move(mov);
        assert!(!g.castling_rights.white_kingside, "king moved, rights should be gone");

        g.unmake_move(mov, undo);
        assert_eq!(g, before, "unmake must restore the exact previous state");
    }
}
