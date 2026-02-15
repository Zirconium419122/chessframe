use std::collections::HashMap;

use crate::{
    bitboard::{BitBoard, EMPTY},
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
    DrawByFiftyMoveRule,
    Resignation(Color),
    Timeout(Color),
}

impl Event {
    pub fn is_gameending(&self) -> bool {
        !matches!(self, Event::Move(_))
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Game {
    pub board: Board,
    pub history: Vec<Event>,
    hashes: Vec<u64>,
    pub ply: usize,
    half_moves: usize,
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
        let board = Board::default();
        Game {
            board,
            history: vec![],
            hashes: vec![board.hash()],
            ply: 0,
            half_moves: 0,
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
        let board = Board::from_fen(fen);
        Game {
            board,
            history: vec![],
            hashes: vec![board.hash()],
            ply: 0,
            half_moves: fen
                .split_whitespace()
                .nth(4)
                .unwrap_or("0")
                .parse()
                .unwrap_or(0),
        }
    }

    /// Get the current [`Board`] of the [`Game`].
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Get the history vector of the [`Game`].
    pub fn history(&self) -> &Vec<Event> {
        &self.history
    }

    /// Get the hashes vector of the [`Game`].
    pub fn hashes(&self) -> &Vec<u64> {
        &self.hashes
    }

    /// Get the ply of the [`Game`].
    pub fn ply(&self) -> usize {
        self.ply
    }

    /// Get the half-move clock of the [`Game`].
    pub fn half_moves(&self) -> usize {
        self.half_moves
    }

    /// Resign the game provided a [`Color`] that resigns.
    pub fn resign(&mut self, color: Color) -> Result<(), Error> {
        if let Some(event) = self.history.last()
            && event.is_gameending()
        {
            return Err(Error::GameEnded);
        }

        self.history.push(Event::Resignation(color));

        Ok(())
    }

    /// Timeout the game provided a [`Color`] that times out.
    ///
    /// # Parameters
    /// - `color`: The [`Color`] that times out, should be the current side to move.
    ///
    /// # Returns
    /// - `Ok(())` if the timeout was successfully recorded.
    /// - `Err(Error)` if the game has already ended.
    ///
    /// # Example
    /// ```
    /// use chessframe::{color::Color, game::{Event, Game}};
    ///
    /// let mut game = Game::new();
    ///
    /// let _ = game.timeout(Color::White);
    ///
    /// assert_eq!(game.history.last(), Some(&Event::Timeout(Color::White)));
    /// ```
    pub fn timeout(&mut self, color: Color) -> Result<(), Error> {
        if let Some(event) = self.history.last()
            && event.is_gameending()
        {
            return Err(Error::GameEnded);
        }

        self.history.push(Event::Timeout(color));

        Ok(())
    }

    /// Play a move on the current [`Board`].
    ///
    /// # Parameters
    /// - `mv`: A [`ChessMove`] representing the move to make.
    ///   The move must be at least pseudo-legal; invalid or unchecked moves will result in undefined behavior.
    ///
    /// # Returns
    /// - `Ok(())` if the move was successfully played.
    /// - `Err(Error)` if the move is pseudo-legal but not legal, or if the game has already ended.
    ///
    /// # Examples
    ///
    /// Play till threefold repetition:
    /// ```
    /// use chessframe::{chess_move::ChessMove, game::{Event, Game}, square::Square};
    ///
    /// let mut game = Game::from_fen("5q2/5k2/7p/8/8/8/1B4Q1/6K1 w - - 3 3");
    ///
    /// let g2_f3 = ChessMove::new(Square::G2, Square::F3);
    /// let f7_e8 = ChessMove::new(Square::F7, Square::E8);
    /// let f3_c6 = ChessMove::new(Square::F3, Square::C6);
    /// let e8_f7 = ChessMove::new(Square::E8, Square::F7);
    /// let c6_f3 = ChessMove::new(Square::C6, Square::F3);
    ///
    /// game.play_move(g2_f3);
    /// game.play_move(f7_e8);
    /// game.play_move(f3_c6);
    /// game.play_move(e8_f7);
    /// game.play_move(c6_f3);
    /// game.play_move(f7_e8);
    /// game.play_move(f3_c6);
    /// game.play_move(e8_f7);
    /// game.play_move(c6_f3);
    ///
    /// assert_eq!(game.history.last(), Some(&Event::DrawByThreefoldRepetition));
    /// ```
    ///
    /// Make a move that results in checkmate:
    /// ```
    /// use chessframe::{chess_move::ChessMove, game::{Event, Game}, square::Square};
    ///
    /// let mut game = Game::from_fen("7k/7p/7K/5Q2/8/8/8/8 w - - 0 1");
    ///
    /// let mv = ChessMove::new(Square::F5, Square::F8);
    ///
    /// let _ = game.play_move(mv);
    ///
    /// assert_eq!(game.history.last(), Some(&Event::Checkmate));
    /// ```
    ///
    /// Make a move that results in stalemate:
    /// ```
    /// use chessframe::{chess_move::ChessMove, game::{Event, Game}, square::Square};
    ///
    /// let mut game = Game::from_fen("7k/7p/7K/5Q2/8/8/8/8 w - - 0 1");
    ///
    /// let mv = ChessMove::new(Square::F5, Square::F7);
    ///
    /// let _ = game.play_move(mv);
    ///
    /// assert_eq!(game.history.last(), Some(&Event::Stalemate));
    /// ```
    pub fn play_move(&mut self, mv: ChessMove) -> Result<(), Error> {
        if let Some(event) = self.history.last()
            && event.is_gameending()
        {
            return Err(Error::GameEnded);
        }

        self.make_move(mv)?;

        if let Some(Event::Move((_, metadata))) = self
            .history
            .iter()
            .filter(|x| matches!(x, Event::Move(_)))
            .next_back()
        {
            match metadata {
                MoveMetaData::Capture(..)
                | MoveMetaData::PawnMove
                | MoveMetaData::EnPassant(..) => {
                    self.half_moves = 0;
                }
                _ => self.half_moves += 1,
            }
        }

        let legal_moves = self
            .board
            .generate_moves_vec(!EMPTY)
            .into_iter()
            .filter(|x| self.board.make_move_new(*x).is_ok())
            .collect::<Vec<ChessMove>>();

        if legal_moves.is_empty() {
            if self.board.in_check() {
                self.history.push(Event::Checkmate);
            } else {
                self.history.push(Event::Stalemate);
            }

            return Ok(());
        } else {
            let mut count_map = HashMap::new();

            for hash in &self.hashes {
                let count = count_map.entry(hash).or_insert(0);
                *count += 1;

                if *count == 3 {
                    self.history.push(Event::DrawByThreefoldRepetition);
                    return Ok(());
                }
            }
        }

        if self.half_moves >= 100 {
            self.history.push(Event::DrawByFiftyMoveRule);
        }

        Ok(())
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
        let metadata = self.board.make_move_metadata(mv)?;
        self.history.push(Event::Move((mv, metadata)));
        self.hashes.push(self.board.hash());
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
