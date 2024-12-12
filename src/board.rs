use std::str::FromStr;

use crate::{
    bitboard::{BitBoard, EMPTY},
    castling_rights::CastlingRights,
    chess_move::{ChessMove, MoveType},
    color::Color,
    file::File,
    magic::*,
    piece::Piece,
    rank::Rank,
    square::Square,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
    pub pieces: [BitBoard; 6],    // 6 for white, 6 for black
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
            pieces: [BitBoard::default(); 6],
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

    /// Get the bitboard of a particular piece type.
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

    pub fn remove_castling_rights(&mut self, castling_rights: CastlingRights) {
        self.castling_rights = self.castling_rights.remove(castling_rights);
    }

    fn xor(&mut self, bitboard: BitBoard, piece: Piece, color: Color) {
        *self.pieces_mut(piece) ^= bitboard;
        *self.occupancy_mut(color) ^= bitboard;
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

        let pieces = self.pieces(piece);

        *self.pieces_mut(piece) = BitBoard::from_square(*from);

        let piece_moves = match piece {
            Piece::Pawn => self.generate_pawn_moves(),
            Piece::Knight => self.generate_knight_moves(),
            Piece::Bishop => self.generate_bishop_moves(),
            Piece::Rook => self.generate_rook_moves(),
            Piece::Queen => self.generate_queen_moves(),
            Piece::King => self.generate_king_moves() | self.generate_castling_moves(),
        };

        *self.pieces_mut(piece) = pieces;

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

        let from_bitboard = BitBoard::from_square(*from);
        let to_bitboard = BitBoard::from_square(*to);

        let piece = self.get_piece(*from).ok_or("No piece found on square!")?;

        let move_piece = |board: &mut Board, piece: Piece, from: BitBoard, to: BitBoard| {
            board.xor(from, piece, board.side_to_move);
            board.xor(to, piece, board.side_to_move);
        };

        if let Some(captured) = self.get_piece(*to) {
            self.xor(to_bitboard, captured, !self.side_to_move);
        }
        move_piece(self, piece, from_bitboard, to_bitboard);

        self.remove_castling_rights(CastlingRights::square_to_castle_rights(
            &!self.side_to_move,
            *to,
        ));

        self.remove_castling_rights(CastlingRights::square_to_castle_rights(
            &self.side_to_move,
            *from,
        ));

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

        if let Piece::Pawn = piece {
            if let Some(promotion) = mv.get_promotion() {
                self.xor(BitBoard::from_square(*to), Piece::Pawn, self.side_to_move);
                self.xor(BitBoard::from_square(*to), promotion, self.side_to_move);
            } else if Some(*to) == self.en_passant_square() {
                let side_to_move = self.side_to_move;
                self.xor(
                    BitBoard::from_square(to.wrapping_backwards(&side_to_move)),
                    Piece::Pawn,
                    !side_to_move,
                );
            }
        }

        if let MoveType::Castle = move_type {
            let index = to.get_file().to_index();
            let start = BitBoard::set(self.side_to_move.to_backrank(), unsafe {
                *CASTLE_ROOK_START.get_unchecked(index)
            });
            let end = BitBoard::set(self.side_to_move.to_backrank(), unsafe {
                *CASTLE_ROOK_END.get_unchecked(index)
            });

            move_piece(self, Piece::Rook, start, end);
        }

        self.remove_en_passant();

        if let Piece::Pawn = piece {
            if from.get_rank() == self.side_to_move.to_second_rank()
                && to.get_rank() == self.side_to_move.to_fourth_rank()
            {
                self.set_en_passant(to.wrapping_backwards(&self.side_to_move));
            }
        }

        self.side_to_move.flip();

        if (self.generate_moves() & self.pieces_color(Piece::King, !self.side_to_move))
            .is_not_zero()
        {
            return Err("Cannot move pinned piece!".to_string());
        }

        Ok(())
    }

    /// Get the piece at a given square.
    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        let bitboard = BitBoard::from_square(square);
        if self.combined() & bitboard == EMPTY {
            None
        } else if (self.pieces(Piece::Pawn)
            | self.pieces(Piece::Knight)
            | self.pieces(Piece::Bishop))
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

    /// Set the piece at a given square (used during board construction and promotions).
    pub fn set_piece(&mut self, piece: Piece, color: Color, square: Square) {
        let bitboard = self.pieces_mut(piece);
        bitboard.set_bit(square);
        self.occupancy_mut(color).set_bit(square);
    }

    /// Remove the piece at a given square.
    pub fn clear_piece(&mut self, square: Square, color: Color) {
        if self.occupancy(color).is_not_set(square) {
            return;
        }

        for i in 0..6 {
            if (self.pieces[i] & self.occupancy(color)).is_set(square) {
                self.pieces[i].clear_bit(square);
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
                    let allied_pieces = self.occupancy(self.side_to_move);
                    let opponent_occupancy = self.occupancy(!self.side_to_move);
                    let blockers = allied_pieces | opponent_occupancy;

                    $(
                        for src in self.pieces_color($piece, $color).into_iter() {
                            let generated_moves = match $piece {
                                Piece::Knight => get_knight_moves(src),
                                Piece::Bishop => get_bishop_moves(src, blockers),
                                Piece::Rook => get_rook_moves(src, blockers),
                                Piece::Queen => get_bishop_moves(src, blockers) | get_rook_moves(src, blockers),
                                Piece::King => get_king_moves(src),
                                _ => BitBoard(0),
                            } & !allied_pieces;

                            let quiet_moves = generated_moves & !opponent_occupancy;
                            let capture_moves = generated_moves & opponent_occupancy;

                            // Extend moves with quiet moves
                            moves.extend(quiet_moves.into_iter().map(|dest| ChessMove::new(src, dest)));

                            // Extend moves with capture moves
                            moves.extend(capture_moves.into_iter().map(|dest| ChessMove::new_capture(src, dest)));

                            match $piece {
                                Piece::Pawn => {
                                    let pawn_quiets = {
                                        if (BitBoard::from_square(src.wrapping_forward(&self.side_to_move)) & !self.combined()) != EMPTY {
                                            get_pawn_moves(src, self.side_to_move) & !self.combined()
                                        } else {
                                            BitBoard::default()
                                        }
                                    };
                                    let pawn_captures = get_pawn_attacks(src, self.side_to_move) & opponent_occupancy;

                                    pawn_quiets.into_iter().for_each(|dest| {
                                        if self.is_promotion(&dest) {
                                            moves.push(ChessMove::new_promotion(src, dest, Piece::Knight));
                                            moves.push(ChessMove::new_promotion(src, dest, Piece::Bishop));
                                            moves.push(ChessMove::new_promotion(src, dest, Piece::Rook));
                                            moves.push(ChessMove::new_promotion(src, dest, Piece::Queen));
                                        } else {
                                            moves.push(ChessMove::new(src, dest));
                                        }
                                    });
                                    pawn_captures.into_iter().for_each(|dest| {
                                        if self.is_promotion(&dest) {
                                            moves.push(ChessMove::new_capture_promotion(src, dest, Piece::Knight));
                                            moves.push(ChessMove::new_capture_promotion(src, dest, Piece::Bishop));
                                            moves.push(ChessMove::new_capture_promotion(src, dest, Piece::Rook));
                                            moves.push(ChessMove::new_capture_promotion(src, dest, Piece::Queen));
                                        } else {
                                            moves.push(ChessMove::new_capture(src, dest));
                                        }
                                    });
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

                        if let Piece::Pawn = $piece {
                            if let Some(en_passant) = self.en_passant_square {
                                for src in get_pawn_attacks(en_passant, !self.side_to_move) & self.pieces_color(Piece::Pawn, self.side_to_move) {
                                    moves.push(ChessMove::new_en_passant(src, en_passant));
                                }
                            }
                        }
                    )+

                    moves
                }
            };
        }

        extract_moves!(
            self.side_to_move,
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King
        )
    }

    /// Generate all moves for ray pieces.
    pub fn generate_ray_moves(&self) -> BitBoard {
        self.generate_bishop_moves() | self.generate_rook_moves() | self.generate_queen_moves()
    }

    /// Generate all pawn moves.
    pub fn generate_pawn_moves(&self) -> BitBoard {
        let mut moves = BitBoard(0);

        // let en_passant_square = self.en_passant_square().unwrap_or_default();

        for square in self.pieces_color(Piece::Pawn, self.side_to_move) {
            if (BitBoard::from_square(square.wrapping_forward(&self.side_to_move))
                & !self.combined())
                != EMPTY
            {
                moves |= get_pawn_moves(square, self.side_to_move) & !self.combined();
            }

            moves |=
                get_pawn_attacks(square, self.side_to_move) & self.occupancy(!self.side_to_move);
            // moves |= get_pawn_attacks(square.to_index(), self.side_to_move) & en_passant_square;
        }

        moves
    }

    /// Generate all en passants.
    pub fn generate_en_passant(&self) -> BitBoard {
        if let Some(en_passant) = self.en_passant_square {
            if (get_pawn_attacks(en_passant, !self.side_to_move)
                & self.pieces_color(Piece::Pawn, self.side_to_move))
                != EMPTY
            {
                return BitBoard::from_square(en_passant);
            }
        }

        BitBoard(0)
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
        let mut moves = BitBoard(0);

        for square in self.pieces_color(Piece::Knight, self.side_to_move) {
            moves |= get_knight_moves(square);
        }

        moves & !self.occupancy(self.side_to_move)
    }

    /// Generate all bishop moves.
    pub fn generate_bishop_moves(&self) -> BitBoard {
        let occupancy = self.combined();

        let mut moves = BitBoard(0);

        for square in self
            .pieces_color(Piece::Bishop, self.side_to_move)
            .into_iter()
        {
            moves |= get_bishop_moves(square, occupancy);
        }

        moves & !self.occupancy(self.side_to_move)
    }

    /// Generate all rook moves.
    pub fn generate_rook_moves(&self) -> BitBoard {
        let occupancy = self.combined();

        let mut moves = BitBoard(0);

        for square in self
            .pieces_color(Piece::Rook, self.side_to_move)
            .into_iter()
        {
            moves |= get_rook_moves(square, occupancy);
        }

        moves & !self.occupancy(self.side_to_move)
    }

    /// Generate all queen moves.
    pub fn generate_queen_moves(&self) -> BitBoard {
        let occupancy = self.combined();

        let mut moves = BitBoard(0);

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
    pub fn generate_king_moves(&self) -> BitBoard {
        let allied_pieces = self.occupancy(self.side_to_move);
        let kings = self.pieces_color(Piece::King, self.side_to_move);

        let mut moves = BitBoard(0);

        for square in kings {
            moves |= get_king_moves(square);
        }

        moves &= !allied_pieces;

        moves
    }

    /// Generate all castling moves.
    pub fn generate_castling_moves(&mut self) -> BitBoard {
        let occupancy = self.combined();

        let king = self.pieces_color(Piece::King, self.side_to_move);
        let (king_side, queen_side) = match self.side_to_move {
            Color::White => (BitBoard(0x60), BitBoard(0x0e)),
            Color::Black => (BitBoard(0x6000000000000000), BitBoard(0x0e00000000000000)),
        };
        let mut moves = BitBoard(0);

        self.side_to_move.flip();

        let enemy_moves = self.generate_moves();

        self.side_to_move.flip();

        if self.castling_rights.can_castle(&self.side_to_move, true)
            && (occupancy & king_side).is_zero()
            && (enemy_moves & (king_side | king)).is_zero()
        {
            moves |= BitBoard::set(self.side_to_move.to_backrank(), File::G);
        }

        if self.castling_rights.can_castle(&self.side_to_move, false)
            && (occupancy & queen_side).is_zero()
            && (enemy_moves & (queen_side | king)).is_zero()
        {
            moves |= BitBoard::set(self.side_to_move.to_backrank(), File::C);
        }

        moves
    }
}
