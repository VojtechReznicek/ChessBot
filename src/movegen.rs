use crate::board::{Bitboards, B, N, P, R, Q, K, p, n, b, r, q, k, SideToMove,};
use crate::movespreloader::{KNIGHT_ATTACKS, KING_ATTACKS};
pub const RANK_2: u64 = 0x000000000000FF00;
pub const RANK_7: u64 = 0x00FF000000000000;
pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_H: u64 = 0x8080808080808080;


pub struct Move{
    pub(crate) from: u8,
    pub(crate) to: u8,
    pub(crate) flag: Flag,
}

#[repr(u8)]
pub enum Flag {
    Normal,
    PromoteQueen,
    PromoteRook,
    PromoteBishop,
    PromoteKnight,
    PromoteQueenCapture,
    PromoteRookCapture,
    PromoteBishopCapture,
    PromoteKnightCapture,
    Castle,
    DoublePush,
    Capture,
    EnPassant,
}

pub fn generate_moves(board: &Bitboards) -> Vec<Move> {
    let mut moves = Vec::new();
    let white_pieces = board.pieces[P] | board.pieces[N] | board.pieces[B] | board.pieces[R] | board.pieces[Q] | board.pieces[K];
    let black_pieces = board.pieces[p] | board.pieces[n] | board.pieces[b] | board.pieces[r] | board.pieces[q] | board.pieces[k];
    let occupancy = white_pieces | black_pieces;
    moves.append(&mut generate_knight_moves(board, white_pieces, black_pieces));
    moves.append(&mut generate_king_moves(board, white_pieces, black_pieces, occupancy));
    moves.append(&mut generate_pawn_moves(board, black_pieces, white_pieces, occupancy));
    moves
}

fn generate_knight_moves(board: &Bitboards, white_pieces: u64, black_pieces: u64) -> Vec<Move> {
    let mut knight_moves = Vec::new();


    if board.side_to_move == SideToMove::White{
        let mut white_knights = board.pieces[N];
        while white_knights != 0 {
            let knight_square = white_knights.trailing_zeros() as u8;
            let mut legal_moves_mask = KNIGHT_ATTACKS[knight_square as usize] & !white_pieces;
            let mut legal_captures_mask = legal_moves_mask & black_pieces;
            legal_moves_mask &= !black_pieces;
            while legal_moves_mask != 0 {  // Without capture
                let to_index = legal_moves_mask.trailing_zeros() as u8;
                knight_moves.push(Move {
                    from: knight_square,
                    to: to_index,
                    flag: Flag::Normal
                });
                legal_moves_mask &= legal_moves_mask - 1;
            }
            while legal_captures_mask != 0 {
                let to_index = legal_captures_mask.trailing_zeros() as u8;
                knight_moves.push(Move{
                    from: knight_square,
                    to: to_index,
                    flag: Flag::Capture
                })
            }

            white_knights &= white_knights - 1;
        }
    }
    else {
        let mut black_knights = board.pieces[n];
        while black_knights != 0 {
            let knight_square = black_knights.trailing_zeros() as u8;
            let mut legal_moves_mask = KNIGHT_ATTACKS[knight_square as usize] & !black_pieces;
            let mut legal_captures_mask = legal_moves_mask & white_pieces;
            legal_moves_mask &= !white_pieces;
            while legal_moves_mask != 0 {
                let to_index = legal_moves_mask.trailing_zeros() as u8;
                knight_moves.push(Move {
                    from: knight_square,
                    to: to_index,
                    flag: Flag::Normal
                });
                legal_moves_mask &= legal_moves_mask - 1;
            }
            while legal_captures_mask != 0 {
                let to_index = legal_captures_mask.trailing_zeros() as u8;
                knight_moves.push(Move{
                    from: knight_square,
                    to: to_index,
                    flag: Flag::Capture
                })
            }

            black_knights &= black_knights - 1;
        }
    }

    knight_moves
}



