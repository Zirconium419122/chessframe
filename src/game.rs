use crate::{
    bitboard::BitBoard, board::Board, chess_move::ChessMove, error::Error, piece::Piece,
    square::Square,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Game {
    pub board: Board,
    pub history: Vec<(ChessMove, Option<(Piece, Square)>)>,
    pub ply: usize,
}

impl Game {
    /// Create a new [`Game`] with the initial starting position.
    pub fn new() -> Game {
        Game {
            board: Board::default(),
            history: vec![],
            ply: 0,
        }
    }

    /// Create a new [`Game`] from a FEN string.
    pub fn from_fen(fen: &str) -> Game {
        Game {
            board: Board::from_fen(fen),
            history: vec![],
            ply: 0,
        }
    }

    /// Make a move on the [`Board`].
    ///
    /// # Example
    /// ```
    /// use chessframe::{chess_move::ChessMove, game::Game, square::Square};
    /// let fen = "r1bqk2r/pppp1pbp/2n2np1/4p1P1/2B1P3/3P1N2/PPP2P1P/RNBQK2R b KQkq - 0 6";
    /// let mut game = Game::from_fen(fen);
    ///
    /// let mv = ChessMove::new(Square::H7, Square::H5);
    /// assert_eq!(game.board.en_passant_square, None);
    ///
    /// game.make_move(mv);
    /// assert_eq!(game.board.en_passant_square, Some(Square::H6));
    /// ```
    pub fn make_move(&mut self, mv: ChessMove) -> Result<(), Error> {
        let to = mv.get_move().1;
        self.history.push((
            mv,
            self.board.en_passant_square()
                .filter(|&en_passant_square| to == en_passant_square)
                .map(|_| (Piece::Pawn, to.wrapping_backward(self.board.side_to_move)))
                .or_else(|| self.board.get_piece(to).map(|captured| (captured, to))),
        ));
        self.board.make_move(&mv)?;
        self.ply += 1;
        Ok(())
    }

    /// Undo the last move made on the [`Board`].
    ///
    /// # Example
    /// ```
    /// use chessframe::{chess_move::ChessMove, game::Game, piece::Piece, square::Square};
    ///
    /// let fen = "r1bqk2r/pppp1pb1/2n2np1/4p1Pp/2B1P3/3P1N2/PPP2P1P/RNBQK2R w KQkq h6 0 7";
    /// let mut game = Game::from_fen(fen);
    ///
    /// let mv = ChessMove::new(Square::G5, Square::H6);
    /// assert_eq!(game.board.get_piece(Square::H5), Some(Piece::Pawn));
    ///
    /// game.make_move(mv);
    /// assert_eq!(game.board.get_piece(Square::H5), None);
    ///
    /// game.undo_move();
    /// assert_eq!(game.board.get_piece(Square::H5), Some(Piece::Pawn));
    /// ```
    pub fn undo_move(&mut self) {
        let (from, to) = self.history[self.ply - 1].0.get_move();
        if let Some(piece) = self.board.get_piece(to) {
            self.board
                .xor(BitBoard::from_square(to), piece, !self.board.side_to_move);
            self.board
                .xor(BitBoard::from_square(from), piece, !self.board.side_to_move);

            if let Some((captured, square)) = self.history[self.ply - 1].1 {
                self.board.xor(
                    BitBoard::from_square(square),
                    captured,
                    self.board.side_to_move,
                );
            }

            self.board.side_to_move = !self.board.side_to_move;
            self.ply -= 1;
        }
    }
}
