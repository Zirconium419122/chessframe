use crate::{
    bitboard::BitBoard,
    castling_rights::CastlingRights,
    color::Color,
    magic::{get_bishop_moves, get_rook_moves},
    piece::Piece,
    r#move::{from_algebraic, BoardHistory, Move, MoveType},
};

#[derive(Clone)]
pub struct Board {
    pub pieces: [BitBoard; 12],   // 6 for white, 6 for black
    pub occupancy: [BitBoard; 2], // white, black occupancy
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<BitBoard>,
    pub half_move_clock: u32,
    pub full_move_clock: u32,
    pub board_history: Vec<BoardHistory>,
}

impl Default for Board {
    fn default() -> Self {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

impl Board {
    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board {
            pieces: [BitBoard::default(); 12],
            occupancy: [BitBoard::default(); 2],
            side_to_move: Color::White,
            castling_rights: CastlingRights::default(),
            en_passant_square: None,
            half_move_clock: 0,
            full_move_clock: 1,
            board_history: Vec::new(),
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
        let square = 8 * rank + file;
        self.set_piece(&piece, &color, square);
    }

    fn parse_en_passant(&mut self, en_passant: &str) {
        if en_passant != "-" {
            let file = en_passant.chars().nth(0).unwrap() as usize - 'a' as usize;
            let rank = en_passant.chars().nth(1).unwrap().to_digit(10).unwrap() as usize - 1;
            let square = 8 * rank + file;

            self.en_passant_square = Some(BitBoard(1 << square));
        }
    }

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

                Ok(())
            }
            Color::Black => {
                if kingside && (castling_moves & BitBoard(0x4000000000000000)).is_zero() {
                    return Err("Cannot castle kingside");
                }

                if !kingside && (castling_moves & BitBoard(0x0400000000000000)).is_zero() {
                    return Err("Cannot castle queenside");
                }

                Ok(())
            }
        }
    }

    pub fn infer_move(&mut self, mv: &str) -> Result<Move, &str> {
        let from = from_algebraic(&mv[0..2])?;
        let to = from_algebraic(&mv[2..4])?;
        let promotion: Option<Piece> = match &mv.len() {
            4 => None,
            5 => Some(Piece::from(mv.chars().last().unwrap())),
            _ => return Err("Invalid move notation!"),
        };

        if let Some(piece) = self.get_piece(from) {
            let mut mv: Option<Move> = None;

            match piece {
                Piece::Pawn => {
                    if let Some(promotion) = promotion {
                        if self.occupancy[(!self.side_to_move).to_index()].is_not_set(to) {
                            mv = Some(Move::new_promotion(from, to, promotion));
                        } else {
                            mv = Some(Move::new_capture_promotion(from, to, promotion));
                        }
                    }
                }
                Piece::King => {
                    let king = match self.side_to_move {
                        Color::White => 4,
                        Color::Black => 60,
                    };

                    if from == king && (to == king + 2 || to == king - 2) {
                        mv = Some(Move::new_castle(from, to));
                    }
                }
                _ => (),
            }

            if let Some(mv) = mv {
                if self.validate_move(&mv).is_ok() {
                    return Ok(mv);
                }
            }

            if self.occupancy[(!self.side_to_move).to_index()].is_not_set(to) {
                mv = Some(Move::new(from, to));
            } else {
                mv = Some(Move::new_capture(from, to));
            }

            if let Some(mv) = mv {
                if self.validate_move(&mv).is_ok() {
                    return Ok(mv);
                }
            }
        }

        Err("No piece at from square")
    }

    pub fn validate_move(&mut self, mv: &Move) -> Result<Piece, &str> {
        let (from, to) = mv.get_move();
        let piece = self.get_piece(from).ok_or("No piece found on square!")?;

        let index = piece.to_index() + self.side_to_move.to_offset();
        let pieces = self.pieces[index];

        self.pieces[index] = BitBoard(1 << from);

        let piece_moves = match piece {
            Piece::Pawn => self.generate_pawn_moves(),
            Piece::Knight => self.generate_knight_moves(),
            Piece::Bishop => self.generate_bishop_moves(),
            Piece::Rook => self.generate_rook_moves(),
            Piece::Queen => self.generate_queen_moves(),
            Piece::King => self.generate_king_moves() | self.generate_castling_moves(),
        };

        self.pieces[index] = pieces;

        if piece_moves.is_not_set(to) {
            return Err("Invalid move!");
        }

        Ok(piece)
    }

    pub fn make_move(&mut self, mv: &Move) -> Result<(), String> {
        let piece = self.validate_move(mv)?;

        let (from, to) = mv.get_move();
        let move_type = mv.get_move_type();

        let offset = self.side_to_move.to_offset();
        let index = piece.to_index() + offset;

        self.board_history.push(BoardHistory::from(&*self));

        let mut move_piece = |index: usize, from: usize, to: usize| {
            self.pieces[index].clear_bit(from);
            self.pieces[index].set_bit(to);
        };

        match move_type {
            MoveType::Quiet | MoveType::Capture | MoveType::Check => {
                move_piece(index, from, to);
                self.clear_piece(to, &!self.side_to_move);
            }
            MoveType::Castle => {
                let (kingside, queenside) = match self.side_to_move {
                    Color::White => (6, 2),
                    Color::Black => (62, 58),
                };

                if to == kingside {
                    move_piece(index, from, to);
                    move_piece(3, kingside + 1, kingside - 1);

                    self.castling_rights.revoke_all(&self.side_to_move);
                } else if to == queenside {
                    move_piece(index, from, to);
                    move_piece(3, queenside - 2, queenside + 1);

                    self.castling_rights.revoke_all(&self.side_to_move);
                }
            }
            MoveType::EnPassant => {
                let behind_pawn = match self.side_to_move {
                    Color::White => to - 8,
                    Color::Black => to + 8,
                };

                move_piece(index, from, to);
                self.clear_piece(behind_pawn, &!self.side_to_move)
            }
            MoveType::Promotion(piece) => {
                self.pieces[offset].clear_bit(from);
                self.set_piece(piece, &self.side_to_move.clone(), to)
            }
            MoveType::CapturePromotion(piece) => {
                self.pieces[offset].clear_bit(from);
                self.set_piece(piece, &self.side_to_move.clone(), to);
                self.clear_piece(to, &!self.side_to_move);
            }
        }

        self.en_passant_square = None;

        match piece {
            Piece::Pawn => {
                let (square_behind_pawn, two_squares_behind_pawn) = match self.side_to_move {
                    Color::White => (to - 8, to - 16),
                    Color::Black => (to + 8, to + 16),
                };

                if from == two_squares_behind_pawn {
                    self.en_passant_square = Some(BitBoard(1 << square_behind_pawn));
                }
            }
            Piece::King => {
                self.castling_rights.revoke_all(&self.side_to_move);
            }
            Piece::Rook => {
                let (kingside_rook, queenside_rook) = match self.side_to_move {
                    Color::White => (7, 0),
                    Color::Black => (63, 56),
                };

                if from == kingside_rook {
                    self.castling_rights.revoke(&self.side_to_move, true);
                } else if from == queenside_rook {
                    self.castling_rights.revoke(&self.side_to_move, false);
                }
            }
            _ => (),
        }

        self.update_occupancy();

        self.side_to_move.flip();

        if (self.generate_moves() & self.pieces[5 + offset]).is_not_zero() {
            self.unmake_move().unwrap();

            return Err("Cannot move pinned piece!".to_string());
        }

        Ok(())
    }

    pub fn unmake_move(&mut self) -> Result<(), &str> {
        match self.board_history.pop() {
            Some(board_history) => {
                self.pieces = board_history.pieces;
                self.occupancy = board_history.occupancy;
                self.side_to_move.flip();
                self.castling_rights = board_history.castling_rights;
                self.en_passant_square = board_history.en_passant_square;
                self.half_move_clock = board_history.half_move_clock;
                self.full_move_clock -= 1;

                Ok(())
            }
            None => Err("No move to unmake!"),
        }
    }

    fn update_occupancy(&mut self) {
        match self.side_to_move {
            Color::White => {
                self.occupancy[0] = self.pieces[0]
                    | self.pieces[1]
                    | self.pieces[2]
                    | self.pieces[3]
                    | self.pieces[4]
                    | self.pieces[5]
            }
            Color::Black => {
                self.occupancy[1] = self.pieces[6]
                    | self.pieces[7]
                    | self.pieces[8]
                    | self.pieces[9]
                    | self.pieces[10]
                    | self.pieces[11]
            }
        }
    }

    pub fn get_piece(&self, square: usize) -> Option<Piece> {
        if (self.occupancy[0] | self.occupancy[1]).is_not_set(square) {
            return None;
        }

        let offset = self.side_to_move.to_offset();

        if (self.pieces[Piece::Pawn.to_index() + offset]
            ^ self.pieces[Piece::Knight.to_index() + offset]
            ^ self.pieces[Piece::Bishop.to_index() + offset])
            .is_set(square)
        {
            if self.pieces[Piece::Pawn.to_index() + offset].is_set(square) {
                Some(Piece::Pawn)
            } else if self.pieces[Piece::Knight.to_index() + offset].is_set(square) {
                Some(Piece::Knight)
            } else {
                Some(Piece::Bishop)
            }
        } else if self.pieces[Piece::Rook.to_index() + offset].is_set(square) {
            Some(Piece::Rook)
        } else if self.pieces[Piece::Queen.to_index() + offset].is_set(square) {
            Some(Piece::Queen)
        } else {
            Some(Piece::King)
        }
    }

    pub fn set_piece(&mut self, piece: &Piece, color: &Color, square: usize) {
        let bitboard = &mut self.pieces[piece.piece_index(color)];
        bitboard.set_bit(square);
        self.occupancy[color.to_index()].set_bit(square);
    }

    pub fn clear_piece(&mut self, square: usize, color: &Color) {
        let color_index = color.to_index();
        if self.occupancy[color_index].is_not_set(square) {
            return;
        }

        let offset = color.to_offset();

        for i in 0..6 {
            if self.pieces[i + offset].is_set(square) {
                self.pieces[i + offset].clear_bit(square);
                break;
            }
        }

        self.occupancy[color_index].clear_bit(square);
    }

    pub fn generate_moves(&self) -> BitBoard {
        self.generate_pawn_moves()
            | self.generate_knight_moves()
            | self.generate_bishop_moves()
            | self.generate_rook_moves()
            | self.generate_queen_moves()
            | self.generate_king_moves()
    }

    pub fn generate_moves_vec(&mut self) -> Vec<Move> {
        macro_rules! extract_moves {
            ($offset:literal, $($piece:expr),+) => {
                {
                    let mut moves: Vec<Move> = Vec::with_capacity(218);
                    let opponent_occupancy = self.occupancy[(!self.side_to_move).to_index()];

                    $(
                        let pieces = self.pieces[$piece as usize + $offset];

                        for square in pieces.into_iter() {
                            self.pieces[$piece as usize + $offset] = BitBoard(1 << square);

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
                            moves.extend(quiet_moves.into_iter().map(|destination| Move::new(square, destination)));

                            // Extend moves with capture moves
                            moves.extend(capture_moves.into_iter().map(|destination| Move::new_capture(square, destination)));

                            match $piece {
                                Piece::Pawn => {
                                    moves.extend(self.generate_pawn_pushes().into_iter().flat_map(|destination| {
                                        if self.is_promotion(destination) {
                                            vec![
                                                Move::new_promotion(square, destination, Piece::Knight),
                                                Move::new_promotion(square, destination, Piece::Bishop),
                                                Move::new_promotion(square, destination, Piece::Rook),
                                                Move::new_promotion(square, destination, Piece::Queen),
                                            ]
                                        } else {
                                            vec![
                                                Move::new(square, destination)
                                            ]
                                        }
                                    }));

                                    moves.extend(self.generate_pawn_captures().into_iter().flat_map(|destination| {
                                        if self.is_promotion(destination) {
                                            vec![
                                                Move::new_capture_promotion(square, destination, Piece::Knight),
                                                Move::new_capture_promotion(square, destination, Piece::Bishop),
                                                Move::new_capture_promotion(square, destination, Piece::Rook),
                                                Move::new_capture_promotion(square, destination, Piece::Queen),
                                            ]
                                        } else {
                                            vec![
                                                Move::new_capture(square, destination),
                                            ]
                                        }
                                    }));

                                    moves.extend(self.generate_en_passant().into_iter().map(|destination| {
                                            Move::new_en_passant(square, destination)
                                        })
                                    );
                                }
                                Piece::King => {
                                    moves.extend(self.generate_castling_moves().into_iter().map(|destination| {
                                            Move::new_castle(square, destination)
                                        })
                                    );
                                }
                                _ => (),
                            }

                        }

                        self.pieces[$piece as usize + $offset] = pieces;
                    )+

                    moves
                }
            };
        }

        match self.side_to_move {
            Color::White => extract_moves!(
                0,
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King
            ),
            Color::Black => extract_moves!(
                6,
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King
            ),
        }
    }

    pub fn generate_ray_moves(&self) -> BitBoard {
        self.generate_bishop_moves() | self.generate_rook_moves() | self.generate_queen_moves()
    }

    pub fn generate_pawn_moves(&self) -> BitBoard {
        self.generate_pawn_pushes() | self.generate_pawn_captures() | self.generate_en_passant()
    }

    pub fn generate_pawn_pushes(&self) -> BitBoard {
        let empty_squares = !(self.occupancy[0] | self.occupancy[1]);

        match self.side_to_move {
            Color::White => {
                let single_push = self.pieces[0] << 8;

                let second_rank = BitBoard(0x000000000000FF00);
                let double_push = ((self.pieces[0] & second_rank) << 16) & (empty_squares << 8);

                (single_push | double_push) & empty_squares
            }
            Color::Black => {
                let single_push = self.pieces[6] >> 8;

                let seventh_rank = BitBoard(0x00FF000000000000);
                let double_push = ((self.pieces[6] & seventh_rank) >> 16) & (empty_squares >> 8);

                (single_push | double_push) & empty_squares
            }
        }
    }

    pub fn generate_pawn_captures(&self) -> BitBoard {
        let opponents_pieces = match self.side_to_move {
            Color::White => self.occupancy[1],
            Color::Black => self.occupancy[0],
        };

        match self.side_to_move {
            Color::White => {
                let northwest_capture =
                    (self.pieces[0] << 7) & opponents_pieces & !BitBoard(0x8080808080808080); // Mask out the H file

                let northeast_capture =
                    (self.pieces[0] << 9) & opponents_pieces & !BitBoard(0x0101010101010101); // Mask out the A file

                northwest_capture | northeast_capture
            }
            Color::Black => {
                let southwest_capture =
                    (self.pieces[6] >> 9) & opponents_pieces & !BitBoard(0x8080808080808080); // Mask out the H file

                let southeast_capture =
                    (self.pieces[6] >> 7) & opponents_pieces & !BitBoard(0x0101010101010101); // Mask out the A file

                southwest_capture | southeast_capture
            }
        }
    }

    pub fn generate_en_passant(&self) -> BitBoard {
        if let Some(en_passant) = self.en_passant_square {
            match self.side_to_move {
                Color::White => {
                    let west_ep =
                        (self.pieces[0] << 7) & en_passant & !BitBoard(0x8080808080808080); // Mask out the H file
                    let east_ep =
                        (self.pieces[0] << 9) & en_passant & !BitBoard(0x0101010101010101); // Mask out the A file

                    west_ep | east_ep
                }
                Color::Black => {
                    let west_ep =
                        (self.pieces[6] >> 9) & en_passant & !BitBoard(0x8080808080808080); // Mask out the H file
                    let east_ep =
                        (self.pieces[6] >> 7) & en_passant & !BitBoard(0x0101010101010101); // Mask out the A file

                    west_ep | east_ep
                }
            }
        } else {
            BitBoard(0)
        }
    }

    pub fn is_promotion(&self, square: usize) -> bool {
        match self.side_to_move {
            Color::White => square >= 56, // Rank 8
            Color::Black => square < 8,   // Rank 1
        }
    }

    pub fn generate_knight_moves(&self) -> BitBoard {
        let allied_pieces = match self.side_to_move {
            Color::White => self.occupancy[0],
            Color::Black => self.occupancy[1],
        };

        let knights = match self.side_to_move {
            Color::White => self.pieces[1],
            Color::Black => self.pieces[7],
        };
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

    pub fn generate_bishop_moves(&self) -> BitBoard {
        let occupancy = self.occupancy[0] | self.occupancy[1];

        match self.side_to_move {
            Color::White => {
                let mut moves = BitBoard(0);

                for square in self.pieces[2].into_iter() {
                    moves |= get_bishop_moves(square, occupancy);
                }

                moves & !self.occupancy[0]
            }
            Color::Black => {
                let mut moves = BitBoard(0);

                for square in self.pieces[8].into_iter() {
                    moves |= get_bishop_moves(square, occupancy);
                }

                moves & !self.occupancy[1]
            }
        }
    }

    pub fn generate_rook_moves(&self) -> BitBoard {
        let occupancy = self.occupancy[0] | self.occupancy[1];

        match self.side_to_move {
            Color::White => {
                let mut moves = BitBoard(0);

                for square in self.pieces[3].into_iter() {
                    moves |= get_rook_moves(square, occupancy);
                }

                moves & !self.occupancy[0]
            }
            Color::Black => {
                let mut moves = BitBoard(0);

                for square in self.pieces[9].into_iter() {
                    moves |= get_rook_moves(square, occupancy);
                }

                moves & !self.occupancy[1]
            }
        }
    }

    pub fn generate_queen_moves(&self) -> BitBoard {
        let occupancy = self.occupancy[0] | self.occupancy[1];

        match self.side_to_move {
            Color::White => {
                let mut moves = BitBoard(0);

                for square in self.pieces[4].into_iter() {
                    moves |= get_bishop_moves(square, occupancy);
                    moves |= get_rook_moves(square, occupancy);
                }

                moves & !self.occupancy[0]
            }
            Color::Black => {
                let mut moves = BitBoard(0);

                for square in self.pieces[10].into_iter() {
                    moves |= get_bishop_moves(square, occupancy);
                    moves |= get_rook_moves(square, occupancy);
                }

                moves & !self.occupancy[1]
            }
        }
    }

    pub fn generate_king_moves(&self) -> BitBoard {
        let allied_pieces = match self.side_to_move {
            Color::White => self.occupancy[0],
            Color::Black => self.occupancy[1],
        };

        let kings = match self.side_to_move {
            Color::White => self.pieces[5],
            Color::Black => self.pieces[11],
        };
        let mut moves = BitBoard(0);

        moves |= (kings << 7) & !BitBoard(0x8080808080808080); // Mask out the H file
        moves |= kings << 8;
        moves |= (kings << 9) & !BitBoard(0x0101010101010101); // Mask out the A file
        moves |= (kings << 1) & !BitBoard(0x0101010101010101); // Mask out the A file

        moves |= (kings >> 7) & !BitBoard(0x0101010101010101); // Mask out the A file
        moves |= kings >> 8;
        moves |= (kings >> 9) & !BitBoard(0x8080808080808080); // Mask out the H file
        moves |= (kings >> 1) & !BitBoard(0x8080808080808080); // Mask out the H file

        moves &= !allied_pieces;

        moves
    }

    pub fn generate_castling_moves(&mut self) -> BitBoard {
        let occupancy = self.occupancy[0] | self.occupancy[1];

        let (king_side, queen_side, king) = match self.side_to_move {
            Color::White => (BitBoard(0x60), BitBoard(0x0e), self.pieces[5]),
            Color::Black => (
                BitBoard(0x6000000000000000),
                BitBoard(0x0e00000000000000),
                self.pieces[11],
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