fn generate_king_moves(board: &Bitboards, white_pieces: u64, black_pieces: u64, occupancy: u64) -> Vec<Move> {
    let mut king_moves = Vec::new();
    if board.side_to_move == SideToMove::White {
        let mut white_kings = board.pieces[K];
        while white_kings != 0 {
            let king_square = white_kings.trailing_zeros() as u8;
            let mut legal_moves_mask = KING_ATTACKS[king_square as usize] & !white_pieces;  // This ignores getting the king into a check
            let mut legal_captures_mask = legal_moves_mask & black_pieces;
            legal_moves_mask &= !black_pieces;
            while legal_moves_mask != 0 {
                let to_index = legal_moves_mask.trailing_zeros() as u8;
                king_moves.push(Move { // Normal king move
                    from: king_square,
                    to: to_index,
                    flag: Flag::Normal
                });
                legal_moves_mask &= legal_moves_mask - 1;
            }
            while legal_captures_mask != 0 {
                let to_index = legal_captures_mask.trailing_zeros() as u8;
                king_moves.push(Move{
                    from: king_square,
                    to: to_index,
                    flag: Flag::Capture
                })
            }

            if king_square == 4 { // Castle
                if board.castling_rights.white_kingside {
                    if (occupancy & ((1u64 << 5) | (1u64 << 6))) == 0 {  // White kingside castle move
                        king_moves.push(Move{
                            from: 4,
                            to: 6,
                            flag: Flag::Castle
                        })
                    }
                }
                if board.castling_rights.white_queenside {
                    if (occupancy & ((1u64 << 1) | (1u64 << 2) | (1u64 << 3))) == 0 { // White queenside castle move
                        king_moves.push(Move{
                            from: 4,
                            to: 2,
                            flag: Flag::Castle
                        })
                    }
                }
            }


            white_kings &= white_kings - 1;
        }
    }
    else {
        let mut black_kings = board.pieces[k];
        while black_kings != 0 {
            let king_square = black_kings.trailing_zeros() as u8;
            let mut legal_moves_mask = KING_ATTACKS[king_square as usize] & !black_pieces;
            let mut legal_captures_mask = legal_moves_mask & white_pieces;
            legal_moves_mask &= !white_pieces;
            while legal_moves_mask != 0 { // Normal king move
                let to_index = legal_moves_mask.trailing_zeros() as u8;
                king_moves.push(Move {
                    from: king_square,
                    to: to_index,
                    flag: Flag::Normal
                });
                legal_moves_mask &= legal_moves_mask - 1;
            }
            while legal_captures_mask != 0 {
                let to_index = legal_captures_mask.trailing_zeros() as u8;
                king_moves.push(Move{
                    from: king_square,
                    to: to_index,
                    flag: Flag::Capture
                })
            }

            if king_square == 60 { // Castle
                if board.castling_rights.black_kingside {
                    if (occupancy & ((1u64 << 61) | (1u64 << 62))) == 0 {  // Black kingside castle move
                        king_moves.push(Move{
                            from: 60,
                            to: 62,
                            flag: Flag::Castle
                        })
                    }
                }
                if board.castling_rights.black_queenside {
                    if (occupancy & ((1u64 << 57) | (1u64 << 58) | (1u64 << 59))) == 0 { // Black queenside castle move
                        king_moves.push(Move{
                            from: 60,
                            to: 58,
                            flag: Flag::Castle
                        })
                    }
                }
            }
            black_kings &= black_kings - 1;
        }
    }

    king_moves
}

