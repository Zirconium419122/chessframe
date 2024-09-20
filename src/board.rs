use crate::{
    bitboard::BitBoard, castling_rights::CastlingRights, color::Color, piece::Piece, r#move::Move,
};

pub struct Board {
    pub pieces: [BitBoard; 12],   // 6 for white, 6 for black
    pub occupancy: [BitBoard; 2], // white, black occupancy
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<BitBoard>,
    pub half_move_clock: u32,
    pub full_move_clock: u32,
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

    pub fn clear_piece(&mut self, square: usize) {
        if self.occupancy[0].is_not_set(square) && self.occupancy[1].is_not_set(square) {
            return;
        }

        if self.occupancy[0].is_set(square) {
            for i in 0..6 {
                self.pieces[i].clear_bit(square);
            }
        } else {
            for i in 6..12 {
                self.pieces[i].clear_bit(square);
            }
        }
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        moves.extend(self.generate_pawn_moves());

        moves.extend(self.generate_knight_moves());

        moves.extend(self.generate_bishop_moves());

        moves.extend(self.generate_rook_moves());

        moves.extend(self.generate_queen_moves());

        moves.extend(self.generate_king_moves());

        moves
    }

    #[rustfmt::skip]
    pub fn generate_pawn_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        let (single_pawn_pushes, double_pawn_pushes) = self.generate_pawn_pushes();
        for square in single_pawn_pushes {
            if self.is_promotion(square) {
                match self.side_to_move {
                    Color::White => {
                        moves.push(Move::new_promotion(square - 8, square, Piece::Queen));
                        moves.push(Move::new_promotion(square - 8, square, Piece::Rook));
                        moves.push(Move::new_promotion(square - 8, square, Piece::Bishop));
                        moves.push(Move::new_promotion(square - 8, square, Piece::Knight));
                    }
                    Color::Black => {
                        moves.push(Move::new_promotion(square + 8, square, Piece::Queen));
                        moves.push(Move::new_promotion(square + 8, square, Piece::Rook));
                        moves.push(Move::new_promotion(square + 8, square, Piece::Bishop));
                        moves.push(Move::new_promotion(square + 8, square, Piece::Knight));
                    }
                }
            } else {
                match self.side_to_move {
                    Color::White => moves.push(Move::new(square - 8, square)),
                    Color::Black => moves.push(Move::new(square + 8, square)),
                }
            }
        }

        for square in double_pawn_pushes {
            match self.side_to_move {
                Color::White => moves.push(Move::new(square - 16, square)),
                Color::Black => moves.push(Move::new(square + 16, square)),
            }
        }

        let (west_pawn_captures, east_pawn_captures) = self.generate_pawn_captures();
        for square in west_pawn_captures {
            if self.is_promotion(square) {
                match self.side_to_move {
                    Color::White => {
                        moves.push(Move::new_capture_promotion(square - 7, square, Piece::Queen));
                        moves.push(Move::new_capture_promotion(square - 7, square, Piece::Rook));
                        moves.push(Move::new_capture_promotion(square - 7, square, Piece::Bishop));
                        moves.push(Move::new_capture_promotion(square - 7, square, Piece::Knight));
                    }
                    Color::Black => {
                        moves.push(Move::new_capture_promotion(square + 9, square, Piece::Queen));
                        moves.push(Move::new_capture_promotion(square + 9, square, Piece::Rook));
                        moves.push(Move::new_capture_promotion(square + 9, square, Piece::Bishop));
                        moves.push(Move::new_capture_promotion(square + 9, square, Piece::Knight));
                    }
                }
            } else {
                match self.side_to_move {
                    Color::White => moves.push(Move::new_capture(square - 7, square)),
                    Color::Black => moves.push(Move::new_capture(square + 9, square)),
                }
            }
        }

