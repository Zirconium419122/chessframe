use crate::{castling_rights::CastlingRights, color::Color, piece::Piece};

#[derive(Clone, Copy)]
pub struct BitBoard(u64);

impl Default for BitBoard {
    fn default() -> Self {
        BitBoard(0)
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
    pieces: [BitBoard; 12],   // 6 for white, 6 for black
    occupancy: [BitBoard; 2], // white, black occupancy
    side_to_move: Color,
    castling_rights: CastlingRights,
    en_passant_square: Option<BitBoard>,
    half_move_clock: u32,
    full_move_clock: u32,
}

impl Default for Board {
    fn default() -> Self {
        todo!()
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
        todo!()
    }

    fn parse_en_passant(&mut self, en_passant: &str) {
        todo!()
    }

    pub fn get_piece(&self, square: usize) -> Option<(Piece, Color)> {
        todo!()
    }

    pub fn set_piece(&mut self, piece: Piece, color: Color, square: usize) {
        todo!()
    }

    pub fn clear_piece(&mut self, square: usize) {
        todo!()
    }
}
