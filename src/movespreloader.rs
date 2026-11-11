//! This module preloads tables for easier move generation (in movegen.rs).
//! Preloaded table is an array of 64 u64 numbers (masks). So each table uses exactly 512 bytes in the memory.
//! These tables are made to speed up the move generating process.
//! Here they are being made in the compile-time in const functions and saved as const variables to not slow up the process in the runtime.

/// Preloaded table of all the possible knight attacks
pub const KNIGHT_ATTACKS: [u64; 64] = init_knight_attacks();

/// Preloaded table of all the possible king attacks
pub const KING_ATTACKS: [u64; 64] = init_king_attacks();

/// Preloaded table of all the possible north ray attacks
pub const NORTH_ATTACKS: [u64; 64] = init_sliding_attacks(1, 0);

/// Preloaded table of all the possible south ray attacks
pub const SOUTH_ATTACKS: [u64; 64] = init_sliding_attacks(-1, 0);

/// Preloaded table of all the possible west ray attacks
pub const WEST_ATTACKS: [u64; 64] = init_sliding_attacks(0, -1);

/// Preloaded table of all the possible east ray attacks
pub const EAST_ATTACKS: [u64; 64] = init_sliding_attacks(0, 1);

/// Preloaded table of all the possible north-west ray attacks
pub const NORTH_WEST_ATTACKS: [u64; 64] = init_sliding_attacks(1, -1);

/// Preloaded table of all the possible north-east ray attacks
pub const NORTH_EAST_ATTACKS: [u64; 64] = init_sliding_attacks(1, 1);

/// Preloaded table of all the possible south-west ray attacks
pub const SOUTH_WEST_ATTACKS: [u64; 64] = init_sliding_attacks(-1, -1);

/// Preloaded table of all the possible south-east ray attacks
pub const SOUTH_EAST_ATTACKS: [u64; 64] = init_sliding_attacks(-1, 1);


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


const fn init_king_attacks() -> [u64; 64] {
    let mut table: [u64; 64] = [0; 64];
    let mut i: usize = 0;
    while i < 64 {
        let mut attacks: u64 = 0;
        let row = i / 8;
        let col = i % 8;

        attacks = attacks | if row > 0 { // able to do vertical moves (down)
            let down = 1u64 << (i - 8);
            let down_and_right = if col < 7 {
                1u64 << (i - 8 + 1)
            }
            else { 0 };
            let down_and_left = if col > 0 {
                1u64 << (i - 8 - 1)
            }
            else { 0 };
            down_and_right | down_and_left | down
        }
        else { 0 };

        attacks = attacks | if row < 7 { // able to do vertical moves (up)
            let up = 1u64 << (i + 8);
            let up_and_right = if col < 7 {
                1u64 << (i + 8 + 1)
            }
            else { 0 };
            let up_and_left = if col > 0 {
                1u64 << (i + 8 - 1)
            }
            else { 0 };
            up_and_right | up_and_left | up
        }
        else { 0 };


        // able to do horizontal moves
        let right = if col < 7 {
            1u64 << (i + 1)
        } else { 0 };

        let left = if col > 0 {
            1u64 << (i - 1)
        } else { 0 };

        attacks = attacks | right | left;

        table[i] = attacks;
        i += 1;
    }


    table
}
// +- 8 (up and down)
// +- 1 (right and left)
// +- 8 +- 1 (diagonals)


const fn init_sliding_attacks(row_step: i8, col_step: i8) ->  [u64; 64] {
    let mut table: [u64; 64] = [0; 64];
    let mut i: usize = 0;
    while i < 64 {
        let mut attacks: u64 = 0;
        let mut row = (i / 8) as i8 + row_step;
        let mut col = (i % 8) as i8 + col_step;
        while row >= 0 && row < 8 && col >= 0 && col < 8 {
            attacks |= 1u64 << (row * 8 + col);
            row += row_step;
            col += col_step;
        }
        table[i] = attacks;
        i += 1;
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaping_moves_symetry() {
        for i in 0..64 {
            let mut moves = KNIGHT_ATTACKS[i];
            while moves != 0 {
                let j = moves.trailing_zeros() as usize;
                let back = KNIGHT_ATTACKS[j];
                let res = back & (1u64 << i) != 0;
                assert_eq!(res, true);
                moves &= moves - 1;
            }
        }

        for i in 0..64 {
            let mut moves = KING_ATTACKS[i];
            while moves != 0 {
                let j = moves.trailing_zeros() as usize;
                let back = KING_ATTACKS[j];
                let res = back & (1u64 << i) != 0;
                assert_eq!(res, true);
                moves &= moves - 1;
            }
        }

    }

    #[test]
    fn test_sliding_moves_symmetry() {
        let opposite_pairs = [
            (NORTH_ATTACKS, SOUTH_ATTACKS),
            (EAST_ATTACKS, WEST_ATTACKS),
            (NORTH_WEST_ATTACKS, SOUTH_EAST_ATTACKS),
            (NORTH_EAST_ATTACKS, SOUTH_WEST_ATTACKS),
        ];

        for (forward, backward) in opposite_pairs {
            for i in 0..64 {
                let mut moves = forward[i];
                while moves != 0 {
                    let j = moves.trailing_zeros() as usize;
                    let back = backward[j];
                    let res = (back & (1u64 << i)) != 0;
                    assert!(res, "Ray mismatch between square {} and {}", i, j);
                    moves &= moves - 1;
                }
            }
        }
    }

}