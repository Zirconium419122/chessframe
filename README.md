# chessframe

chessframe is a Rust library for working with chess positions, generating pseudo-legal moves, and handling Universal Chess Interface (UCI) messages.

## Installation

Run `cargo add chessframe` in your Rust project to install the [published crate](https://crates.io/crates/chessframe). To browse the API documentation locally, run `cargo doc --no-deps --features game --open` from this repository.

## Cargo features

All features are opt-in.

| Feature | What it enables |
| --- | --- |
| `game` | The `Game` type, with move history, undo, and game-ending events. Enable it with `cargo add chessframe --features game`. |
| `bmi2` | PEXT-based bishop and rook attack lookups. Both the build host and the machine running the library need x86-64 with BMI2 support. The default build uses magic bitboards. |
| `rand-chacha` | The standalone `magic_gen` developer tool. |

## Examples

### Loading a position from a FEN string

`Board::from_fen` expects a six-field FEN and can panic on malformed input. `Board::default()` creates the starting position. `Board::to_fen` serializes a position with a fullmove number of `1`.

```rust
use chessframe::board::Board;

let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
let board = Board::from_fen(fen);
assert_eq!(board.to_fen(), fen);
```

### Making a move

`Board::make_move` applies a move in place and restores the original position if it returns an error. `Board::make_move_new` applies the move to a copy and returns the resulting board. Both take a `ChessMove` by value and expect a generated or prevalidated move.

Parse coordinate notation such as `e2e4` or `b7b8q` into a `ChessMove`. Match it against the generated moves before applying it. Applying the move checks whether the moving side’s king is in check afterward.

```rust
use chessframe::{bitboard::EMPTY, board::Board, chess_move::ChessMove, piece::Piece, square::Square};

let mut board = Board::default();
let mv: ChessMove = "e2e4".parse().unwrap();

assert!(board.generate_moves_vec(!EMPTY).contains(&mv));
board.make_move(mv).unwrap();
assert_eq!(board.get_piece(Square::E4), Some(Piece::Pawn));
```

### Making and undoing a move using `Game`

Enable the `game` feature to keep a move history. Use `Game::play_move` to also record checkmate, stalemate, threefold repetition, or a draw under the fifty-move rule when detected after a move.

```rust
use chessframe::{board::Board, chess_move::ChessMove, game::Game, piece::Piece, square::Square};

let mut game = Game::new();
let mv = ChessMove::new(Square::E2, Square::E4);

game.make_move(mv).unwrap();
assert_eq!(game.board().get_piece(Square::E4), Some(Piece::Pawn));
game.undo_move();
assert_eq!(game.board(), &Board::default());
```

### Counting legal move sequences (perft)

`generate_moves_vec` returns pseudo-legal moves for the side to move. Its mask selects destination squares, and `!EMPTY` selects the entire board. A mask containing only occupied squares omits en passant because its destination is empty.

For a caller-provided buffer, use `generate_moves` and read only the prefix indicated by the returned count. The buffer must fit every generated move, including all promotion choices. Use the vector API when capacity is unknown.

Perft counts legal move sequences at a given depth. Skip any generated move that `make_move_new` rejects.

```rust
use chessframe::{bitboard::EMPTY, board::Board};

fn perft(board: &Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    board.generate_moves_vec(!EMPTY)
        .into_iter()
        .filter_map(|mv| board.make_move_new(mv).ok())
        .map(|next| perft(&next, depth - 1))
        .sum()
}

assert_eq!(perft(&Board::default(), 3), 8_902);
```

### Running a UCI example

`UciCommand` parses individual UCI lines, and the `Uci` trait lets an engine define how it reads, sends, and handles commands. The [examples](examples/) use standard input and output. This runs the random-move engine from the starting position and asks it for a move.

```sh
printf 'uci\nisready\nposition startpos\ngo\nquit\n' | cargo run --example random
```

## Development

Use a recent stable Rust toolchain. On x86-64 Linux with Nix, run `nix develop` to enter the repository's development shell.

```sh
cargo test --features game --lib --test board --test chess_move --test move_generation --test uci
cargo test --doc --features game
cargo test --release --test perft test_perft_depth_3
cargo run --release --bin movegen_bench
```

Remove the depth filter to run the full perft suite. Some cases visit billions of nodes.

Cargo generates the lookup tables during the build. To search for smaller bishop and rook magic tables, run the standalone generator with `OUT_DIR` set to an existing output directory.

```sh
cargo build --release --features rand-chacha --bin magic_gen --target-dir target
mkdir -p target/generated-magic
OUT_DIR=target/generated-magic ./target/release/magic_gen
```

Wait for the generator to report 64 of 64 squares for both pieces, then enter `stop` to write `magic_tables.rs` to that directory.
