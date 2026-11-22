//! This module generates all the possible pseudo-legal moves from a given position for the player who moves at the moment.
//! It uses preloaded tables from movespreloader.rs to run faster.
//! Valid pseudo-legal moves are saved as a Vec<Move>
//! Move is a simplified structure for saving moves as indexes (not bitboards) and a flag (that contains data about moves that require any special handling)

use crate::board::{GameState, B, N, P, R, Q, K, p, n, b, r, q, k, SideToMove,};
use crate::movespreloader::{KNIGHT_ATTACKS, KING_ATTACKS, NORTH_ATTACKS, SOUTH_ATTACKS, EAST_ATTACKS, WEST_ATTACKS, NORTH_EAST_ATTACKS, NORTH_WEST_ATTACKS, SOUTH_EAST_ATTACKS, SOUTH_WEST_ATTACKS};
pub const RANK_2: u64 = 0x000000000000FF00;
pub const RANK_7: u64 = 0x00FF000000000000;
pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_H: u64 = 0x8080808080808080;

#[derive(Clone, Copy)]
pub struct Move{
    pub(crate) from: u8,
    pub(crate) to: u8,
    pub(crate) flag: Flag,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone)]
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
    CastleQueenside,
    CastleKingside,
    DoublePush,
    Capture,
    EnPassant,
}

pub fn generate_moves(board: &GameState) -> Vec<Move> {
    let mut moves = Vec::new();
    let white_pieces = board.bitboards[P] | board.bitboards[N] | board.bitboards[B] | board.bitboards[R] | board.bitboards[Q] | board.bitboards[K];
    let black_pieces = board.bitboards[p] | board.bitboards[n] | board.bitboards[b] | board.bitboards[r] | board.bitboards[q] | board.bitboards[k];
    let occupancy = white_pieces | black_pieces;
    moves.append(&mut generate_knight_moves(board, white_pieces, black_pieces));
    moves.append(&mut generate_king_moves(board, white_pieces, black_pieces, occupancy));
    moves.append(&mut generate_pawn_moves(board,white_pieces, black_pieces, occupancy));
    moves.append(&mut generate_rook_moves(board, white_pieces, black_pieces, occupancy));
    moves.append(&mut generate_bishop_moves(board, white_pieces, black_pieces, occupancy));

    moves
}

fn generate_knight_moves(board: &GameState, white_pieces: u64, black_pieces: u64) -> Vec<Move> {
    let mut knight_moves = Vec::new();
    let (mut knights, friendly_pieces, enemy_pieces) = if board.side_to_move == SideToMove::White {
        (board.bitboards[N], white_pieces, black_pieces)
    } else {
        (board.bitboards[n], black_pieces, white_pieces)
    };

    while knights != 0 {
        let knight_square = knights.trailing_zeros() as u8;
        let mut legal_moves_mask = KNIGHT_ATTACKS[knight_square as usize] & !friendly_pieces;
        let mut legal_captures_mask = legal_moves_mask & enemy_pieces;
        legal_moves_mask &= !enemy_pieces;
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
            });
            legal_captures_mask &= legal_captures_mask - 1;
        }

        knights &= knights - 1;
    }
    knight_moves
}




fn generate_king_moves(board: &GameState, white_pieces: u64, black_pieces: u64, occupancy: u64) -> Vec<Move> {
    let mut king_moves = Vec::new();

    let (mut kings, friendly_pieces, enemy_pieces) = if board.side_to_move == SideToMove::White {
        (board.bitboards[K], white_pieces, black_pieces)
    } else {
        (board.bitboards[k], black_pieces, white_pieces)
    };


    while kings != 0 {
        let king_square = kings.trailing_zeros() as u8;
        let mut legal_moves_mask = KING_ATTACKS[king_square as usize] & !friendly_pieces;  // This ignores getting the king into a check
        let mut legal_captures_mask = legal_moves_mask & enemy_pieces;
        legal_moves_mask &= !enemy_pieces;
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
            });
            legal_captures_mask &= legal_captures_mask - 1;
        }

        king_moves.append(check_for_castle(board, king_square, occupancy).as_mut());

        kings &= kings - 1;
    }

    king_moves
}

