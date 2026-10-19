mod board;
mod uci;

use crate::board::Bitboards;

fn main() {
    let mut board = Bitboards::new();
    board.print_board();

    loop {
        let input = get_input();

        if input.is_empty() {
            continue;
        }
        uci::parse_uci(&input, &mut board);
    }
}

pub fn get_input() -> Vec<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|s| s.to_string()).collect()
}