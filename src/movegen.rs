use crate::board::{Bitboards, B, N, P, R, Q, K, p, n, b, r, q, k, SideToMove};

struct Move{
    from: u8,
    to: u8,
    flag: Flag,
}

#[repr(u8)]
enum Flag {
    Normal,
    Promote,
    Castle,
}

pub fn generate_moves(board: &Bitboards) -> Vec<Move> {
    let mut moves = Vec::new();
    let white_pieces = board.pieces[P] | board.pieces[N] | board.pieces[B] | board.pieces[R] | board.pieces[Q] | board.pieces[K];
    let black_pieces = board.pieces[p] | board.pieces[n] | board.pieces[b] | board.pieces[r] | board.pieces[q] | board.pieces[k];
    moves.append(&mut generate_knight_moves(board, white_pieces, black_pieces));

    moves
}

fn generate_knight_moves(board: &Bitboards, _white_pieces: u64, _black_pieces: u64) -> Vec<Move> {
    let mut knight_moves = Vec::new();


    if board.side_to_move == SideToMove::White{
        let mut white_knights = board.pieces[N];
        while white_knights != 0 {
            let _knight_square = white_knights.trailing_zeros() as u8;

            white_knights &= white_knights - 1;
        }
    }
    else {
        let mut black_knights = board.pieces[n];
        while black_knights != 0 {
            let _knight_square = black_knights.trailing_zeros() as u8;

            black_knights &= black_knights - 1;
        }
    }


    knight_moves
}


// I'm doing this in compile time :3
pub const KNIGHT_ATTACKS: [u64; 64] = init_knight_attacks();
const fn init_knight_attacks() -> [u64; 64] {
    let mut table: [u64; 64] = [0; 64];
    let mut i: usize = 0;
    while i < 64 {
        let mut attacks: u64 = 0;
        let row = i / 8;
        let col = i % 8;

        attacks = attacks | if row > 1 { // able to do vertical moves (down)
            let down_and_right = if col < 7 {
                1u64 << (i - 8 * 2 + 1)
            }
            else { 0 };

            let down_and_left = if col > 0 {
                1u64 << (i - 8 * 2 - 1)
            }
            else { 0 };
            down_and_right | down_and_left
        }
        else {
            0
        };
        attacks = attacks | if row < 6 { // able to do vertical moves (up)
            let up_and_right = if col < 7 {
                1u64 << (i + 8 * 2 + 1)
            }
            else { 0 };

            let up_and_left = if col > 0 {
                1u64 << (i + 8 * 2 - 1)
            }
            else { 0 };
            up_and_right | up_and_left

        }
        else { 0 };

        attacks = attacks | if col > 1 { // able to do horizontal moves (left)
            let left_and_up = if row < 7 {
                1u64 << (i - 2 + 8)
            }
            else { 0 };
            let left_and_down = if row > 0 {
                1u64 << (i - 2 - 8)
            }
            else { 0 };
            left_and_up | left_and_down
        }
        else { 0 };
        attacks = attacks | if col < 6 { // able to do horizontal moves (right)
            let right_and_up = if row < 7 {
                1u64 << (i + 2 + 8)
            }
            else { 0 };
            let right_and_down = if row > 0 {
                1u64 << (i + 2 - 8)
            }
            else { 0 };
            right_and_up | right_and_down
        }
        else { 0 };


        table[i] = attacks;
        i += 1;
    }
    table
}

// +- 8*2 +- 1    for vertical attacks
// +- 2 +- 8      for horizontal attacks