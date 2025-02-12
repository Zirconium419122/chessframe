//! # chessframe
//! chessframe is a chess library written in Rust. It provides a simple interface for working with chess positions and generating psudo-legal moves efficiently.
//!
//! ## Example
//!
//! This generates all psudo-legal moves from the starting position and makes the move "e2e4" on the board.
//!
//! ```rust
//! use chessframe::{board::Board, bitboard::EMPTY, chess_move::ChessMove, piece::Piece, square::Square};
//!
//! let mut board = Board::default();
//! assert_eq!(board.generate_moves_vec(!EMPTY).len(), 20);
//!
//! let mv = ChessMove::new(Square::E2, Square::E4);
//!
//! let _ = board.make_move(&mv);
//! assert_eq!(board.get_piece(Square::E4), Some(Piece::Pawn));
//! ```

pub mod bitboard;
pub mod board;
pub mod castling_rights;
pub mod chess_move;
pub mod color;
pub mod error;
pub mod file;
pub mod game;
pub mod magic;
pub mod piece;
pub mod rank;
pub mod square;
pub mod uci;
