# ChessBot

A custom UCI chess engine written from scratch in Rust, focused on zero-cost abstractions and maximum performance.

## What it is
This is a long-term project to build a complete chess engine capable of full move generation, Minimax search, and position evaluation. It uses the Universal Chess Interface (UCI) protocol to communicate, meaning it can eventually be plugged into standard chess GUIs (like Cutechess or Arena) or play online (Lichess and so).

## How it's built
The engine is developed purely in **Rust**. At its core, it uses **bitboards** (64-bit integers) to represent the board state efficiently. To maximize runtime speed, piece attack patterns are pre-calculated at compile time using CTFE (Compile-Time Function Execution).

## How to run it
To compile and run the engine from the source code, ensure you have the Rust toolchain installed and execute:

    cargo run --release

*Note: For pre-compiled testing executables and step-by-step CLI instructions, please check the [Releases tab](https://github.com/VojtechReznicek/ChessBot/releases).*