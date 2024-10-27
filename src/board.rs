use crate::{
    bitboard::BitBoard,
    castling_rights::CastlingRights,
    color::Color,
    magic::{get_bishop_moves, get_rook_moves},
    piece::Piece,
    r#move::{BoardHistory, Move, MoveType},
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
        self.set_piece(piece, color, square);
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

    pub fn validate_move(&mut self, mv: &Move) -> Result<Piece, String> {
        let (from, to) = mv.get_move();
        let move_type = mv.get_move_type();

        let generate_moves = [
            self.generate_pawn_moves(),
            self.generate_knight_moves(),
            self.generate_bishop_moves(),
            self.generate_rook_moves(),
            self.generate_queen_moves(),
            self.generate_king_moves() | self.generate_castling_moves(),
        ];

        let offset = match self.side_to_move {
            Color::White => 0,
            Color::Black => 6,
        };

        if let Some((piece, _)) = self.get_piece(from) {
            if self.occupancy[self.side_to_move.clone() as usize].is_set(to) {
                return Err(format!("Can't move piece to square: {}!", to));
            }

            match move_type {
                MoveType::Quiet => {
                    let moves = generate_moves[piece.clone() as usize];

                    if moves.is_not_set(to) {
                        return Err(format!("Invalid move: {}!", to));
                    }

                    if (moves & self.occupancy[self.side_to_move.toggle() as usize]).is_set(to) {
                        return Err("Move is not a quiet move!".to_string());
                    }
                }
                MoveType::Capture => {
                    let moves = generate_moves[piece.clone() as usize];

                    if moves.is_not_set(to) {
                        return Err(format!("Invalid move: {}!", to));
                    }

                    if (moves & self.occupancy[self.side_to_move.toggle() as usize]).is_not_set(to)
                    {
                        return Err("Move is not a capture!".to_string());
                    }
                }
                MoveType::Castle => {
                    let (king_side, queen_side) = match self.side_to_move {
                        Color::White => (6, 2),
                        Color::Black => (62, 58),
                    };

                    if to == king_side {
                        self.can_castle(true)?
                    } else if to == queen_side {
                        self.can_castle(false)?
                    } else {
                        return Err("Invalid castling move!".to_string());
                    }
                }
                MoveType::EnPassant => {
                    if self.en_passant_square.is_some() {
                        if self.generate_en_passant().is_not_set(to) {
                            return Err(format!("En passant to: {}, is not a legal move!", to));
                        }
                    } else {
                        return Err("No en passant square set!".to_string());
                    }
                }
                MoveType::Promotion(_) => {
                    if self.generate_pawn_pushes().is_not_set(to) {
                        return Err(format!("Cannot promote at: {}!", to));
                    }
                }
                MoveType::CapturePromotion(_) => {
                    if self.generate_pawn_captures().is_not_set(to) {
                        return Err(format!("Cannot capture and promote at: {}!", to));
                    }
                }
                MoveType::Check => {
                    let moves = generate_moves[piece.clone() as usize];

                    if moves.is_not_set(to) {
                        return Err(format!("Cannot move piece to: {}!", to));
                    }

                    if (self.pieces[5 + offset] & moves).is_zero() {
                        return Err(format!("Moving piece to: {}, is not a check!", to));
                    }
                }
            }

            return Ok(piece);
        }

        Err(format!("No piece found on square: {}!", from))
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), String> {
        let (from, to) = mv.get_move();
        let move_type = mv.get_move_type();

        let piece = self.validate_move(&mv)?;

        self.en_passant_square = None;

        let offset = match self.side_to_move {
            Color::White => 0,
            Color::Black => 6,
        };

        match move_type {
            MoveType::Quiet => {
                self.board_history.push(BoardHistory::from(self.clone()));

                self.pieces[piece.clone() as usize + offset].clear_bit(from);
                self.pieces[piece.clone() as usize + offset].set_bit(to)
            }
            MoveType::Capture => {
                self.board_history.push(BoardHistory::from(self.clone()));

                self.pieces[piece.clone() as usize + offset].clear_bit(from);
                self.pieces[piece.clone() as usize + offset].set_bit(to);
                self.clear_piece(to, self.side_to_move.toggle());
            }
            MoveType::Castle => {
                let (kingside, queenside) = match self.side_to_move {
                    Color::White => (6, 2),
                    Color::Black => (62, 58),
                };

                if to == kingside {
                    self.board_history.push(BoardHistory::from(self.clone()));

                    self.pieces[piece.clone() as usize + offset].clear_bit(from);
                    self.pieces[piece.clone() as usize + offset].set_bit(to);

                    self.pieces[3].clear_bit(kingside + 1);
                    self.pieces[3].set_bit(kingside - 1);

                    self.castling_rights.revoke_all(&self.side_to_move);
                } else if to == queenside {
                    self.board_history.push(BoardHistory::from(self.clone()));

                    self.pieces[piece.clone() as usize + offset].clear_bit(from);
                    self.pieces[piece.clone() as usize + offset].set_bit(to);

                    self.pieces[3].clear_bit(queenside - 2);
                    self.pieces[3].set_bit(queenside + 1);

                    self.castling_rights.revoke_all(&self.side_to_move);
                }
            }
            MoveType::EnPassant => {
                let behind_pawn = match self.side_to_move {
                    Color::White => to - 8,
                    Color::Black => to + 8,
                };
                self.board_history.push(BoardHistory::from(self.clone()));

                self.pieces[offset].clear_bit(from);
                self.pieces[offset].set_bit(to);
                self.clear_piece(behind_pawn, self.side_to_move.toggle())
            }
            MoveType::Promotion(piece) => {
                self.board_history.push(BoardHistory::from(self.clone()));

                self.pieces[offset].clear_bit(from);
                self.set_piece(piece.clone(), self.side_to_move.clone(), to)
            }
            MoveType::CapturePromotion(piece) => {
                self.board_history.push(BoardHistory::from(self.clone()));

                self.pieces[offset].clear_bit(from);
                self.set_piece(piece.clone(), self.side_to_move.clone(), to);
                self.clear_piece(to, self.side_to_move.toggle());
            }
            MoveType::Check => {
                self.board_history.push(BoardHistory::from(self.clone()));

                self.pieces[piece.clone() as usize + offset].clear_bit(from);
                self.pieces[piece.clone() as usize + offset].set_bit(to)
            }
        }

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
                if let MoveType::Castle = move_type {
                } else {
                    self.castling_rights.revoke_all(&self.side_to_move);
                }
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

        self.side_to_move = self.side_to_move.toggle();

        self.update_occupancy();

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
                self.side_to_move = board_history.side_to_move.clone();
                self.castling_rights = board_history.castling_rights.clone();
                self.en_passant_square = board_history.en_passant_square;
                self.half_move_clock = board_history.half_move_clock;
                self.full_move_clock = board_history.full_move_clock;

                Ok(())
            }
            None => Err("No move to unmake!"),
        }
    }

    fn update_occupancy(&mut self) {
        match self.side_to_move {
            Color::White => {
                self.occupancy[0] = self
                    .pieces
                    .iter()
                    .take(6)
                    .fold(BitBoard(0), |acc, x| acc | *x);
            }
            Color::Black => {
                self.occupancy[1] = self
                    .pieces
                    .iter()
                    .skip(6)
                    .fold(BitBoard(0), |acc, x| acc | *x);
            }
        }
    }

    pub fn get_piece(&self, square: usize) -> Option<(Piece, Color)> {
        if self.occupancy[0].is_not_set(square) && self.occupancy[1].is_not_set(square) {
            return None;
        }

        for (i, piece) in self.pieces.into_iter().enumerate() {
            if piece.is_set(square) {
                match i {
                    0..6 => {
                        return Some((Piece::from(i), Color::White));
                    }
                    6..12 => {
                        return Some((Piece::from(i - 6), Color::Black));
                    }
                    _ => panic!("Invalid piece index: {}", i),
                }
            }
        }

        None
    }

    pub fn set_piece(&mut self, piece: Piece, color: Color, square: usize) {
        let bitboard = &mut self.pieces[piece.piece_index(&color)];
        bitboard.set_bit(square);
        self.occupancy[color.color_index()].set_bit(square);
    }

    pub fn clear_piece(&mut self, square: usize, color: Color) {
        if self.occupancy[color.clone() as usize].is_not_set(square) {
            return;
        }

        let offset = match color {
            Color::White => 0,
            Color::Black => 6,
        };

        for i in 0..6 {
            self.pieces[i + offset].clear_bit(square);
        }
    }

    pub fn generate_moves(&self) -> BitBoard {
        let mut moves = BitBoard(0);

        moves |= self.generate_pawn_moves();

        moves |= self.generate_knight_moves();

        moves |= self.generate_bishop_moves();

        moves |= self.generate_rook_moves();

        moves |= self.generate_queen_moves();

        moves |= self.generate_king_moves();

        moves
    }

    pub fn generate_moves_vec(&mut self) -> Vec<Move> {
        macro_rules! extract_moves {
            ($offset:literal, $($piece:expr),+) => {
                {
                    let mut moves: Vec<Move> = Vec::new();

                    $(
                        let pieces = self.pieces[$piece as usize + $offset];

                        for square in pieces.into_iter() {
                            self.pieces[$piece as usize + $offset] = BitBoard(1 << square);

                            let quiet_moves: Option<BitBoard> = match $piece {
                                Piece::Knight => Some(self.generate_knight_moves() & !self.occupancy[self.side_to_move.toggle() as usize]),
                                Piece::Bishop => Some(self.generate_bishop_moves() & !self.occupancy[self.side_to_move.toggle() as usize]),
                                Piece::Rook => Some(self.generate_rook_moves() & !self.occupancy[self.side_to_move.toggle() as usize]),
                                Piece::Queen => Some(self.generate_queen_moves() & !self.occupancy[self.side_to_move.toggle() as usize]),
                                Piece::King => Some(self.generate_king_moves() & !self.occupancy[self.side_to_move.toggle() as usize]),
                                _ => None,
                            };

                            if let Some(quiet_moves) = quiet_moves {
                                moves.extend(quiet_moves.into_iter().map(|destination| Move::new(square, destination)));
                            }

                            let capture_moves: Option<BitBoard> = match $piece {
                                Piece::Knight => Some(self.generate_knight_moves() & self.occupancy[self.side_to_move.toggle() as usize]),
                                Piece::Bishop => Some(self.generate_bishop_moves() & self.occupancy[self.side_to_move.toggle() as usize]),
                                Piece::Rook => Some(self.generate_rook_moves() & self.occupancy[self.side_to_move.toggle() as usize]),
                                Piece::Queen => Some(self.generate_queen_moves() & self.occupancy[self.side_to_move.toggle() as usize]),
                                Piece::King => Some(self.generate_king_moves() & self.occupancy[self.side_to_move.toggle() as usize]),
                                _ => None,
                            };

                            if let Some(capture_moves) = capture_moves {
                                moves.extend(capture_moves.into_iter().map(|destination| Move::new_capture(square, destination)));
                            }

                            if let Piece::Pawn = $piece {
                                let pawn_pushes: Vec<Move> = self.generate_pawn_pushes().into_iter().flat_map(|destination| {
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
                                }).collect();

                                moves.extend(pawn_pushes);

                                let pawn_captures: Vec<Move> = self.generate_pawn_captures().into_iter().flat_map(|destination| {
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
                                }).collect();

                                moves.extend(pawn_captures);

                                let en_passants: Vec<Move> = self.generate_en_passant().into_iter().map(|destination| {
                                    Move::new_en_passant(square, destination)
                                }).collect();

                                moves.extend(en_passants);
                            }

                            if let Piece::King = $piece {
                                let castling_moves: Vec<Move> = self.generate_castling_moves().into_iter().map(|destination| {
                                    Move::new_castle(square, destination)
                                }).collect();

                                moves.extend(castling_moves);
                            }
                        }

                        self.pieces[$piece as usize + $offset] = pieces;
                    )+

                    moves
                }
            };
        }

        let mut moves = Vec::new();

        match self.side_to_move {
            Color::White => moves.extend(extract_moves!(
                0,
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King
            )),
            Color::Black => moves.extend(extract_moves!(
                6,
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King
            )),
        }

        moves
    }

    pub fn generate_ray_moves(&self) -> BitBoard {
        let mut moves = BitBoard(0);

        moves |= self.generate_bishop_moves();

        moves |= self.generate_rook_moves();

        moves |= self.generate_queen_moves();

        moves
    }

    pub fn generate_pawn_moves(&self) -> BitBoard {
        let pawn_pushes = self.generate_pawn_pushes();
        let pawn_captures = self.generate_pawn_captures();
        let en_passant = self.generate_en_passant();

        pawn_pushes | pawn_captures | en_passant
    }

    pub fn generate_pawn_pushes(&self) -> BitBoard {
        let empty_squares = !(self.occupancy[0] | self.occupancy[1]);

        match self.side_to_move {
            Color::White => {
                let single_push = (self.pieces[0] << 8) & empty_squares;

                let second_rank = BitBoard(0x000000000000FF00);
                let double_push =
                    ((self.pieces[0] & second_rank) << 16) & (empty_squares << 8) & empty_squares;

                single_push | double_push
            }
            Color::Black => {
                let single_push = (self.pieces[6] >> 8) & empty_squares;

                let seventh_rank = BitBoard(0x00FF000000000000);
                let double_push =
                    ((self.pieces[6] & seventh_rank) >> 16) & (empty_squares >> 8) & empty_squares;

                single_push | double_push
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

        self.side_to_move = self.side_to_move.toggle();

        if self
            .castling_rights
            .can_castle(&self.side_to_move.toggle(), true)
            && (occupancy & king_side).is_zero()
            && (self.generate_moves() & (king_side | king)).is_zero()
        {
            moves |= match self.side_to_move.toggle() {
                Color::White => BitBoard(0x40),
                Color::Black => BitBoard(0x4000000000000000),
            }
        }

        if self
            .castling_rights
            .can_castle(&self.side_to_move.toggle(), false)
            && (occupancy & queen_side).is_zero()
            && (self.generate_moves() & (queen_side | king)).is_zero()
        {
            moves |= match self.side_to_move.toggle() {
                Color::White => BitBoard(0x04),
                Color::Black => BitBoard(0x0400000000000000),
            }
        }

        self.side_to_move = self.side_to_move.toggle();

        moves
    }
}
