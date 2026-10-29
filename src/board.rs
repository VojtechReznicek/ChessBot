#![allow(non_upper_case_globals)]

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

pub struct Bitboards {
    pub pieces: [u64; 12],
    pub en_passant_mask: u64,
    pub side_to_move: SideToMove,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u8,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SideToMove {
    Black,
    White,
}
#[derive()]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl Bitboards {
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

        let mut side_to_move = SideToMove::White;
        let mut castling_rights= CastlingRights{
            white_queenside: true,
            white_kingside: true,
            black_queenside: true,
            black_kingside: true,
        };
        let mut halfmove_clock = 0;
        let mut en_passant_mask = 0;
        Self { pieces, side_to_move, castling_rights, halfmove_clock, en_passant_mask }
    }

    pub fn print_board(&self) {
        println!("  a b c d e f g h");

        let piece_chars = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'];

        for row in (0..8).rev() {
            print!("{} ", row + 1);
            for col in 0..8 {
                let index = row * 8 + col;
                let mask = 1u64 << index;
                let mut current_piece = '.';

                for i in 0..12 {
                    if (self.pieces[i] & mask) != 0 {
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



    pub fn make_moves(&mut self, moves: &[String]) {

        if moves.len() % 2 == 0 {
            self.side_to_move = SideToMove::White;
        }
        else {
            self.side_to_move = SideToMove::Black;
        }
        for m in moves {
            let from_mask = square_to_mask(&m[0..2]);
            let to_mask = square_to_mask(&m[2..4]);
            let mut captured_a_piece = false;

            let move_mask = from_mask | to_mask;

            self.en_passant_mask = 0;

            if m.len() == 5 {  // Promoting
                for i in 0..12 {
                    if (self.pieces[i] & to_mask) != 0 {   // Capture
                        self.pieces[i] &= !to_mask;
                        captured_a_piece = true;
                        break;
                    }
                }
                if (self.pieces[P] & from_mask) != 0 { // White pawn promoting
                    self.pieces[P] ^= from_mask;
                    let promoting_type = match m.as_bytes()[4] {
                        b'n' => N,
                        b'b' => B,
                        b'r' => R,
                        b'q' => Q,
                        _ => unreachable!()
                    };
                    self.pieces[promoting_type] ^= to_mask;
                }
                else {    // Black pawn promoting
                    self.pieces[p] ^= from_mask;
                    let promoting_type = match m.as_bytes()[4] {
                        b'n' => n,
                        b'b' => b,
                        b'r' => r,
                        b'q' => q,
                        _ => unreachable!()
                    };
                    self.pieces[promoting_type] ^= to_mask;
                }
                self.halfmove_clock = 0;
                continue;
            }
            else {  // Normal move, castle, En Passant
                let is_pawn_moving = (self.pieces[P] & from_mask) != 0 || (self.pieces[p] & from_mask) != 0;

                for i in 0..12 {
                    if (self.pieces[i] & to_mask) != 0 {   // Usual capture
                        self.pieces[i] &= !to_mask;
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

                    self.pieces[p] &= !en_passant_capture_mask;
                    self.pieces[P] &= !en_passant_capture_mask;
                }

                for i in 0..12 { // The move
                    if (self.pieces[i] & from_mask) != 0 {
                        self.pieces[i] ^= move_mask;
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
                    "e1g1" => if (self.pieces[K] & to_mask) != 0 { self.make_simple_move("h1f1") },
                    "e1c1" => if (self.pieces[K] & to_mask) != 0 { self.make_simple_move("a1d1") },
                    "e8g8" => if (self.pieces[k] & to_mask) != 0 { self.make_simple_move("h8f8") },
                    "e8c8" => if (self.pieces[k] & to_mask) != 0 { self.make_simple_move("a8d8") },
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
            self.print_board();
        }
    }


    /// Assist function for simply moving one piece (without capturing)
    fn make_simple_move(&mut self, m: &str) {
        let from_mask = square_to_mask(&m[0..2]);
        let to_mask = square_to_mask(&m[2..4]);
        let move_mask = from_mask | to_mask;
        for i in 0..12 {
            if (self.pieces[i] & from_mask) != 0 {  // The move
                self.pieces[i] ^= move_mask;
                break;
            }
        }
    }
}

/// Makes a bitboard with a single bit from given coordinates
fn square_to_mask(square: &str) -> u64 {
    let bytes = square.as_bytes();
    let col = bytes[0] - b'a';
    let row = bytes[1] - b'1';
    let index = row * 8 + col;
    1u64 << index
}

