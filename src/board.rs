use std::str::FromStr;

use crate::{
    bitboard::{BitBoard, EMPTY},
    castling_rights::CastlingRights,
    chess_move::{ChessMove, MoveType},
    color::Color,
    file::File,
    magic::{get_bishop_moves, get_king_moves, get_pawn_attacks, get_pawn_moves, get_rook_moves},
    piece::Piece,
    rank::Rank,
    square::Square,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
    pub pieces: [BitBoard; 12],   // 6 for white, 6 for black
    pub occupancy: [BitBoard; 2], // white, black occupancy
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<Square>,
    pub half_move_clock: u32,
    pub full_move_clock: u32,
}

impl Default for Board {
    fn default() -> Self {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

impl Board {
    /// Create a board from a FEN in the form of a `&str`.
    /// ```
    /// use chess_frame::board::Board;
    ///
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// let board = Board::from_fen(fen);
    ///
    /// assert_eq!(board, Board::default());
    /// ```
    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board {
            pieces: [BitBoard::default(); 12],
            occupancy: [BitBoard::default(); 2],
            side_to_move: Color::White,
            castling_rights: CastlingRights::default(),
            en_passant_square: None,
            half_move_clock: 0,
            full_move_clock: 1,
        };

        let parts: Vec<&str> = fen.split_whitespace().collect();
        assert_eq!(parts.len(), 6);

        board.parse_pieces(parts[0]);

        board.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid active color in FEN"),
        };

        board.castling_rights = CastlingRights::from_fen(parts[2]);

        board.parse_en_passant(parts[3]);

        board.half_move_clock = parts[4].parse().expect("Invalid halfmove clock");

        board.full_move_clock = parts[5].parse().expect("Invalid fullmove clock");

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
            let file = en_passant.chars().nth(0).unwrap() as u8 - b'a';
            let rank = en_passant.chars().nth(1).unwrap().to_digit(10).unwrap() as u8 - 1;
            let square = Square::new(8 * rank + file);

            self.set_en_passant(square);
        }
    }

    /// Get the combined bitboard of all pieces on the board.
    /// ```
    /// use chess_frame::{bitboard::BitBoard, board::Board};
    ///
    /// let board = Board::default();
    ///
    /// assert_eq!(board.combined(), BitBoard(0xFFFF00000000FFFF));
    /// ```
    #[inline]
    pub fn combined(&self) -> BitBoard {
        self.occupancy(Color::White) | self.occupancy(Color::Black)
    }

    /// Get the occupancy bitboard for a particular color.
    /// ```
    /// use chess_frame::{bitboard::BitBoard, board::Board, color::Color};
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

    /// Get the bitboard of a particular piece and color.
    #[inline]
    pub fn pieces(&self, piece: Piece, color: Color) -> BitBoard {
        unsafe {
            *self
                .pieces
                .get_unchecked(piece.to_index() + color.to_offset())
        }
    }

    /// Get a mutable reference to a particular piece and color.
    #[inline]
    pub fn pieces_mut(&mut self, piece: Piece, color: Color) -> &mut BitBoard {
        unsafe {
            self.pieces
                .get_unchecked_mut(piece.to_index() + color.to_offset())
        }
    }

    /// Get the en passant square, if there is one.
    #[inline]
    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    fn remove_en_passant(&mut self) {
        self.en_passant_square = None;
    }

    fn set_en_passant(&mut self, square: Square) {
        self.en_passant_square = Some(square);
    }

    /// Check if one can castle to the given side.
    pub fn can_castle(&mut self, kingside: bool) -> Result<(), &str> {
        let castling_moves = self.generate_castling_moves();

        match self.side_to_move {
            Color::White => {
                if kingside && (castling_moves & BitBoard(0x40)).is_zero() {
                    return Err("Cannot castle kingside");
                }

                if !kingside && (castling_moves & BitBoard(0x04)).is_zero() {
                    return Err("Cannot castle queenside");
                }
            }
            Color::Black => {
                if kingside && (castling_moves & BitBoard(0x4000000000000000)).is_zero() {
                    return Err("Cannot castle kingside");
                }

                if !kingside && (castling_moves & BitBoard(0x0400000000000000)).is_zero() {
                    return Err("Cannot castle queenside");
                }
            }
        }

        Ok(())
    }

    /// Infer a `ChessMove` from a string based on the current `Board`.
    pub fn infer_move(&mut self, mv: &str) -> Result<ChessMove, String> {
        let from = Square::from_str(&mv[0..2]).map_err(|err| err.to_string())?;
        let to = Square::from_str(&mv[2..4]).map_err(|err| err.to_string())?;
        let promotion: Option<Piece> = match &mv.len() {
            4 => None,
            5 => Some(Piece::from(mv.chars().last().unwrap())),
            _ => return Err("Invalid move notation!".to_string()),
        };

        if let Some(piece) = self.get_piece(from) {
            let mut mv: Option<ChessMove> = None;

            match piece {
                Piece::Pawn => {
                    if let Some(promotion) = promotion {
                        if self.occupancy(!self.side_to_move).is_not_set(to) {
                            mv = Some(ChessMove::new_promotion(from, to, promotion));
                        } else {
                            mv = Some(ChessMove::new_capture_promotion(from, to, promotion));
                        }
                    }
                }
                Piece::King => {
                    let king = match self.side_to_move {
                        Color::White => Square::E1,
                        Color::Black => Square::E8,
                    };

                    if from == king
                        && (to == king.wrapping_right().wrapping_right()
                            || to == king.wrapping_left().wrapping_left())
                    {
                        mv = Some(ChessMove::new_castle(from, to));
                    }
                }
                _ => (),
            }

            if let Some(mv) = mv {
                if self.validate_move(&mv).is_ok() {
                    return Ok(mv);
                }
            }

            if self.occupancy(!self.side_to_move).is_not_set(to) {
                mv = Some(ChessMove::new(from, to));
            } else {
                mv = Some(ChessMove::new_capture(from, to));
            }

            if let Some(mv) = mv {
                if self.validate_move(&mv).is_ok() {
                    return Ok(mv);
                }
            }
        }

        Err("No piece at from square".to_string())
    }

    /// Checks that a `ChessMove` is a valid move for the current board state. Does not check if the move leaves the king in check.
    pub fn validate_move(&mut self, mv: &ChessMove) -> Result<Piece, &str> {
        let (from, to) = mv.get_move();
        let piece = self.get_piece(*from).ok_or("No piece found on square!")?;

        let pieces = self.pieces(piece, self.side_to_move);

        *self.pieces_mut(piece, self.side_to_move) = BitBoard::from_square(*from);

        let piece_moves = match piece {
            Piece::Pawn => self.generate_pawn_moves(),
            Piece::Knight => self.generate_knight_moves(),
            Piece::Bishop => self.generate_bishop_moves(),
            Piece::Rook => self.generate_rook_moves(),
            Piece::Queen => self.generate_queen_moves(),
            Piece::King => self.generate_king_moves() | self.generate_castling_moves(),
        };

        *self.pieces_mut(piece, self.side_to_move) = pieces;

        if piece_moves.is_not_set(*to) {
            return Err("Invalid move!");
        }

        Ok(piece)
    }

    /// Make a `ChessMove` on a copy of the current `Board`.
    ///
    /// # Parameters
    /// - `mv`: A reference to a `ChessMove` representing the move to make.
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
    /// use chess_frame::{board::Board, chess_move::ChessMove, square::Square};
    ///
    /// let board = Board::default();
    /// let mv = ChessMove::new(Square::E2, Square::E4);
    ///
    /// assert_eq!(board.make_move_new(&mv), Ok(Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")));
    /// ```
    ///
    /// # Notes
    /// This method assumes all moves are pre-validated (pseudo-legal or legal) or generated by `generate_moves_vec`.
    /// It does not perform a legality check before execution but will enforce
    /// certain rules (e.g., pinned pieces cannot move) during processing.
    pub fn make_move_new(&self, mv: &ChessMove) -> Result<Board, String> {
        let mut board = *self;

        board.make_move(mv)?;

        Ok(board)
    }

    /// Make a `ChessMove` on the current `Board`.
    ///
    /// # Parameters
    /// - `mv`: A reference to a `ChessMove` representing the move to make.
    ///   The move must be either pseudo-legal or fully legal; invalid or unchecked moves
    ///   will result in undefined behavior.
    ///
    /// # Returns
    /// - `Ok(())` if the move is successfully made.
    /// - `Err(String)` if the move is invalid, such as attempting to move a pinned piece.
    ///
    /// # Errors
    /// - Returns an error if the resulting board state places or leaves the king in check.
    /// - Errors when attempting to move a piece from an empty square or with other invalid conditions.
    ///
    /// # Example
    /// ```
    /// use chess_frame::{board::Board, chess_move::ChessMove, square::Square};
    ///
    /// let mut board = Board::default();
    /// let mv = ChessMove::new(Square::E2, Square::E4);
    ///
    /// assert_eq!(board.make_move(&mv), Ok(()));
    /// ```
    ///
    /// # Notes
    /// This method assumes all moves are pre-validated (pseudo-legal or legal) or generated by `generate_moves_vec`.
    /// It does not perform a legality check before execution but will enforce
    /// certain rules (e.g., pinned pieces cannot move) during processing.
    pub fn make_move(&mut self, mv: &ChessMove) -> Result<(), String> {
        let (from, to) = mv.get_move();
        let move_type = mv.get_move_type();

        let piece = self.get_piece(*from).ok_or("No piece found on square!")?;

        let move_piece = |board: &mut Board, piece: Piece, from: Square, to: Square| {
            let pieces_mut = board.pieces_mut(piece, board.side_to_move);
            pieces_mut.clear_bit(from);
            pieces_mut.set_bit(to);

            let occupancy_mut = board.occupancy_mut(board.side_to_move);
            occupancy_mut.clear_bit(from);
            occupancy_mut.set_bit(to);
        };

        move_piece(self, piece, *from, *to);
        self.clear_piece(*to, !self.side_to_move);

        match move_type {
            MoveType::Castle => {
                let (kingside, queenside) = match self.side_to_move {
                    Color::White => (Square::G1, Square::C1),
                    Color::Black => (Square::G8, Square::C8),
                };

                if to == &kingside {
                    move_piece(
                        self,
                        Piece::Rook,
                        kingside.wrapping_right(),
                        kingside.wrapping_left(),
                    );
                } else if to == &queenside {
                    move_piece(
                        self,
                        Piece::Rook,
                        queenside.wrapping_left().wrapping_left(),
                        queenside.wrapping_right(),
                    );
                }
            }
            MoveType::EnPassant => {
                let behind_pawn = to.backwards(&self.side_to_move).unwrap();
                self.clear_piece(behind_pawn, !self.side_to_move)
            }
            MoveType::Promotion(piece) | MoveType::CapturePromotion(piece) => {
                self.pieces_mut(Piece::Pawn, self.side_to_move)
                    .clear_bit(*to);
                self.set_piece(*piece, self.side_to_move, *to);
            }
            _ => {}
        }

        self.remove_en_passant();

        match piece {
            Piece::Pawn => {
                if from.get_rank() == self.side_to_move.to_second_rank()
                    && to.get_rank() == self.side_to_move.to_fourth_rank()
                {
                    self.set_en_passant(to.wrapping_backwards(&self.side_to_move));
                }
            }
            Piece::King => {
                self.castling_rights.revoke_all(&self.side_to_move);
            }
            Piece::Rook => {
                let (kingside_rook, queenside_rook) = match self.side_to_move {
                    Color::White => (Square::H1, Square::A1),
                    Color::Black => (Square::H8, Square::A8),
                };

                if from == &kingside_rook {
                    self.castling_rights.revoke(&self.side_to_move, true);
                } else if from == &queenside_rook {
                    self.castling_rights.revoke(&self.side_to_move, false);
                }
            }
            _ => {}
        }

        self.side_to_move.flip();

        if (self.generate_moves() & self.pieces(Piece::King, !self.side_to_move)).is_not_zero() {
            return Err("Cannot move pinned piece!".to_string());
        }

        Ok(())
    }

    /// Get the piece at a given square.
    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        let bitboard = BitBoard::from_square(square);
        if self.combined() & bitboard == EMPTY {
            None
        } else if (self.pieces(Piece::Pawn, self.side_to_move)
            ^ self.pieces(Piece::Knight, self.side_to_move)
            ^ self.pieces(Piece::Bishop, self.side_to_move))
        .is_set(square)
        {
            if self.pieces(Piece::Pawn, self.side_to_move).is_set(square) {
                Some(Piece::Pawn)
            } else if self.pieces(Piece::Knight, self.side_to_move).is_set(square) {
                Some(Piece::Knight)
            } else {
                Some(Piece::Bishop)
            }
        } else if self.pieces(Piece::Rook, self.side_to_move).is_set(square) {
            Some(Piece::Rook)
        } else if self.pieces(Piece::Queen, self.side_to_move).is_set(square) {
            Some(Piece::Queen)
        } else {
            Some(Piece::King)
        }
    }

    /// Set the piece at a given square (used during board construction and promotions).
    pub fn set_piece(&mut self, piece: Piece, color: Color, square: Square) {
        let bitboard = self.pieces_mut(piece, color);
        bitboard.set_bit(square);
        self.occupancy[color.to_index()].set_bit(square);
    }

    /// Remove the piece at a given square.
    pub fn clear_piece(&mut self, square: Square, color: Color) {
        if self.occupancy(color).is_not_set(square) {
            return;
        }

        let offset = color.to_offset();

        for i in 0..6 {
            if self.pieces[i + offset].is_set(square) {
                self.pieces[i + offset].clear_bit(square);
                break;
            }
        }

        self.occupancy_mut(color).clear_bit(square);
    }

    /// Generate all psuedo-legal moves.
    pub fn generate_moves(&self) -> BitBoard {
        self.generate_pawn_moves()
            | self.generate_knight_moves()
            | self.generate_bishop_moves()
            | self.generate_rook_moves()
            | self.generate_queen_moves()
            | self.generate_king_moves()
    }

    /// Generate a vector of psudo-legal `ChessMoves`'s.
    pub fn generate_moves_vec(&mut self) -> Vec<ChessMove> {
        macro_rules! extract_moves {
            ($color:expr, $($piece:expr),+) => {
                {
                    let mut moves: Vec<ChessMove> = Vec::with_capacity(218);
                    let opponent_occupancy = self.occupancy(!self.side_to_move).clone();

                    $(
                        let pieces = self.pieces($piece, $color);

                        for src in pieces.into_iter() {
                            *self.pieces_mut($piece, $color) = BitBoard::from_square(src);

                            let generated_moves = match $piece {
                                Piece::Knight => self.generate_knight_moves(),
                                Piece::Bishop => self.generate_bishop_moves(),
                                Piece::Rook => self.generate_rook_moves(),
                                Piece::Queen => self.generate_queen_moves(),
                                Piece::King => self.generate_king_moves(),
                                _ => BitBoard(0),
                            };

                            let quiet_moves = generated_moves & !opponent_occupancy;
                            let capture_moves = generated_moves & opponent_occupancy;

                            // Extend moves with quiet moves
                            moves.extend(quiet_moves.into_iter().map(|dest| ChessMove::new(src, dest)));

                            // Extend moves with capture moves
                            moves.extend(capture_moves.into_iter().map(|dest| ChessMove::new_capture(src, dest)));

                            match $piece {
                                Piece::Pawn => {
                                    let pawn_moves = {
                                        if (BitBoard::from_square(src.wrapping_forward(&self.side_to_move)) & !self.combined()) != EMPTY {
                                            (get_pawn_moves(src.to_index(), self.side_to_move) & !self.combined()) | (get_pawn_attacks(src.to_index(), self.side_to_move) & opponent_occupancy)
                                        } else {
                                            get_pawn_attacks(src.to_index(), self.side_to_move) & opponent_occupancy
                                        }
                                    };
                                    
                                    pawn_moves.into_iter().for_each(|dest| {
                                        if self.is_promotion(&dest) {
                                            moves.push(ChessMove::new_capture_promotion(src, dest, Piece::Knight));
                                            moves.push(ChessMove::new_capture_promotion(src, dest, Piece::Bishop));
                                            moves.push(ChessMove::new_capture_promotion(src, dest, Piece::Rook));
                                            moves.push(ChessMove::new_capture_promotion(src, dest, Piece::Queen));
                                        } else {
                                            moves.push(ChessMove::new_capture(src, dest));
                                        }
                                    });

                                    self.generate_en_passant().into_iter().for_each(|dest| {
                                        moves.push(ChessMove::new_en_passant(src, dest));
                                    })
                                }
                                Piece::King => {
                                    moves.extend(self.generate_castling_moves().into_iter().map(|dest| {
                                            ChessMove::new_castle(src, dest)
                                        })
                                    );
                                }
                                _ => {},
                            }

                        }

                        *self.pieces_mut($piece, $color) = pieces;
                    )+

                    moves
                }
            };
        }

        match self.side_to_move {
            Color::White => extract_moves!(
                Color::White,
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King
            ),
            Color::Black => extract_moves!(
                Color::Black,
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King
            ),
        }
    }

    /// Generate all moves for ray pieces.
    pub fn generate_ray_moves(&self) -> BitBoard {
        self.generate_bishop_moves() | self.generate_rook_moves() | self.generate_queen_moves()
    }

    /// Generate all pawn moves.
    pub fn generate_pawn_moves(&self) -> BitBoard {
        let mut moves = BitBoard(0);

        // let en_passant_square = self.en_passant_square().unwrap_or_default();

        for square in self.pieces(Piece::Pawn, self.side_to_move) {
            if (BitBoard::from_square(square.wrapping_forward(&self.side_to_move)) & !self.combined()) != EMPTY {
                moves |= get_pawn_moves(square.to_index(), self.side_to_move) & !self.combined();
            }

            moves |= get_pawn_attacks(square.to_index(), self.side_to_move) & self.occupancy(!self.side_to_move);
            // moves |= get_pawn_attacks(square.to_index(), self.side_to_move) & en_passant_square;
        }

        moves
    }

        /// Generate all en passants.
        pub fn generate_en_passant(&self) -> BitBoard {
            if let Some(en_passant) = self.en_passant_square {
                let mut moves = BitBoard(0);

                for square in self.pieces(Piece::Pawn, self.side_to_move) {
                    moves |= get_pawn_attacks(square.to_index(), self.side_to_move);
                }

                moves & BitBoard::from_square(en_passant)
            } else {
                BitBoard(0)
            }
        }

    /// Check if a square is a promotion square.
    pub fn is_promotion(&self, square: &Square) -> bool {
        match self.side_to_move {
            Color::White => square.get_rank() == Rank::Eighth,
            Color::Black => square.get_rank() == Rank::First,
        }
    }

    /// Generate all knight moves.
    pub fn generate_knight_moves(&self) -> BitBoard {
        let allied_pieces = self.occupancy(self.side_to_move);
        let knights = self.pieces(Piece::Knight, self.side_to_move);

        let mut knight_moves = BitBoard(0);

        knight_moves |= (knights << 17) & !BitBoard(0x0101010101010101); // Mask out the H file
        knight_moves |= (knights << 15) & !BitBoard(0x8080808080808080); // Mask out the A file
        knight_moves |= (knights >> 15) & !BitBoard(0x0101010101010101); // Mask out the H file
        knight_moves |= (knights >> 17) & !BitBoard(0x8080808080808080); // Mask out the A file

        knight_moves |= (knights << 10) & !BitBoard(0x0303030303030303); // Mask out the GH file
        knight_moves |= (knights << 6) & !BitBoard(0xC0C0C0C0C0C0C0C0); // Mask out the AB file
        knight_moves |= (knights >> 6) & !BitBoard(0x0303030303030303); // Mask out the GH file
        knight_moves |= (knights >> 10) & !BitBoard(0xC0C0C0C0C0C0C0C0); // Mask out the AB file

        knight_moves &= !allied_pieces;

        knight_moves
    }

    /// Generate all bishop moves.
    pub fn generate_bishop_moves(&self) -> BitBoard {
        let occupancy = self.combined();

        let mut moves = BitBoard(0);

        for square in self.pieces(Piece::Bishop, self.side_to_move).into_iter() {
            moves |= get_bishop_moves(square.to_index(), occupancy);
        }

        moves & !self.occupancy(self.side_to_move)
    }

    /// Generate all rook moves.
    pub fn generate_rook_moves(&self) -> BitBoard {
        let occupancy = self.combined();

        let mut moves = BitBoard(0);

        for square in self.pieces(Piece::Rook, self.side_to_move).into_iter() {
            moves |= get_rook_moves(square.to_index(), occupancy);
        }

        moves & !self.occupancy(self.side_to_move)
    }

    /// Generate all queen moves.
    pub fn generate_queen_moves(&self) -> BitBoard {
        let occupancy = self.combined();

        let mut moves = BitBoard(0);

        for square in self.pieces(Piece::Queen, self.side_to_move).into_iter() {
            moves |= get_bishop_moves(square.to_index(), occupancy);
            moves |= get_rook_moves(square.to_index(), occupancy);
        }

        moves & !self.occupancy(self.side_to_move)
    }

    /// Generate all king moves except castling moves.
    pub fn generate_king_moves(&self) -> BitBoard {
        let allied_pieces = self.occupancy(self.side_to_move);
        let kings = self.pieces(Piece::King, self.side_to_move);

        let mut moves = BitBoard(0);

        for square in kings {
            moves |= get_king_moves(square.to_index());
        }

        moves &= !allied_pieces;

        moves
    }

    /// Generate all castling moves.
    pub fn generate_castling_moves(&mut self) -> BitBoard {
        let occupancy = self.combined();

        let (king_side, queen_side, king) = match self.side_to_move {
            Color::White => (
                BitBoard(0x60),
                BitBoard(0x0e),
                self.pieces(Piece::King, Color::White),
            ),
            Color::Black => (
                BitBoard(0x6000000000000000),
                BitBoard(0x0e00000000000000),
                self.pieces(Piece::King, Color::Black),
            ),
        };
        let mut moves = BitBoard(0);

        self.side_to_move.flip();

        let enemy_moves = self.generate_moves();

        if self.castling_rights.can_castle(&!self.side_to_move, true)
            && (occupancy & king_side).is_zero()
            && (enemy_moves & (king_side | king)).is_zero()
        {
            moves |= match !self.side_to_move {
                Color::White => BitBoard(0x40),
                Color::Black => BitBoard(0x4000000000000000),
            }
        }

        if self.castling_rights.can_castle(&!self.side_to_move, false)
            && (occupancy & queen_side).is_zero()
            && (enemy_moves & (queen_side | king)).is_zero()
        {
            moves |= match !self.side_to_move {
                Color::White => BitBoard(0x04),
                Color::Black => BitBoard(0x0400000000000000),
            }
        }

        self.side_to_move.flip();

        moves
    }
}