        for square in east_pawn_captures {
            if self.is_promotion(square) {
                match self.side_to_move {
                    Color::White => {
                        moves.push(Move::new_capture_promotion(square - 9, square, Piece::Queen));
                        moves.push(Move::new_capture_promotion(square - 9, square, Piece::Rook));
                        moves.push(Move::new_capture_promotion(square - 9, square, Piece::Bishop));
                        moves.push(Move::new_capture_promotion(square - 9, square, Piece::Knight));
                    }
                    Color::Black => {
                        moves.push(Move::new_capture_promotion(square + 7, square, Piece::Queen));
                        moves.push(Move::new_capture_promotion(square + 7, square, Piece::Rook));
                        moves.push(Move::new_capture_promotion(square + 7, square, Piece::Bishop));
                        moves.push(Move::new_capture_promotion(square + 7, square, Piece::Knight));
                    }
                }
            } else {
                match self.side_to_move {
                    Color::White => moves.push(Move::new_capture(square - 9, square)),
                    Color::Black => moves.push(Move::new_capture(square + 7, square)),
                }
            }
        }

        let (west_en_passant, east_en_passant) = self.generate_en_passant();
        for square in west_en_passant {
            match self.side_to_move {
                Color::White => moves.push(Move::new_en_passant(square - 7, square)),
                Color::Black => moves.push(Move::new_en_passant(square + 9, square)),
            }
        }

        for square in east_en_passant {
            match self.side_to_move {
                Color::White => moves.push(Move::new_en_passant(square - 9, square)),
                Color::Black => moves.push(Move::new_en_passant(square + 7, square)),
            }
        }

        moves
    }

    pub fn generate_pawn_pushes(&self) -> (BitBoard, BitBoard) {
        let empty_squares = !(self.occupancy[0] | self.occupancy[1]);

        match self.side_to_move {
            Color::White => {
                let single_push = (self.pieces[0] << 8) & empty_squares;

                let second_rank = BitBoard(0x000000000000FF00);
                let double_push =
                    ((self.pieces[0] & second_rank) << 16) & (empty_squares << 8) & empty_squares;

                (single_push, double_push)
            }
            Color::Black => {
                let single_push = (self.pieces[6] >> 8) & empty_squares;

                let seventh_rank = BitBoard(0x00FF000000000000);
                let double_push =
                    ((self.pieces[6] & seventh_rank) >> 16) & (empty_squares >> 8) & empty_squares;

                (single_push, double_push)
            }
        }
    }

    pub fn generate_pawn_captures(&self) -> (BitBoard, BitBoard) {
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

                (northwest_capture, northeast_capture)
            }
            Color::Black => {
                let southwest_capture =
                    (self.pieces[6] >> 9) & opponents_pieces & !BitBoard(0x8080808080808080); // Mask out the H file

                let southeast_capture =
                    (self.pieces[6] >> 7) & opponents_pieces & !BitBoard(0x0101010101010101); // Mask out the A file

                (southwest_capture, southeast_capture)
            }
        }
    }

    pub fn generate_en_passant(&self) -> (BitBoard, BitBoard) {
        if let Some(en_passant) = self.en_passant_square {
            match self.side_to_move {
                Color::White => {
                    let west_ep =
                        (self.pieces[0] << 7) & en_passant & !BitBoard(0x8080808080808080); // Mask out the H file
                    let east_ep =
                        (self.pieces[0] << 9) & en_passant & !BitBoard(0x0101010101010101); // Mask out the A file

                    (west_ep, east_ep)
                }
                Color::Black => {
                    let west_ep =
                        (self.pieces[6] >> 9) & en_passant & !BitBoard(0x8080808080808080); // Mask out the H file
                    let east_ep =
                        (self.pieces[6] >> 7) & en_passant & !BitBoard(0x0101010101010101); // Mask out the A file

                    (west_ep, east_ep)
                }
            }
        } else {
            (BitBoard(0), BitBoard(0))
        }
    }

    pub fn is_promotion(&self, square: usize) -> bool {
        match self.side_to_move {
            Color::White => square >= 56, // Rank 8
            Color::Black => square < 8,   // Rank 1
        }
    }

    pub fn generate_knight_moves(&self) -> Vec<Move> {
        todo!()
    }

    pub fn generate_bishop_moves(&self) -> Vec<Move> {
        todo!()
    }

    pub fn generate_rook_moves(&self) -> Vec<Move> {
        todo!()
    }

    pub fn generate_queen_moves(&self) -> Vec<Move> {
        todo!()
    }

    pub fn generate_king_moves(&self) -> Vec<Move> {
        todo!()
    }
}
