use crate::board::{Bitboards, B, N, P, R, Q, K, p, n, b, r, q, k, SideToMove};
use crate::movespreloader::{KNIGHT_ATTACKS, KING_ATTACKS};

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

fn generate_knight_moves(board: &Bitboards, white_pieces: u64, black_pieces: u64) -> Vec<Move> {
    let mut knight_moves = Vec::new();


    if board.side_to_move == SideToMove::White{
        let mut white_knights = board.pieces[N];
        while white_knights != 0 {
            let knight_square = white_knights.trailing_zeros() as u8;
            let mut legal_moves_mask = KNIGHT_ATTACKS[knight_square as usize] & !white_pieces;  // This ignores getting the knight into a check
            while legal_moves_mask != 0 {
                let to_index = legal_moves_mask.trailing_zeros() as u8;
                knight_moves.push(Move {
                    from: knight_square,
                    to: to_index,
                    flag: Flag::Normal
                });
                legal_moves_mask &= legal_moves_mask - 1;
            }
            white_knights &= white_knights - 1;
        }
    }
    else {
        let mut black_knights = board.pieces[n];
        while black_knights != 0 {
            let knight_square = black_knights.trailing_zeros() as u8;
            let mut legal_moves_mask = KNIGHT_ATTACKS[knight_square as usize] & !black_pieces;  // This ignores getting the knight into a check
            while legal_moves_mask != 0 {
                let to_index = legal_moves_mask.trailing_zeros() as u8;
                knight_moves.push(Move {
                    from: knight_square,
                    to: to_index,
                    flag: Flag::Normal
                });
                legal_moves_mask &= legal_moves_mask - 1;
            }
            black_knights &= black_knights - 1;
        }
    }

    knight_moves
}