fn generate_pawn_moves(board: &Bitboards, black_pieces: u64, white_pieces: u64, occupancy: u64) -> Vec<Move> {
    let mut pawn_moves = Vec::new();
    if board.side_to_move == SideToMove::White{
        let white_pawns = board.pieces[P];
        let mut basic_moves_mask = white_pawns << 8 & !occupancy;
        let mut twostep_mask = (((white_pawns & RANK_2) << 8) & !occupancy ) << 8 & !occupancy;
        while basic_moves_mask != 0 {
            let to_index = basic_moves_mask.trailing_zeros() as u8;
            if to_index > 55 {   // Promoting
                pawn_moves.push(Move { from: to_index - 8, to: to_index, flag: Flag::PromoteQueen });
                pawn_moves.push(Move { from: to_index - 8, to: to_index, flag: Flag::PromoteRook });
                pawn_moves.push(Move { from: to_index - 8, to: to_index, flag: Flag::PromoteBishop });
                pawn_moves.push(Move { from: to_index - 8, to: to_index, flag: Flag::PromoteKnight });
            }
            else {
                pawn_moves.push(Move {
                    from: to_index - 8,
                    to: to_index,
                    flag: Flag::Normal,
                });
            }

            basic_moves_mask &= basic_moves_mask - 1;
        }
        while twostep_mask != 0 {
            let to_index = twostep_mask.trailing_zeros() as u8;
            pawn_moves.push(Move {
                from: to_index - 16,
                to: to_index,
                flag: Flag::DoublePush,
            });
            twostep_mask &= twostep_mask - 1;
        }

        let mut left_attacks_mask = ((white_pawns & !FILE_A) << 7) & black_pieces;
        let mut right_attacks_mask = ((white_pawns & !FILE_H) << 9) & black_pieces;
        while left_attacks_mask != 0 { // Capture
            let to_index = left_attacks_mask.trailing_zeros() as u8;
            if to_index > 55 {   // Promoting
                pawn_moves.push(Move { from: to_index - 7, to: to_index, flag: Flag::PromoteQueenCapture });
                pawn_moves.push(Move { from: to_index - 7, to: to_index, flag: Flag::PromoteRookCapture });
                pawn_moves.push(Move { from: to_index - 7, to: to_index, flag: Flag::PromoteBishopCapture });
                pawn_moves.push(Move { from: to_index - 7, to: to_index, flag: Flag::PromoteKnightCapture });
            }
            else {
                pawn_moves.push(Move {
                    from: to_index - 7,
                    to: to_index,
                    flag: Flag::Capture,
                });
            }

            left_attacks_mask &= left_attacks_mask - 1;

        }
        while right_attacks_mask != 0 { // Capture
            let to_index = right_attacks_mask.trailing_zeros() as u8;
            if to_index > 55 {   // Promoting
                pawn_moves.push(Move { from: to_index - 9, to: to_index, flag: Flag::PromoteQueenCapture });
                pawn_moves.push(Move { from: to_index - 9, to: to_index, flag: Flag::PromoteRookCapture });
                pawn_moves.push(Move { from: to_index - 9, to: to_index, flag: Flag::PromoteBishopCapture });
                pawn_moves.push(Move { from: to_index - 9, to: to_index, flag: Flag::PromoteKnightCapture });
            }
            else {
                pawn_moves.push(Move {
                    from: to_index - 9,
                    to: to_index,
                    flag: Flag::Capture,
                });
            }
            right_attacks_mask &= right_attacks_mask - 1;
        }


        if board.en_passant_mask >> 7 & (white_pawns & !FILE_A) != 0 {
           let to_index = board.en_passant_mask.trailing_zeros() as u8;
            pawn_moves.push(Move{
                from: to_index - 7,
                to: to_index,
                flag: Flag::EnPassant,
            })
        }
        if board.en_passant_mask >> 9 & (white_pawns & !FILE_H) != 0 {
            let to_index = board.en_passant_mask.trailing_zeros() as u8;
            pawn_moves.push(Move{
                from: to_index - 9,
                to: to_index,
                flag: Flag::EnPassant,
            })
        }

    }
    else { // Black
        let black_pawns = board.pieces[p];
        let mut basic_moves_mask = black_pawns >> 8 & !occupancy;
        let mut twostep_mask = (((black_pawns & RANK_7) >> 8) & !occupancy ) >> 8 & !occupancy;

        while basic_moves_mask != 0 {
            let to_index = basic_moves_mask.trailing_zeros() as u8;
            if to_index < 8 {   // Promoting
                pawn_moves.push(Move { from: to_index + 8, to: to_index, flag: Flag::PromoteQueen });
                pawn_moves.push(Move { from: to_index + 8, to: to_index, flag: Flag::PromoteRook });
                pawn_moves.push(Move { from: to_index + 8, to: to_index, flag: Flag::PromoteBishop });
                pawn_moves.push(Move { from: to_index + 8, to: to_index, flag: Flag::PromoteKnight });
            }
            else {
                pawn_moves.push(Move {
                    from: to_index + 8,
                    to: to_index,
                    flag: Flag::Normal,
                });
            }
            basic_moves_mask &= basic_moves_mask - 1;
        }

        while twostep_mask != 0 {
            let to_index = twostep_mask.trailing_zeros() as u8;
            pawn_moves.push(Move {
                from: to_index + 16,
                to: to_index,
                flag: Flag::DoublePush,
            });
            twostep_mask &= twostep_mask - 1;
        }

        let mut left_attacks_mask = ((black_pawns & !FILE_H) >> 7) & white_pieces;
        let mut right_attacks_mask = ((black_pawns & !FILE_A) >> 9) & white_pieces;

        while left_attacks_mask != 0 { // Capture
            let to_index = left_attacks_mask.trailing_zeros() as u8;
            if to_index < 8 {   // Promoting
                pawn_moves.push(Move { from: to_index + 7, to: to_index, flag: Flag::PromoteQueenCapture });
                pawn_moves.push(Move { from: to_index + 7, to: to_index, flag: Flag::PromoteRookCapture });
                pawn_moves.push(Move { from: to_index + 7, to: to_index, flag: Flag::PromoteBishopCapture });
                pawn_moves.push(Move { from: to_index + 7, to: to_index, flag: Flag::PromoteKnightCapture });
            }
            else {
                pawn_moves.push(Move {
                    from: to_index + 7,
                    to: to_index,
                    flag: Flag::Capture,
                });
            }
            left_attacks_mask &= left_attacks_mask - 1;
        }

        while right_attacks_mask != 0 { // Capture
            let to_index = right_attacks_mask.trailing_zeros() as u8;
            if to_index < 8 {   // Promoting
                pawn_moves.push(Move { from: to_index + 9, to: to_index, flag: Flag::PromoteQueenCapture });
                pawn_moves.push(Move { from: to_index + 9, to: to_index, flag: Flag::PromoteRookCapture });
                pawn_moves.push(Move { from: to_index + 9, to: to_index, flag: Flag::PromoteBishopCapture });
                pawn_moves.push(Move { from: to_index + 9, to: to_index, flag: Flag::PromoteKnightCapture });
            }
            else {
                pawn_moves.push(Move {
                    from: to_index + 9,
                    to: to_index,
                    flag: Flag::Capture,
                });
            }
            right_attacks_mask &= right_attacks_mask - 1;
        }
        if board.en_passant_mask << 7 & (black_pawns & !FILE_H) != 0 {
            let to_index = board.en_passant_mask.trailing_zeros() as u8;
            pawn_moves.push(Move{
                from: to_index + 7,
                to: to_index,
                flag: Flag::EnPassant,
            })
        }
        if board.en_passant_mask << 9 & (black_pawns & !FILE_A) != 0 {
            let to_index = board.en_passant_mask.trailing_zeros() as u8;
            pawn_moves.push(Move{
                from: to_index + 9,
                to: to_index,
                flag: Flag::EnPassant,
            })
        }

    }

    pawn_moves
}