fn check_for_castle(board: &GameState, king_square: u8, occupancy: u64) -> Vec<Move> {
    let mut result: Vec<Move> = Vec::new();
    if king_square == 4 { // Castle

        if !board.is_attacked(king_square, false) {
            if board.castling_rights.white_kingside && !board.is_attacked(5, false) {
                if (occupancy & ((1u64 << 5) | (1u64 << 6))) == 0 {  // White kingside castle move
                    result.push(Move{
                        from: 4,
                        to: 6,
                        flag: Flag::CastleKingside
                    })
                }
            }
            if board.castling_rights.white_queenside && !board.is_attacked(3, false)  {
                if (occupancy & ((1u64 << 1) | (1u64 << 2) | (1u64 << 3))) == 0 { // White queenside castle move
                    result.push(Move{
                        from: 4,
                        to: 2,
                        flag: Flag::CastleQueenside
                    })
                }
            }
        }
    }

    else if king_square == 60 { // Castle
        if !board.is_attacked(king_square, true) {
            if board.castling_rights.black_kingside && !board.is_attacked(61, true) {
                if (occupancy & ((1u64 << 61) | (1u64 << 62))) == 0 {  // Black kingside castle move
                    result.push(Move{
                        from: 60,
                        to: 62,
                        flag: Flag::CastleKingside
                    })
                }
            }
            if board.castling_rights.black_queenside && !board.is_attacked(59, true) {
                if (occupancy & ((1u64 << 57) | (1u64 << 58) | (1u64 << 59))) == 0 { // Black queenside castle move
                    result.push(Move{
                        from: 60,
                        to: 58,
                        flag: Flag::CastleQueenside
                    });
                }
            }
        }

    }
    result
}

