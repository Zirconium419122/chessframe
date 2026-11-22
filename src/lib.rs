//! # chessframe
//!
//! Load chess positions, generate pseudo-legal moves, and parse Universal Chess
//! Interface (UCI) messages.
//!
//! [`board::Board`] represents a position. Enable the `game` feature for
//! `game::Game`, which keeps move history and supports undo. The `bmi2` feature
//! selects PEXT-based sliding-piece attack tables and requires x86-64 with BMI2
//! on both the build host and the machine running the library.
//!
//! ## Example
//!
//! Generate moves from the starting position and play `e2e4`.
//!
//! ```
//! use chessframe::{bitboard::EMPTY, board::Board, chess_move::ChessMove, piece::Piece, square::Square};
//!
//! let mut board = Board::default();
//! assert_eq!(board.generate_moves_vec(!EMPTY).len(), 20);
//!
//! let mv: ChessMove = "e2e4".parse().unwrap();
//! board.validate_move(mv).unwrap();
//! board.make_move(mv).unwrap();
//! assert_eq!(board.get_piece(Square::E4), Some(Piece::Pawn));
//! ```

pub mod bitboard;
pub mod board;
pub mod castling_rights;
pub mod chess_move;
pub mod color;
pub mod error;
pub mod file;
#[cfg(feature = "game")]
pub mod game;
pub mod magic;
pub mod piece;
pub mod rank;
pub mod square;
pub mod transpositiontable;
pub mod uci;
