# ChessBot

A fast, zero-dependency UCI chess engine written from scratch in pure Rust.

ChessBot is built around bitboard representation, compile-time precomputed attack tables, and an alpha-beta Negamax search with iterative deepening. It implements the Universal Chess Interface (UCI) protocol, allowing it to run inside any standard chess GUI or engine tournament manager.

---

## Features

### Board Representation & Move Generation
* **Bitboard architecture:** 12 `u64` bitboards tracking piece placement with fast bitwise state transitions and lightweight `Undo` structs for `make_move` / `unmake_move`.
* **Compile-time attack tables (CTFE):** Knight, king, and directional sliding ray tables are computed at compile time via `const fn` for zero runtime initialization overhead.
* **Complete legal chess rules:** Full move generation covering double pawn pushes, *en passant*, kingside/queenside castling with path and check validation, and all underpromotions.
* **FEN support:** Native `GameState::fen` parser supporting both full 6-part and shortened 4-part FEN strings.

### Search & Evaluation
* **Negamax with Alpha-Beta pruning:** Efficient game-tree search with depth-adjusted checkmate scoring (preferring faster mates) and stalemate detection.
* **Iterative deepening & time management:** Dynamic time allocation based on remaining clock and increment (`wtime`, `btime`, `winc`, `binc`) with periodic timeout checks and a guaranteed depth-1 legal move fallback.
* **Evaluation:** Fast handcrafted evaluation combining material balance, pawn chain defense bonuses, and territorial aggression.

### Engineering
* **Zero external dependencies:** Built exclusively on the Rust standard library (`std`).
* **Verified move generation:** Covered by unit tests and a Perft test suite across standard benchmark positions (including *Kiwipete*, en passant pin endgames, and promotion/castling edge cases).

---

## Playing Strength

Estimated strength: **~1475 ELO** (measured in a 48-game time-controlled CuteChess gauntlet against Stockfish with zero crashes or time losses).

| Opponent | Score |
| :--- | :---: |
| Stockfish (1320 ELO) | 84.4% |
| Stockfish (1500 ELO) | 28.1% |
| Stockfish (1800 ELO) | 9.4% (1.5 / 16) |

---

## Project Architecture

```text
src/
├── main.rs            # Entry point, CLI flag parsing (--ascii), and main UCI loop
├── uci.rs             # UCI protocol parser and ASCII debug board/move printers
├── board.rs           # GameState bitboards, FEN parser, make/unmake, attack detection
├── movespreloader.rs  # Compile-time (const fn) attack and sliding ray lookup tables
├── movegen.rs         # Pseudo-legal move generator and Perft test suite
├── search.rs          # Iterative deepening, Negamax with Alpha-Beta, time management
└── eval.rs            # Static position evaluation
```

---

## Getting Started

### Option 1: Play in a Chess GUI (Recommended)

Any UCI-compatible GUI works out of the box. **[En Croissant](https://encroissant.org/)** is recommended for everyday play, while **CuteChess** is ideal for automated engine-vs-engine matches.

1. Download the latest pre-compiled binary (`ChessBot` for Linux or `ChessBot.exe` for Windows) from the **[Releases](https://github.com/VojtechReznicek/ChessBot/releases)** page.
   *(On Linux, grant execution permissions with `chmod +x ChessBot`.)*
2. Open your chess GUI and register the binary as a new UCI engine (e.g., *Engines → Add New* in En Croissant).
3. Start a game.

### Option 2: Build from Source

Requires the [Rust toolchain](https://rustup.rs/).

```bash
git clone [https://github.com/VojtechReznicek/ChessBot.git](https://github.com/VojtechReznicek/ChessBot.git)
cd ChessBot
cargo build --release
```

The compiled executable will be generated at `target/release/ChessBot` (or `ChessBot.exe` on Windows).

### Running Tests

To run the unit tests and Perft verification suite (release mode is recommended for speed):

```bash
cargo test --release
```

### Terminal / ASCII Mode

ChessBot includes a built-in `--ascii` flag for visual board inspection and manual UCI testing directly in the terminal. For exact CLI usage and example UCI commands, see the latest **[Release Notes](https://github.com/VojtechReznicek/ChessBot/releases)**.

---

## Supported UCI Commands

`uci` · `isready` · `position [startpos | fen <fenstring>] [moves ...]` · `go [depth <x> | wtime <ms> btime <ms> winc <ms> binc <ms>]` · `quit`

---

## Roadmap

* Full move ordering (wiring up MVV-LVA captures, promotions, and killer moves)
* Quiescence search to eliminate horizon-effect blunders
* Transposition tables (Zobrist hashing)
* Piece-Square Tables (PST) for stronger positional play