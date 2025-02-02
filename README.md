# chessframe

chessframe is a Rust library for working with chess positions, generating psudo-legal moves, and interacting with the UCI protocol. It provides efficient tools for move generation and board manipulation.

## Features

- Load positions from FEN strings.
- Generate legal moves efficiently.

## Examples

### Loading a position from a FEN string

Creating a `Board` from a FEN string is easy with the `Board::from_fen()` constructor. The `Board::default()` method constructs the starting position using the standard starting FEN.

```rust
use chessframe::board::Board;

let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
let board = Board::from_fen(fen);
```

### Making a Move

This example demonstrates how to make a move on the chessboard. Since `Board` does not maintain move history, you need to use `board.make_move_new()` to create a new board instead of modifying the current one. However, in this example, we use `board.make_move()`, which updates the board state but does not allow undoing the move.

```rust
use chessframe::{board::Board, chess_move::ChessMove, square::Square};

let mut board = Board::default();

let mv = ChessMove::new(Square::E2, Square::E4);

assert_eq!(board.make_move(&mv), Ok(()));
```

### Using `make_move_new` for Perft Tests

The following example implements a Perft test, which counts all possible positions after a given number of moves.

```rust
use chessframe::{board::Board, bitboard::EMPTY};

fn perft(board: &Board, depth: usize) -> usize {
    let mut count = 0;

    for mv in board.generate_moves_vec(!EMPTY) {
        if let Ok(ref board) = board.make_move_new(&mv) {
            let perft_results = if depth == 1 {
                1
            } else {
                self.perft(board, depth - 1)
            };
            count += perft_results;
        }
    }

    count
}

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen);

    let depth = 5;
    println!("The number of nodes after {} moves is: {}", depth, perft(&board, depth));
}
```
