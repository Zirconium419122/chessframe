use crate::{
    bitboard::BitBoard,
    board::Board,
    chess_move::{ChessMove, MoveMetaData},
    color::Color,
    error::Error,
    file::File,
    piece::Piece,
    square::Square,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum Event {
    Move((ChessMove, MoveMetaData)),
    Checkmate,
    Stalemate,
    DrawByThreefoldRepetition,
    Resignation(Color),
    Timeout(Color),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Game {
    pub board: Board,
    pub history: Vec<Event>,
    pub ply: usize,
}

impl Game {
    /// Create a new [`Game`] with the initial starting position.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, game::Game};
    ///
    /// let game = Game::new();
    ///
    /// assert_eq!(game.board, Board::default());
    pub fn new() -> Game {
        Game {
            board: Board::default(),
            history: vec![],
            ply: 0,
        }
    }

    /// Create a new [`Game`] from a FEN string.
    ///
    /// # Parameters
    /// - `fen` a string slice representing the FEN string.
    ///
    /// # Example
    /// ```
    /// use chessframe::game::Game;
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let game = Game::from_fen(fen);
    ///
    /// assert_eq!(game, Game::new());
    /// ```
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
        let metadata = self.board.make_move_metadata(&mv)?;
        self.history.push(Event::Move((mv, metadata)));
        self.ply += 1;
        Ok(())
    }

    /// Undo the last move made on the [`Board`].
    ///
    /// # Examples
    ///
    /// Undo a en passant move:
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
    ///
    /// Undo a castling move:
    /// ```
    /// use chessframe::{chess_move::ChessMove, game::Game, piece::Piece, square::Square};
    ///
    /// let fen = "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 1 5";
    /// let mut game = Game::from_fen(fen);
    ///
    /// let mv = ChessMove::new(Square::E1, Square::G1);
    /// assert_eq!(game.board.get_piece(Square::F1), None);
    /// assert_eq!(game.board.get_piece(Square::G1), None);
    ///
    /// game.make_move(mv);
    /// assert_eq!(game.board.get_piece(Square::F1), Some(Piece::Rook));
    /// assert_eq!(game.board.get_piece(Square::G1), Some(Piece::King));
    ///
    /// game.undo_move();
    /// assert_eq!(game.board.get_piece(Square::F1), None);
    /// assert_eq!(game.board.get_piece(Square::G1), None);
    /// ```
    ///
    /// Undo a promotion move:
    /// ```
    /// use chessframe::{chess_move::ChessMove, game::Game, piece::Piece, square::Square};
    ///
    /// let fen = "8/1PK5/7b/6k1/8/8/8/8 w - - 0 1";
    /// let mut game = Game::from_fen(fen);
    ///
    /// let mv = ChessMove::new_promotion(Square::B7, Square::B8, Piece::Queen);
    /// assert_eq!(game.board.get_piece(Square::B8), None);
    ///
    /// game.make_move(mv);
    ///
    /// assert_eq!(game.board.get_piece(Square::B8), Some(Piece::Queen));
    ///
    /// game.undo_move();
    /// assert_eq!(game.board.get_piece(Square::B8), None);
    /// assert_eq!(game.board.get_piece(Square::B7), Some(Piece::Pawn));
    /// ```
    pub fn undo_move(&mut self) {
        let moves: Vec<(usize, (ChessMove, MoveMetaData))> = self
            .history
            .iter()
            .enumerate()
            .filter(|x| matches!(x.1, Event::Move(_)))
            .map(|x| {
                if let Event::Move(mv) = x.1 {
                    (x.0, *mv)
                } else {
                    unreachable!()
                }
            })
            .collect();
        let mv_with_index = moves[self.ply - 1];

        let mv = mv_with_index.1;
        let (from, to) = mv.0.get_move();

        if let Some(piece) = self.board.get_piece(to) {
            self.board
                .xor(BitBoard::from_square(to), piece, !self.board.side_to_move);
            self.board.xor(
                BitBoard::from_square(from),
                if mv.0.promotion().is_some() {
                    Piece::Pawn
                } else {
                    piece
                },
                !self.board.side_to_move,
            );

            match mv.1 {
                MoveMetaData::Capture(captured, square) => {
                    self.board.xor(
                        BitBoard::from_square(square),
                        captured,
                        self.board.side_to_move,
                    );
                }
                MoveMetaData::EnPassant(square) => {
                    self.board.xor(
                        BitBoard::from_square(square),
                        Piece::Pawn,
                        self.board.side_to_move,
                    );
                }
                MoveMetaData::Castle => {
                    let backrank = (!self.board.side_to_move).to_backrank();
                    let (rook_start, rook_end) = match to.file() {
                        File::G => (
                            Square::make_square(backrank, File::F),
                            Square::make_square(backrank, File::H),
                        ),
                        File::C => (
                            Square::make_square(backrank, File::D),
                            Square::make_square(backrank, File::A),
                        ),
                        _ => unreachable!(),
                    };

                    self.board.xor(
                        BitBoard::from_square(rook_start),
                        Piece::Rook,
                        !self.board.side_to_move,
                    );
                    self.board.xor(
                        BitBoard::from_square(rook_end),
                        Piece::Rook,
                        !self.board.side_to_move,
                    );
                }
                _ => {}
            }

            self.board.side_to_move = !self.board.side_to_move;
            self.ply -= 1;
            self.history.truncate(mv_with_index.0);
        }
    }
}
