use core::fmt;

use crate::{color::Color, piece::Piece, square::Square};

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
    /// use chessframe::{board::Board, chess_move::ChessMove, square::Square};
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
    /// use chessframe::{board::Board, chess_move::ChessMove, piece::Piece, square::Square};
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
    /// use chessframe::{chess_move::ChessMove, square::Square};
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
    /// use chessframe::{chess_move::ChessMove, piece::Piece, square::Square};
    ///
    /// let mv = ChessMove::new_promotion(Square::B7, Square::B8, Piece::Queen);
    ///
    /// assert_eq!(mv.promotion(), Some(Piece::Queen));
    /// ```
    pub fn promotion(&self) -> Option<Piece> {
        self.promotion
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Default)]
pub enum MoveMetaData {
    #[default]
    None,
    PawnMove,
    Capture(Piece, Square),
    EnPassant(Square),
    Castle,
}

impl MoveMetaData {
    /// Creates a new `MoveMetaData` instance based on the move's properties.
    ///
    /// # Parameters
    /// - `captured`: An optional [`Piece`] that was captured during the move.
    /// - `square`: The [`Square`] where the move took place.
    /// - `en_passant`: A [`bool`] indicating whether the move was an en passant capture.
    /// - `castle`: A [`bool`] indicating whether the move was a castling move.
    /// - `color`: The [`Color`] of the side that made the move.
    ///
    /// # Returns
    /// A [`MoveMetaData`] variant corresponding to the move type:
    /// - [`MoveMetaData::Capture`] if a piece was captured.
    /// - [`MoveMetaData::EnPassant`] if an en passant capture occurred.
    /// - [`MoveMetaData::Castle`] if the move was a castling move.
    /// - [`MoveMetaData::None`] if the move had no special properties.
    ///
    /// # Example
    /// ```
    /// use chessframe::{chess_move::MoveMetaData, color::Color, piece::Piece, square::Square};
    ///
    /// let piece = Piece::Pawn;
    /// let captured = Some(Piece::Pawn);
    /// let square = Square::D5;
    /// let en_passant = false;
    /// let castle = false;
    /// let color = Color::White;
    ///
    /// let move_metadata = MoveMetaData::new(square, piece, captured, en_passant, castle, color);
    ///
    /// assert_eq!(
    ///     move_metadata,
    ///     MoveMetaData::Capture(Piece::Pawn, Square::D5),
    /// );
    /// ```
    pub fn new(
        square: Square,
        moved: Piece,
        captured: Option<Piece>,
        en_passant: bool,
        castle: bool,
        color: Color,
    ) -> MoveMetaData {
        match (captured, en_passant, castle) {
            (Some(captured), _, _) => MoveMetaData::Capture(captured, square),
            (_, true, _) => MoveMetaData::EnPassant(square.wrapping_backward(color)),
            (_, _, true) => MoveMetaData::Castle,
            _ => {
                if moved != Piece::Pawn {
                    MoveMetaData::None
                } else {
                    MoveMetaData::PawnMove
                }
            }
        }
    }

    /// Returns the captured piece and its position, if the move was a capture.
    ///
    /// # Returns
    /// - `Some((Piece, Square))` if the move captured a piece.
    /// - `None` if no piece was captured.
    pub fn capture(&self) -> Option<(Piece, Square)> {
        if let MoveMetaData::Capture(captured, square) = *self {
            Some((captured, square))
        } else {
            None
        }
    }

    /// Returns the en passant target square if the move was an en passant.
    ///
    /// # Returns
    /// - `Some(Square)` if the move was an en passant.
    /// - `None` otherwise.
    pub fn en_passant(&self) -> Option<Square> {
        if let MoveMetaData::EnPassant(square) = *self {
            Some(square)
        } else {
            None
        }
    }

    /// Checks if the move was a castling move.
    ///
    /// # Returns
    /// - `true` if the move was castling.
    /// - `false` otherwise.
    pub fn castle(&self) -> bool {
        self == &MoveMetaData::Castle
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
