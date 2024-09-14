use crate::{castling_rights::CastlingRights, color::Color, piece::Piece, r#move::Square};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitBoard(pub u64);

impl Default for BitBoard {
    fn default() -> Self {
        BitBoard(0)
    }
}

impl From<Square> for BitBoard {
    fn from(value: Square) -> Self {
        BitBoard(1 << value as usize)
    }
}

impl BitBoard {
    pub fn new(bits: u64) -> BitBoard {
        BitBoard(bits)
    }

    pub fn set_bit(&mut self, square: usize) {
        self.0 |= 1 << square;
    }

    pub fn clear_bit(&mut self, square: usize) {
        self.0 &= !(1 << square);
    }

    pub fn is_set(&self, square: usize) -> bool {
        (self.0 & (1 << square)) != 0
    }
}

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
        if !self.occupancy[0].is_set(square) && !self.occupancy[1].is_set(square) {
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
        if !self.occupancy[0].is_set(square) && !self.occupancy[1].is_set(square) {
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
}
