use std::{
    hash::{Hash, Hasher},
    str::FromStr,
};

use crate::{
    bitboard::{BitBoard, EMPTY},
    castling_rights::CastlingRights,
    chess_move::ChessMove,
    color::{Color, COLORS},
    error::Error,
    file::File,
    magic::*,
    piece::Piece,
    rank::Rank,
    square::Square,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Board {
    pub pieces: [BitBoard; 6],    // 6 for white, 6 for black
    pub occupancy: [BitBoard; 2], // white, black occupancy
    pub combined: BitBoard,       // combined occupancy
    pub pinned: BitBoard,
    pub check: u8,
    pub hash: u64,
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<Square>,
}

impl Default for Board {
    fn default() -> Self {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash().hash(state);
    }
}

impl Board {
    /// Create a empty `Board` which has no pieces in it.
    /// ```
    /// use chessframe::{board::Board, bitboard::EMPTY};
    ///
    /// let board = Board::new();
    ///
    /// assert_eq!(board.combined(), EMPTY);
    /// ```
    pub fn new() -> Board {
        Board {
            pieces: [EMPTY; 6],
            occupancy: [EMPTY; 2],
            combined: EMPTY,
            pinned: EMPTY,
            check: 0,
            hash: 0,
            side_to_move: Color::White,
            castling_rights: CastlingRights::default(),
            en_passant_square: None,
        }
    }

    /// Create a board from a FEN in the form of a `&str`.
    /// ```
    /// use chessframe::board::Board;
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board, Board::default());
    /// ```
    #[rustfmt::skip]
    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::new();

        let parts: Vec<&str> = fen.split_whitespace().collect();
        assert_eq!(parts.len(), 6);

        board.parse_pieces(parts[0]);

        board.combined = board.occupancy(Color::White) | board.occupancy(Color::Black);

        let knights = get_knight_moves(board.pieces_color(Piece::King, !board.side_to_move).to_square())
            & board.pieces_color(Piece::Knight, board.side_to_move);
        board.check = (knights.count_ones() > 0) as u8;

        for color in COLORS {
            let king_square = board.pieces_color(Piece::King, !color).to_square();
            let attackers = board.occupancy(color) & ((get_bishop_moves(king_square, EMPTY) & (board.pieces(Piece::Bishop) | board.pieces(Piece::Queen)))
                | (get_rook_moves(king_square, EMPTY) & (board.pieces(Piece::Rook) | board.pieces(Piece::Queen))));

            for square in attackers {
                let between = get_between(square, king_square) & board.combined();
                if between.count_ones() == 1 {
                    board.pinned ^= between & board.occupancy(!color);
                } else if between == EMPTY {
                    board.check += 1;
                }
            }
        }

        board.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid active color in FEN"),
        };

        board.castling_rights = CastlingRights::from_fen(parts[2]);

        board.parse_en_passant(parts[3]);

        board
    }

    fn parse_pieces(&mut self, piece_placement: &str) {
        let mut rank = 7;
        let mut file = 0;

        for ch in piece_placement.chars() {
            match ch {
                // Piece placement characters
                'P' => self.place_piece(Piece::Pawn, Color::White, rank, file),
                'N' => self.place_piece(Piece::Knight, Color::White, rank, file),
                'B' => self.place_piece(Piece::Bishop, Color::White, rank, file),
                'R' => self.place_piece(Piece::Rook, Color::White, rank, file),
                'Q' => self.place_piece(Piece::Queen, Color::White, rank, file),
                'K' => self.place_piece(Piece::King, Color::White, rank, file),
                'p' => self.place_piece(Piece::Pawn, Color::Black, rank, file),
                'n' => self.place_piece(Piece::Knight, Color::Black, rank, file),
                'b' => self.place_piece(Piece::Bishop, Color::Black, rank, file),
                'r' => self.place_piece(Piece::Rook, Color::Black, rank, file),
                'q' => self.place_piece(Piece::Queen, Color::Black, rank, file),
                'k' => self.place_piece(Piece::King, Color::Black, rank, file),

                // Empty squares
                '1'..='8' => {
                    file += ch.to_digit(10).unwrap() as usize;
                }

                // End of rank
                '/' => {
                    rank -= 1;
                    file = 0;
                }

                _ => panic!("Invalid character in FEN piece placement: {}", ch),
            }

            // Move to the next file if piece was placed
            if ch.is_alphabetic() {
                file += 1;
            }
        }
    }

    fn place_piece(&mut self, piece: Piece, color: Color, rank: usize, file: usize) {
        let square = Square::make_square(Rank::from_index(rank), File::from_index(file));
        self.set_piece(piece, color, square);
    }

    fn parse_en_passant(&mut self, en_passant: &str) {
        if en_passant != "-" {
            let file =
                unsafe { (en_passant.chars().nth(0).unwrap_unchecked() as u8).unchecked_sub(b'a') };
            let rank = unsafe {
                (en_passant
                    .chars()
                    .nth(1)
                    .unwrap_unchecked()
                    .to_digit(10)
                    .unwrap_unchecked() as u8)
                    .unchecked_sub(1)
            };
            let square = Square::new(rank << (3 + file));

            self.set_en_passant(square);
        }
    }

    /// Get the combined bitboard of all pieces on the board.
    /// ```
    /// use chessframe::{bitboard::BitBoard, board::Board};
    ///
    /// let board = Board::default();
    ///
    /// assert_eq!(board.combined(), BitBoard(0xFFFF00000000FFFF));
    /// ```
    #[inline]
    pub fn combined(&self) -> BitBoard {
        self.combined
    }

    /// Get a mutable reference to the combined bitboard.
    #[inline]
    pub fn combined_mut(&mut self) -> &mut BitBoard {
        &mut self.combined
    }

    /// Get the occupancy bitboard for a particular color.
    /// ```
    /// use chessframe::{bitboard::BitBoard, board::Board, color::Color};
    ///
    /// let board = Board::default();
    ///
    /// assert_eq!(board.occupancy(Color::White), BitBoard(0x000000000000FFFF));
    /// assert_eq!(board.occupancy(Color::Black), BitBoard(0xFFFF000000000000));
    /// ```
    #[inline]
    pub fn occupancy(&self, color: Color) -> BitBoard {
        unsafe { *self.occupancy.get_unchecked(color.to_index()) }
    }

    /// Get a mutable reference to the occupancy bitboard for a particular color.
    #[inline]
    pub fn occupancy_mut(&mut self, color: Color) -> &mut BitBoard {
        unsafe { self.occupancy.get_unchecked_mut(color.to_index()) }
    }

    /// Get the bitboard of a particular piece type.
    /// ```
    /// use chessframe::{bitboard::BitBoard, board::Board, piece::Piece};
    ///
    /// let board = Board::default();
    ///
    /// assert_eq!(board.pieces(Piece::Pawn), BitBoard(0x00FF00000000FF00));
    /// ```
    #[inline]
    pub fn pieces(&self, piece: Piece) -> BitBoard {
        unsafe { *self.pieces.get_unchecked(piece.to_index()) }
    }

    /// Get the bitboard of a particular piece and color.
    #[inline]
    pub fn pieces_color(&self, piece: Piece, color: Color) -> BitBoard {
        unsafe { *self.pieces.get_unchecked(piece.to_index()) & self.occupancy(color) }
    }

    /// Get a mutable reference to a particular piece and color.
    #[inline]
    pub fn pieces_mut(&mut self, piece: Piece) -> &mut BitBoard {
        unsafe { self.pieces.get_unchecked_mut(piece.to_index()) }
    }

    /// Get the zobrist hash of the board. This is different from the hash field in [`Board`] as it only contains the hash for the pieces.
    /// # Example
    /// ```
    /// use chessframe::board::Board;
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.hash(), 0x50FE28372FB16071);
    /// ```
    #[inline]
    pub fn hash(&self) -> u64 {
        self.hash
            ^ if let Some(en_passant_square) = self.en_passant_square {
                Zobrist::get_en_passant(en_passant_square.file(), !self.side_to_move)
            } else {
                0
            }
            ^ Zobrist::get_castle(self.castling_rights, self.side_to_move)
            ^ Zobrist::get_castle(self.castling_rights, !self.side_to_move)
            ^ if self.side_to_move == Color::Black {
                Zobrist::get_side_to_move()
            } else {
                0
            }
    }

    /// Looks up the check field in the [`Board`] and checks if it's above `0`.
    ///
    /// # Example
    /// ```
    /// use chessframe::board::Board;
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert!(!board.in_check());
    /// ```
    #[inline]
    pub fn in_check(&self) -> bool {
        self.check > 0
    }

    /// Get the en passant square, returns [`Option<Square>`].
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, square::Square};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.en_passant_square(), None);
    /// ```
    #[inline]
    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    fn remove_en_passant(&mut self) {
        self.en_passant_square = None;
    }

    fn set_en_passant(&mut self, square: Square) {
        if get_adjacent_files(square.file())
            & get_rank(square.wrapping_backward(!self.side_to_move).rank())
            & self.pieces_color(Piece::Pawn, !self.side_to_move)
            != EMPTY
        {
            self.en_passant_square = Some(square);
        }
    }

    /// Remove the castling rights provided in the castling_rights parameter.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, castling_rights::CastlingRights};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let mut board = Board::from_fen(fen);
    ///
    /// board.remove_castling_rights(CastlingRights::from_fen("KQ"));
    ///
    /// assert_eq!(board.castling_rights, CastlingRights::from_fen("kq"));
    /// ```
    pub fn remove_castling_rights(&mut self, castling_rights: CastlingRights) {
        self.castling_rights = self.castling_rights.remove(castling_rights);
    }

    fn xor(&mut self, bitboard: BitBoard, piece: Piece, color: Color) {
        *self.pieces_mut(piece) ^= bitboard;
        *self.occupancy_mut(color) ^= bitboard;
        *self.combined_mut() ^= bitboard;
        self.hash ^= Zobrist::get_piece(piece, bitboard.to_square(), color);
    }

    /// Check if one can castle to the given side.
    ///
    /// # Examples
    ///
    /// Starting position:
    /// ```
    /// use chessframe::board::Board;
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.can_castle(true), Err("Cannot castle kingside"));
    /// assert_eq!(board.can_castle(false), Err("Cannot castle queenside"));
    /// ```
    ///
    /// Opening position:
    /// ```
    /// use chessframe::board::Board;
    ///
    /// let fen = "r1bqk2r/ppppbppp/2n2n2/4p3/2B1P3/2P2N2/PP1P1PPP/RNBQK2R w KQkq - 1 5";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.can_castle(true), Ok(()));
    /// assert_eq!(board.can_castle(false), Err("Cannot castle queenside"));
    /// ```
    pub fn can_castle(&self, kingside: bool) -> Result<(), &str> {
        let castling_moves = self.generate_castling_moves();

        if kingside
            && (castling_moves & BitBoard::set(self.side_to_move.to_backrank(), File::G)).is_zero()
        {
            return Err("Cannot castle kingside");
        } else if !kingside
            && (castling_moves & BitBoard::set(self.side_to_move.to_backrank(), File::C)).is_zero()
        {
            return Err("Cannot castle queenside");
        }

        Ok(())
    }

    /// Infer a [`ChessMove`] from a string based on the current [`Board`].
    ///
    /// # Example:
    /// ```
    /// use chessframe::{board::Board, chess_move::ChessMove, square::Square};
    ///
    /// let mut board = Board::default();
    /// let mv = board.infer_move("e2e4");
    ///
    /// assert_eq!(mv, Ok(ChessMove::new(Square::E2, Square::E4)));
    /// ```
    pub fn infer_move(&mut self, mv: &str) -> Result<ChessMove, Error> {
        let from = Square::from_str(&mv[0..2])?;
        let to = Square::from_str(&mv[2..4])?;
        let promotion: Option<Piece> = match &mv.len() {
            4 => None,
            5 => Some(Piece::from(unsafe { mv.chars().last().unwrap_unchecked() })),
            _ => return Err(Error::InvalidMove),
        };

        if self.get_piece(from).is_some() {
            if let Some(promotion) = promotion {
                let mv = ChessMove::new_promotion(from, to, promotion);

                if self.validate_move(&mv).is_ok() {
                    return Ok(mv);
                }
            }

            let mv = ChessMove::new(from, to);

            if self.validate_move(&mv).is_ok() {
                return Ok(mv);
            }
        }

        Err(Error::NoPieceOnSquare)
    }

    /// Checks that a [`ChessMove`] is a valid move for the current board state. Does not check if the move leaves the king in check.
    pub fn validate_move(&mut self, mv: &ChessMove) -> Result<Piece, &str> {
        let (from, to) = mv.get_move();
        let piece = self.get_piece(from).ok_or("No piece found on square!")?;

        let pieces = self.pieces(piece);

        *self.pieces_mut(piece) = BitBoard::from_square(from);

        let piece_moves = match piece {
            Piece::Pawn => self.generate_pawn_moves(),
            Piece::Knight => self.generate_knight_moves(),
            Piece::Bishop => self.generate_bishop_moves(),
            Piece::Rook => self.generate_rook_moves(),
            Piece::Queen => self.generate_queen_moves(),
            Piece::King => self.generate_king_moves() | self.generate_castling_moves(),
        };

        *self.pieces_mut(piece) = pieces;

        if piece_moves.is_not_set(to) {
            return Err("Invalid move!");
        }

        Ok(piece)
    }

    /// Make a [`ChessMove`] on a copy of the current [`Board`].
    ///
    /// # Parameters
    /// - `mv`: A reference to a [`ChessMove`] representing the move to make.
    ///   The move must be either pseudo-legal or fully legal; invalid or unchecked moves
    ///   will result in undefined behavior.
    ///
    /// # Returns
    /// - `Ok(Board)` if the move is successfully made.
    /// - `Err(String)` if the move is invalid, such as attempting to move a pinned piece.
    ///
    /// # Errors
    /// - Returns an error if the resulting board state places or leaves the king in check.
    /// - Errors when attempting to move a piece from an empty square or with other invalid conditions.
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
    ///
    /// # Notes
    /// This method assumes all moves are pre-validated (pseudo-legal or legal) or generated by [`generate_moves_vec`].
    /// It does not perform a legality check before execution but will enforce
    /// certain rules (e.g., pinned pieces cannot move) during processing.
    ///
    /// [`generate_moves_vec`]: #method.generate_moves_vec
    pub fn make_move_new(&self, mv: &ChessMove) -> Result<Board, Error> {
        let mut board = *self;

        board.make_move(mv)?;

        Ok(board)
    }

    /// Make a [`ChessMove`] on the current [`Board`].
    ///
    /// # Parameters
    /// - `mv`: A reference to a [`ChessMove`] representing the move to make.
    ///   The move must be either pseudo-legal or fully legal; invalid or unchecked moves
    ///   will result in undefined behavior.
    ///
    /// # Returns
    /// - `Ok(())` if the move is successfully made.
    /// - `Err(&str)` if the move is invalid, such as attempting to move a pinned piece.
    ///
    /// # Errors
    /// - Returns an error if the resulting board state places or leaves the king in check.
    /// - Errors when attempting to move a piece from an empty square or with other invalid conditions.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, chess_move::ChessMove, square::Square};
    ///
    /// let mut board = Board::default();
    /// let mv = ChessMove::new(Square::E2, Square::E4);
    ///
    /// assert_eq!(board.make_move(&mv), Ok(()));
    /// ```
    ///
    /// # Notes
    /// This method assumes all moves are pre-validated (pseudo-legal or legal) or generated by [`generate_moves_vec`].
    /// It does not perform a legality check before execution but will enforce
    /// certain rules (e.g., pinned pieces cannot move) during processing.
    ///
    /// [`generate_moves_vec`]: #method.generate_moves_vec
    #[rustfmt::skip]
    pub fn make_move(&mut self, mv: &ChessMove) -> Result<(), Error> {
        let (from, to) = mv.get_move();

        self.check = 0;

        let from_bitboard = BitBoard::from_square(from);
        let to_bitboard = BitBoard::from_square(to);
        let move_bitboard = from_bitboard ^ to_bitboard;

        let piece = self.get_piece(from).ok_or(Error::NoPieceOnSquare)?;

        let en_passant_square = self.en_passant_square();
        self.remove_en_passant();

        if let Some(captured) = self.get_piece(to) {
            self.xor(to_bitboard, captured, !self.side_to_move);
        }
        self.xor(from_bitboard, piece, self.side_to_move);
        self.xor(to_bitboard, piece, self.side_to_move);

        self.remove_castling_rights(CastlingRights::square_to_castle_rights(
            &!self.side_to_move,
            to,
        ));

        self.remove_castling_rights(CastlingRights::square_to_castle_rights(
            &self.side_to_move,
            from,
        ));

        let king_square = self.pieces_color(Piece::King, !self.side_to_move).to_square();

        let castle = piece == Piece::King && (move_bitboard & get_castle_moves()) == move_bitboard;

        const CASTLE_ROOK_START: [File; 8] = [
            File::A,
            File::A,
            File::A,
            File::A,
            File::H,
            File::H,
            File::H,
            File::H,
        ];
        const CASTLE_ROOK_END: [File; 8] = [
            File::D,
            File::D,
            File::D,
            File::D,
            File::F,
            File::F,
            File::F,
            File::F,
        ];

        if let Piece::Knight = piece {
            self.check = (get_knight_moves(king_square) & to_bitboard != EMPTY) as u8;
        } else if let Piece::Pawn = piece {
            if let Some(Piece::Knight) = mv.promotion() {
                self.xor(BitBoard::from_square(to), Piece::Pawn, self.side_to_move);
                self.xor(BitBoard::from_square(to), Piece::Knight, self.side_to_move);
                self.check = (get_knight_moves(king_square) & to_bitboard != EMPTY) as u8;
            } else if let Some(promotion) = mv.promotion() {
                self.xor(BitBoard::from_square(to), Piece::Pawn, self.side_to_move);
                self.xor(BitBoard::from_square(to), promotion, self.side_to_move);
            } else if from.rank() == self.side_to_move.to_second_rank()
                && to.rank() == self.side_to_move.to_fourth_rank()
            {
                self.set_en_passant(to.wrapping_backward(self.side_to_move));
                self.check = (get_pawn_attacks(king_square, !self.side_to_move) & to_bitboard != EMPTY) as u8;
            } else if Some(to) == en_passant_square {
                let side_to_move = self.side_to_move;
                self.xor(
                    BitBoard::from_square(to.wrapping_backward(side_to_move)),
                    Piece::Pawn,
                    !side_to_move,
                );
                self.check = (get_pawn_attacks(king_square, !self.side_to_move) & to_bitboard != EMPTY) as u8;
            } else {
                self.check = (get_pawn_attacks(king_square, !self.side_to_move) & to_bitboard != EMPTY) as u8;
            }
        } else if castle {
            let index = to.file().to_index();
            let start = BitBoard::set(self.side_to_move.to_backrank(), unsafe {
                *CASTLE_ROOK_START.get_unchecked(index)
            });
            let end = BitBoard::set(self.side_to_move.to_backrank(), unsafe {
                *CASTLE_ROOK_END.get_unchecked(index)
            });

            self.xor(start, Piece::Rook, self.side_to_move);
            self.xor(end, Piece::Rook, self.side_to_move);
        }

        let attackers = self.occupancy(self.side_to_move) & ((get_bishop_moves(king_square, EMPTY) & (self.pieces(Piece::Bishop) | self.pieces(Piece::Queen)))
            | (get_rook_moves(king_square, EMPTY) & (self.pieces(Piece::Rook) | self.pieces(Piece::Queen))));

        for square in attackers {
            let between = get_between(square, king_square) & self.combined();
            if between.count_ones() == 1 {
                self.pinned ^= between & self.occupancy(!self.side_to_move);
            } else if between == EMPTY {
                self.check += 1;
            }
        }

        if self
            .get_attackers(
                self.pieces_color(Piece::King, self.side_to_move)
                    .to_square(),
            )
            .is_not_zero()
        {
            return Err(Error::CannotMovePinned);
        }

        self.side_to_move = !self.side_to_move;

        Ok(())
    }

    /// Get the piece at a given square.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, piece::Piece, square::Square};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.get_piece(Square::E1), Some(Piece::King));
    /// assert_eq!(board.get_piece(Square::E2), Some(Piece::Pawn));
    /// assert_eq!(board.get_piece(Square::E3), None);
    /// ```
    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        let bitboard = BitBoard::from_square(square);
        if self.combined() & bitboard == EMPTY {
            None
        } else if (self.pieces(Piece::Pawn)
            ^ self.pieces(Piece::Knight)
            ^ self.pieces(Piece::Bishop))
            & bitboard
            != EMPTY
        {
            if self.pieces(Piece::Pawn) & bitboard != EMPTY {
                Some(Piece::Pawn)
            } else if self.pieces(Piece::Knight) & bitboard != EMPTY {
                Some(Piece::Knight)
            } else {
                Some(Piece::Bishop)
            }
        } else if self.pieces(Piece::Rook) & bitboard != EMPTY {
            Some(Piece::Rook)
        } else if self.pieces(Piece::Queen) & bitboard != EMPTY {
            Some(Piece::Queen)
        } else {
            Some(Piece::King)
        }
    }

    /// Set the piece at a given square (used during board construction).
    fn set_piece(&mut self, piece: Piece, color: Color, square: Square) {
        self.xor(BitBoard::from_square(square), piece, color);
    }

    /// Generate all psuedo-legal moves.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, bitboard::BitBoard};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.generate_moves(), BitBoard(0xFFFF0000))
    /// ```
    pub fn generate_moves(&self) -> BitBoard {
        self.generate_pawn_moves()
            | self.generate_knight_moves()
            | self.generate_bishop_moves()
            | self.generate_rook_moves()
            | self.generate_queen_moves()
            | self.generate_king_moves()
    }

    /// Generate a vector of psudo-legal [`ChessMove`]'s.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, bitboard::EMPTY};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.generate_moves_vec(!EMPTY).len(), 20);
    /// ```
    #[rustfmt::skip]
    pub fn generate_moves_vec(&self, mask: BitBoard) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::with_capacity(218);

        let allied_pieces = self.occupancy(self.side_to_move);
        let opponent_occupancy = self.occupancy(!self.side_to_move);
        let blockers = self.combined();

        if self.check < 2 {
            for piece in [
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ] {
                for src in self.pieces_color(piece, self.side_to_move).into_iter() {
                    let generated_moves = match piece {
                        Piece::Knight => get_knight_moves(src),
                        Piece::Bishop => get_bishop_moves(src, blockers),
                        Piece::Rook => get_rook_moves(src, blockers),
                        Piece::Queen => get_bishop_moves(src, blockers) | get_rook_moves(src, blockers),
                        Piece::King => get_king_moves(src) | if self.check < 1 { self.generate_castling_moves() } else { EMPTY },
                        _ => unreachable!(),
                    } & !allied_pieces & mask;

                    generated_moves
                        .into_iter()
                        .for_each(|dest| moves.push(ChessMove::new(src, dest)));
                }
            }

            for src in self
                .pieces_color(Piece::Pawn, self.side_to_move)
                .into_iter()
            {
                let pawn_moves = {
                    if (BitBoard::from_square(src.wrapping_forward(self.side_to_move))
                        & !self.combined())
                        != EMPTY
                    {
                        get_pawn_moves(src, self.side_to_move) & !self.combined()
                    } else {
                        EMPTY
                    }
                } | (get_pawn_attacks(src, self.side_to_move) & opponent_occupancy) & mask;

                pawn_moves.into_iter().for_each(|dest| {
                    if self.is_promotion(&dest) {
                        moves.push(ChessMove::new_promotion(src, dest, Piece::Knight));
                        moves.push(ChessMove::new_promotion(src, dest, Piece::Bishop));
                        moves.push(ChessMove::new_promotion(src, dest, Piece::Rook));
                        moves.push(ChessMove::new_promotion(src, dest, Piece::Queen));
                    } else {
                        moves.push(ChessMove::new(src, dest));
                    }
                });
            }

            if let Some(en_passant) = self.en_passant_square {
                if mask & BitBoard::from_square(en_passant) != EMPTY {
                    for src in get_pawn_attacks(en_passant, !self.side_to_move)
                        & self.pieces_color(Piece::Pawn, self.side_to_move)
                    {
                        moves.push(ChessMove::new(src, en_passant));
                    }
                }
            }
        } else {
            for src in self
                .pieces_color(Piece::King, self.side_to_move)
                .into_iter()
            {
                let generated_moves = get_king_moves(src) & !allied_pieces & mask;

                generated_moves
                    .into_iter()
                    .for_each(|dest| moves.push(ChessMove::new(src, dest)));
            }
        }

        moves
    }

    /// Generate all moves for ray pieces.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, bitboard::EMPTY};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.generate_ray_moves(), EMPTY);
    /// ```
    pub fn generate_ray_moves(&self) -> BitBoard {
        self.generate_bishop_moves() | self.generate_rook_moves() | self.generate_queen_moves()
    }

    /// Get attackers for a given square.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, bitboard::{BitBoard, EMPTY}, chess_move::ChessMove, square::Square};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.get_attackers(Square::E4), EMPTY);
    /// #
    /// # // let _ = board.make_move_new(&ChessMove::new(Square::E2, Square::E4));
    /// #
    /// # // assert_eq!(board.get_attackers(Square::D5), BitBoard::from_square(Square::E4));
    /// ```
    #[rustfmt::skip]
    #[inline]
    pub fn get_attackers(&self, square: Square) -> BitBoard {
        let mut attackers = BitBoard::default();
        let combined = self.combined();

        let bishops = self.pieces_color(Piece::Bishop, !self.side_to_move)
            | self.pieces_color(Piece::Queen, !self.side_to_move);
        let rooks = self.pieces_color(Piece::Rook, !self.side_to_move)
            | self.pieces_color(Piece::Queen, !self.side_to_move);

        attackers |= get_pawn_attacks(square, self.side_to_move) & self.pieces_color(Piece::Pawn, !self.side_to_move);
        attackers |= get_knight_moves(square) & self.pieces_color(Piece::Knight, !self.side_to_move);
        attackers |= get_bishop_moves(square, combined) & bishops;
        attackers |= get_rook_moves(square, combined) & rooks;
        attackers |= get_king_moves(square) & self.pieces_color(Piece::King, !self.side_to_move);

        attackers
    }

    /// Generate all pawn moves.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, bitboard::BitBoard};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.generate_pawn_moves(), BitBoard(0xFFFF0000))
    /// ```
    pub fn generate_pawn_moves(&self) -> BitBoard {
        let mut moves = BitBoard::default();

        for square in self.pieces_color(Piece::Pawn, self.side_to_move) {
            if (BitBoard::from_square(square.wrapping_forward(self.side_to_move))
                & !self.combined())
                != EMPTY
            {
                moves |= get_pawn_moves(square, self.side_to_move) & !self.combined();
            }

            moves |=
                get_pawn_attacks(square, self.side_to_move) & self.occupancy(!self.side_to_move);
        }

        moves | self.generate_en_passant()
    }

    /// Generate all en passants.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, bitboard::EMPTY};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.generate_en_passant(), EMPTY);
    /// ```
    #[inline]
    pub fn generate_en_passant(&self) -> BitBoard {
        if let Some(en_passant) = self.en_passant_square {
            if (get_pawn_attacks(en_passant, !self.side_to_move)
                & self.pieces_color(Piece::Pawn, self.side_to_move))
                != EMPTY
            {
                return BitBoard::from_square(en_passant);
            }
        }

        EMPTY
    }

    /// Check if a square is a promotion square.
    #[inline]
    pub fn is_promotion(&self, square: &Square) -> bool {
        match self.side_to_move {
            Color::White => square.rank() == Rank::Eighth,
            Color::Black => square.rank() == Rank::First,
        }
    }

    /// Generate all knight moves.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, bitboard::BitBoard};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.generate_knight_moves(), BitBoard(0x00A50000))
    /// ```
    pub fn generate_knight_moves(&self) -> BitBoard {
        let mut moves = BitBoard::default();

        for square in self.pieces_color(Piece::Knight, self.side_to_move) {
            moves |= get_knight_moves(square);
        }

        moves & !self.occupancy(self.side_to_move)
    }

    /// Generate all bishop moves.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, bitboard::EMPTY};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.generate_bishop_moves(), EMPTY)
    /// ```
    pub fn generate_bishop_moves(&self) -> BitBoard {
        let occupancy = self.combined();

        let mut moves = BitBoard::default();

        for square in self
            .pieces_color(Piece::Bishop, self.side_to_move)
            .into_iter()
        {
            moves |= get_bishop_moves(square, occupancy);
        }

        moves & !self.occupancy(self.side_to_move)
    }

    /// Generate all rook moves.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, bitboard::EMPTY};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.generate_rook_moves(), EMPTY)
    /// ```
    pub fn generate_rook_moves(&self) -> BitBoard {
        let occupancy = self.combined();

        let mut moves = BitBoard::default();

        for square in self
            .pieces_color(Piece::Rook, self.side_to_move)
            .into_iter()
        {
            moves |= get_rook_moves(square, occupancy);
        }

        moves & !self.occupancy(self.side_to_move)
    }

    /// Generate all queen moves.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, bitboard::EMPTY};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.generate_queen_moves(), EMPTY)
    /// ```
    pub fn generate_queen_moves(&self) -> BitBoard {
        let occupancy = self.combined();

        let mut moves = BitBoard::default();

        for square in self
            .pieces_color(Piece::Queen, self.side_to_move)
            .into_iter()
        {
            moves |= get_bishop_moves(square, occupancy);
            moves |= get_rook_moves(square, occupancy);
        }

        moves & !self.occupancy(self.side_to_move)
    }

    /// Generate all king moves except castling moves.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, bitboard::EMPTY};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.generate_king_moves(), EMPTY)
    /// ```
    pub fn generate_king_moves(&self) -> BitBoard {
        let mut moves = BitBoard::default();

        for square in self.pieces_color(Piece::King, self.side_to_move) {
            moves |= get_king_moves(square);
        }

        moves & !self.occupancy(self.side_to_move)
    }

    /// Generate all castling moves.
    ///
    /// # Example
    /// ```
    /// use chessframe::{board::Board, bitboard::EMPTY};
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board.generate_castling_moves(), EMPTY)
    /// ```
    pub fn generate_castling_moves(&self) -> BitBoard {
        const WHITE_KING_SIDE: BitBoard = BitBoard(0x60);
        const WHITE_QUEEN_SIDE: BitBoard = BitBoard(0x0E);
        const WHITE_QUEEN_SIDE_ATTACKS: BitBoard = BitBoard(0x0C);
        const BLACK_KING_SIDE: BitBoard = BitBoard(0x6000000000000000);
        const BLACK_QUEEN_SIDE: BitBoard = BitBoard(0x0E00000000000000);
        const BLACK_QUEEN_SIDE_ATTACKS: BitBoard = BitBoard(0x0C00000000000000);

        let (king_side, queen_side, queen_side_attacks) = match self.side_to_move {
            Color::White => (WHITE_KING_SIDE, WHITE_QUEEN_SIDE, WHITE_QUEEN_SIDE_ATTACKS),
            Color::Black => (BLACK_KING_SIDE, BLACK_QUEEN_SIDE, BLACK_QUEEN_SIDE_ATTACKS),
        };

        let combined = self.combined();

        let empty_kingside = (combined & king_side) == EMPTY;
        let empty_queenside = (combined & queen_side) == EMPTY;

        if !empty_kingside && !empty_queenside {
            return EMPTY;
        }

        if self.check != 0 {
            return EMPTY;
        }

        let attackers_kingside = king_side
            .into_iter()
            .any(|square| self.get_attackers(square) != EMPTY);
        let attackers_queenside = queen_side_attacks
            .into_iter()
            .any(|square| self.get_attackers(square) != EMPTY);

        let can_castle_kingside = empty_kingside
            && !attackers_kingside
            && self.castling_rights.can_castle(&self.side_to_move, true);
        let can_castle_queenside = empty_queenside
            && !attackers_queenside
            && self.castling_rights.can_castle(&self.side_to_move, false);

        let mut moves = BitBoard::default();

        if can_castle_kingside {
            moves |= BitBoard::set(self.side_to_move.to_backrank(), File::G);
        }

        if can_castle_queenside {
            moves |= BitBoard::set(self.side_to_move.to_backrank(), File::C);
        }

        moves
    }
}
