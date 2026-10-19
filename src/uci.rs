use crate::board::Bitboards;

pub fn parse_uci(input: &[String], board: &mut Bitboards) {
    match input[0].as_str() {
        "uci" => {
            println!("id name ChessBot");
            println!("id author VojtechReznicek");
            println!("uciok");
        },
        "isready" => println!("readyok"),
        "position" => {
            if input.len() > 1 && input[1] == "startpos" {
                *board = Bitboards::new();

                if input.len() > 3 && input[2] == "moves" {

                    board.make_moves(&input[3..]);
                }

                board.print_board();
            }
        },
        "quit" => std::process::exit(0),
        _ => {}
    }
}
