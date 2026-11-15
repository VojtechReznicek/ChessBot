# ChessBot

A custom UCI chess engine written from scratch in Rust, focused on zero-cost abstractions and maximum performance.

## What it is
This is a long-term project to build a complete chess engine capable of full legal move generation, Negamax search, and position evaluation. It communicates via the Universal Chess Interface (UCI) protocol, meaning it can be plugged directly into any standard chess GUI or connected to online platforms like Lichess.

## How it's built
The engine is developed purely in **Rust**. At its core, it uses **bitboards** (64-bit integers) to represent the board state efficiently. To maximize runtime speed, piece attack patterns are pre-calculated at compile time using CTFE (Compile-Time Function Execution).

## How to run it

### 1. Play against the engine in a Chess GUI (Recommended)
Because ChessBot communicates over the standard UCI protocol, you don't need to interact with it through raw terminal commands. Any UCI-compatible chess interface will work—**[En Croissant](https://encroissant.org/)** is recommended for its modern interface and cross-platform support, though alternatives like **CuteChess** etc. work just as well.

1. Download the latest pre-compiled executable from the **[Releases tab](https://github.com/VojtechReznicek/ChessBot/releases)** (or compile it yourself from source).
2. Open your preferred chess GUI and navigate to its engine settings (e.g., *Engines → Add New* in En Croissant).
3. Select the downloaded `ChessBot` binary and start a game.

### 2. Build from source & CLI ASCII mode
If you have the Rust toolchain installed, you can compile the release binary yourself:

```bash
cargo build --release
```

The compiled executable will be located in `target/release/`.

For quick testing directly in the terminal without a GUI, you can also run the built-in ASCII board mode:

```bash
cargo run --release -- --ascii
```