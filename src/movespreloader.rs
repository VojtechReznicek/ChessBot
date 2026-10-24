// I'm doing this in compile time :3
pub const KNIGHT_ATTACKS: [u64; 64] = init_knight_attacks();
pub const KING_ATTACKS: [u64; 64] = init_king_attacks();
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
    while i < 64{
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