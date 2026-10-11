pub struct Bitboards {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_kings: u64,

    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_kings: u64,
}

impl Bitboards {
    pub fn new() -> Self {
        Self {
            white_pawns: 0x0000_0000_0000_ff00,
            white_knights:0x0000_0000_0000_0042,
            white_bishops:0x0000_0000_0000_0024,
            white_rooks:0x0000_0000_0000_0081,
            white_queens:0x0000_0000_0000_0008,
            white_kings:0x0000_0000_0000_0010,

            black_pawns: 0x00ff_0000_0000_0000,
            black_knights:0x4200_0000_0000_0000,
            black_bishops:0x2400_0000_0000_0000,
            black_rooks:0x8100_0000_0000_0000,
            black_queens:0x0800_0000_0000_0000,
            black_kings:0x1000_0000_0000_0000,
        }
    }
}


impl Bitboards {
    pub fn print_board(&self) {
        println!("  a b c d e f g h");

        for row in (0..8).rev() {
            print!("{} ", row + 1);
            for col in 0..8 {
                let index = row * 8 + col;
                let mask = 1u64 << index;


                let piece = if (self.white_pawns & mask) != 0 { 'P' }
                else if (self.white_knights & mask) != 0 { 'N' }
                else if (self.white_bishops & mask) != 0 { 'B' }
                else if (self.white_rooks & mask) != 0 { 'R' }
                else if (self.white_queens & mask) != 0 { 'Q' }
                else if (self.white_kings & mask) != 0 { 'K' }
                else if (self.black_pawns & mask) != 0 { 'p' }
                else if (self.black_knights & mask) != 0 { 'n' }
                else if (self.black_bishops & mask) != 0 { 'b' }
                else if (self.black_rooks & mask) != 0 { 'r' }
                else if (self.black_queens & mask) != 0 { 'q' }
                else if (self.black_kings & mask) != 0 { 'k' }
                else { '.' };

                print!("{} ", piece);
            }
            println!();
        }
        println!();
    }
}