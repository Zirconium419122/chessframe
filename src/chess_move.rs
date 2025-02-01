use core::fmt;

use crate::{piece::Piece, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Default)]
pub struct ChessMove {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Piece>,
}

impl fmt::Display for ChessMove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (rank_from, file_from) = (
            unsafe { self.from.rank().to_index().unchecked_add(1) },
            self.from.file().to_index(),
        );
        let (rank_to, file_to) = (
            unsafe { self.to.rank().to_index().unchecked_add(1) },
            self.to.file().to_index(),
        );

        let file_from_char = (file_from as u8 + b'a') as char;
        let file_to_char = (file_to as u8 + b'a') as char;

        if let Some(promotion) = self.promotion {
            return write!(
                f,
                "{}{}{}{}{}",
                file_from_char,
                rank_from,
                file_to_char,
                rank_to,
                promotion.to_fen()
            );
        }

        write!(
            f,
            "{}{}{}{}",
            file_from_char, rank_from, file_to_char, rank_to
        )
    }
}

impl ChessMove {
    /// Create a new `ChessMove` given a `from` and `to` square.
    ///
    /// # Parameters
    /// - `from` a [`Square`] representing the starting square of the move.
    /// - `to` a [`Square`] representing the ending square of the move.
    ///
    /// # Example
    /// ```
    /// use chess_frame::{board::Board, chess_move::ChessMove, square::Square};
    ///
    /// let board = Board::default();
    /// let mv = ChessMove::new(Square::E2, Square::E4);
    ///
    /// assert_eq!(board.make_move_new(&mv), Ok(Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1")));
    /// ```
    pub fn new(from: Square, to: Square) -> ChessMove {
        ChessMove {
            from,
            to,
            promotion: None,
        }
    }

    /// Create a new `ChessMove` given a `from` and `to` square and a `promotion` piece.
    ///
    /// # Parameters
    /// - `from` a [`Square`] representing the starting square of the move.
    /// - `to` a [`Square`] representing the ending square of the move.
    /// - `promotion` a [`Piece`] representing the piece to promote to.
    ///
    /// # Example
    /// ```
    /// use chess_frame::{board::Board, chess_move::ChessMove, piece::Piece, square::Square};
    ///
    /// let fen = "7r/1Pk5/8/K7/8/8/8/1R6 w - - 0 1";
    /// let board = Board::from_fen(fen);
    /// let mv = ChessMove::new_promotion(Square::B7, Square::B8, Piece::Queen);
    ///
    /// assert_eq!(board.make_move_new(&mv), Ok(Board::from_fen("1Q5r/2k5/8/K7/8/8/8/1R6 b - - 0 1")));
    /// ```
    pub fn new_promotion(from: Square, to: Square, promotion: Piece) -> ChessMove {
        ChessMove {
            from,
            to,
            promotion: Some(promotion),
        }
    }

    /// Get the `from` and `to` squares of the move.
    ///
    /// # Returns
    /// - `(Square, Square)` a tuple containing the `from` and `to` squares of the move.
    ///
    /// # Example
    /// ```
    /// use chess_frame::{chess_move::ChessMove, square::Square};
    ///
    /// let mv = ChessMove::new(Square::E2, Square::E4);
    ///
    /// assert_eq!(mv.get_move(), (Square::E2, Square::E4));
    /// ```
    pub fn get_move(&self) -> (Square, Square) {
        (self.from, self.to)
    }

    /// Get the promotion piece of the move.
    ///
    /// # Returns
    /// - [`Option<Piece>`] an optional [`Piece`] representing the promotion piece of the move.
    ///
    /// # Example
    /// ```
    /// use chess_frame::{chess_move::ChessMove, piece::Piece, square::Square};
    ///
    /// let mv = ChessMove::new_promotion(Square::B7, Square::B8, Piece::Queen);
    ///
    /// assert_eq!(mv.promotion(), Some(Piece::Queen));
    /// ```
    pub fn promotion(&self) -> Option<Piece> {
        self.promotion
    }
}

#[deprecated(
    since = "0.0.0",
    note = "MoveType has been phased out of the make_move and generate_moves_vec methods and is therefore not needed any longer for move handling."
)]
#[derive(Debug, Clone, PartialEq)]
pub enum MoveType {
    Quiet,
    Capture,
    Check,
    Castle,
    EnPassant,
    Promotion(Piece),
    CapturePromotion(Piece),
}