fn generate_pawn_moves(board: &GameState, white_pieces: u64, black_pieces: u64, occupancy: u64) -> Vec<Move> {
    let mut pawn_moves = Vec::new();
    if board.side_to_move == SideToMove::White{
        let white_pawns = board.bitboards[P];
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
        let black_pawns = board.bitboards[p];
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


fn generate_rook_moves(board: &GameState, white_pieces: u64, black_pieces: u64, occupancy: u64) -> Vec<Move> {
    let mut rook_moves = Vec::new();

    let (mut rooks, enemy_pieces) = if board.side_to_move == SideToMove::White {
        (board.bitboards[R] | board.bitboards[Q], black_pieces)
    } else {
        (board.bitboards[r] | board.bitboards[q], white_pieces)
    };

    while rooks != 0 {
        let rook_index = rooks.trailing_zeros() as u8;
        let north_obstacle = (NORTH_ATTACKS[rook_index as usize] & occupancy).trailing_zeros() as usize;
        let north_ray = if north_obstacle != 64 {
             NORTH_ATTACKS[rook_index as usize] ^ NORTH_ATTACKS[north_obstacle]
        }
        else {
            NORTH_ATTACKS[rook_index as usize]
        };

        let south_zeros = (SOUTH_ATTACKS[rook_index as usize] & occupancy).leading_zeros() as usize;
        let south_ray = if south_zeros != 64 {
            SOUTH_ATTACKS[rook_index as usize] ^ SOUTH_ATTACKS[63 - south_zeros]
        } else {
            SOUTH_ATTACKS[rook_index as usize]
        };

        let east_obstacle = (EAST_ATTACKS[rook_index as usize] & occupancy).trailing_zeros() as usize;
        let east_ray = if east_obstacle != 64 {
            EAST_ATTACKS[rook_index as usize] ^ EAST_ATTACKS[east_obstacle]
        }
        else {
            EAST_ATTACKS[rook_index as usize]
        };

        let west_zeros = (WEST_ATTACKS[rook_index as usize] & occupancy).leading_zeros() as usize;
        let west_ray = if west_zeros != 64 {
            WEST_ATTACKS[rook_index as usize] ^ WEST_ATTACKS[63 - west_zeros]
        } else {
            WEST_ATTACKS[rook_index as usize]
        };

        let all_rays = north_ray | west_ray | south_ray | east_ray;
        rook_moves.append(&mut rays_to_moves(rook_index, all_rays, occupancy, enemy_pieces));

        rooks &= rooks - 1;
    }


    rook_moves
}

fn generate_bishop_moves(board: &GameState, white_pieces: u64, black_pieces: u64, occupancy: u64) -> Vec<Move> {
    let mut moves = Vec::new();

    let (mut bishops, enemy_pieces) = if board.side_to_move == SideToMove::White {
        (board.bitboards[B] | board.bitboards[Q], black_pieces)
    } else {
        (board.bitboards[b] | board.bitboards[q], white_pieces)
    };

    while bishops != 0 {
        let bishop_index = bishops.trailing_zeros() as usize;

        let northeast_obstacle = (NORTH_EAST_ATTACKS[bishop_index] & occupancy).trailing_zeros() as usize;
        let northeast_ray = if northeast_obstacle != 64 {
            NORTH_EAST_ATTACKS[bishop_index] ^ NORTH_EAST_ATTACKS[northeast_obstacle]
        }
        else { NORTH_EAST_ATTACKS[bishop_index] };

        let northwest_obstacle = (NORTH_WEST_ATTACKS[bishop_index] & occupancy).trailing_zeros() as usize;
        let northwest_ray = if northwest_obstacle != 64 {
            NORTH_WEST_ATTACKS[bishop_index] ^ NORTH_WEST_ATTACKS[northwest_obstacle]
        }
        else { NORTH_WEST_ATTACKS[bishop_index] };

        let southeast_obstacle = (SOUTH_EAST_ATTACKS[bishop_index] & occupancy).leading_zeros() as usize;
        let southeast_ray = if southeast_obstacle != 64 {
            SOUTH_EAST_ATTACKS[bishop_index] ^ SOUTH_EAST_ATTACKS[63 - southeast_obstacle]
        }
        else { SOUTH_EAST_ATTACKS[bishop_index] };

        let southwest_obstacle = (SOUTH_WEST_ATTACKS[bishop_index] & occupancy).leading_zeros() as usize;
        let southwest_ray = if southwest_obstacle != 64 {
            SOUTH_WEST_ATTACKS[bishop_index] ^ SOUTH_WEST_ATTACKS[63 - southwest_obstacle]
        }
        else { SOUTH_WEST_ATTACKS[bishop_index] };

        let all_rays = northeast_ray | northwest_ray | southeast_ray | southwest_ray;
        moves.append(&mut rays_to_moves(bishop_index as u8, all_rays, occupancy, enemy_pieces));

        bishops &= bishops - 1;
    }

    moves
}

fn rays_to_moves(my_index: u8, all_rays: u64, occupancy: u64, enemy_pieces: u64) -> Vec<Move> {
    let mut moves = Vec::new();
    let mut normal_rays = all_rays & !occupancy;
    let mut captures = all_rays & enemy_pieces;
    while normal_rays != 0 {
        moves.push(Move {
            from: my_index,
            to: normal_rays.trailing_zeros() as u8,
            flag: Flag::Normal,
        });
        normal_rays &= normal_rays - 1;
    }
    while captures != 0 {
        moves.push(Move{
            from: my_index,
            to: captures.trailing_zeros() as u8,
            flag: Flag::Capture,
        });
        captures &= captures - 1;
    }
    moves
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startpos_move_count() {
        let mut board = GameState::new();

        // White moves (16 pawn pushes + 4 knight jumps)
        let white_moves = generate_moves(&board);
        assert_eq!(white_moves.len(), 20, "White must have exactly 20 moves in startpos");

        // Black moves
        board.side_to_move = SideToMove::Black;
        let black_moves = generate_moves(&board);
        assert_eq!(black_moves.len(), 20, "Black must have exactly 20 moves in startpos");
    }

    fn perft(board: &mut GameState, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }

        let moves = generate_moves(board);
        let mut nodes = 0;
        let is_white = board.side_to_move == SideToMove::White;

        for mov in moves {
            let undo = board.make_move(mov);

            let king_index = if is_white {
                board.bitboards[K].trailing_zeros() as u8
            } else {
                board.bitboards[k].trailing_zeros() as u8
            };

            if !board.is_attacked(king_index, !is_white) {
                nodes += perft(board, depth - 1);
            }

            board.unmake_move(mov, undo);
        }

        nodes
    }

    #[test]
    fn test_perft_startpos() {
        let mut board = GameState::new();
        assert_eq!(perft(&mut board, 1), 20);
        assert_eq!(perft(&mut board, 2), 400);
        assert_eq!(perft(&mut board, 3), 8_902);
        assert_eq!(perft(&mut board, 4), 197_281);
    }

    #[test]
    fn test_perft_kiwipete() {
        let mut board = GameState::fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
        );
        assert_eq!(perft(&mut board, 1), 48);
        assert_eq!(perft(&mut board, 2), 2_039);
        assert_eq!(perft(&mut board, 3), 97_862);
    }

    #[test]
    fn test_perft_endgame_ep_and_pins() {
        let mut board = GameState::fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
        assert_eq!(perft(&mut board, 1), 14);
        assert_eq!(perft(&mut board, 2), 191);
        assert_eq!(perft(&mut board, 3), 2_812);
        assert_eq!(perft(&mut board, 4), 43_238);
    }

    #[test]
    fn test_perft_promotions_and_castling() {
        let mut board = GameState::fen(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"
        );
        assert_eq!(perft(&mut board, 1), 44);
        assert_eq!(perft(&mut board, 2), 1_486);
        assert_eq!(perft(&mut board, 3), 62_379);
    }
}