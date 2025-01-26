use crate::color::Color;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub const PIECES: [Piece; 6] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

impl From<usize> for Piece {
    fn from(value: usize) -> Self {
        match value {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => panic!("Invalid piece index"),
        }
    }
}

impl From<char> for Piece {
    fn from(value: char) -> Self {
        match value {
            'p' => Piece::Pawn,
            'n' => Piece::Knight,
            'b' => Piece::Bishop,
            'r' => Piece::Rook,
            'q' => Piece::Queen,
            'k' => Piece::King,
            _ => panic!("Invalid piece character"),
        }
    }
}

#[allow(dead_code)]
impl Piece {
    pub fn to_fen(self) -> char {
        match self {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        }
    }

    pub fn to_index(self) -> usize {
        self as usize
    }

    pub fn piece_index(&self, color: &Color) -> usize {
        let offset = match color {
            Color::White => 0,
            Color::Black => 6,
        };
        *self as usize + offset
    }
}
