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
    pub side_to_move: SideToMove
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SideToMove {
    Black,
    White,
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

        Self { pieces, side_to_move }
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

            let move_mask = from_mask | to_mask;

            if m.len() == 5 { // Promoting
                for i in 0..12 {
                    if (self.pieces[i] & to_mask) != 0 {   // Capture
                        self.pieces[i] &= !to_mask;
                        break;
                    }
                }
                if (self.pieces[P] & from_mask) != 0 { // White promoting
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
                else {    // Black promoting
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
                continue;
            }
            else {  // Normal move
                for i in 0..12 {
                    if (self.pieces[i] & to_mask) != 0 {   // Capture
                        self.pieces[i] &= !to_mask;
                        break;
                    }
                }
                for i in 0..12 {
                    if (self.pieces[i] & from_mask) != 0 {  // The move
                        self.pieces[i] ^= move_mask;
                        break;
                    }
                }
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
