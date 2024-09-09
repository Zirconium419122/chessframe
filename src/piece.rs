use crate::color::Color;

#[repr(usize)]
#[derive(Clone)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub fn to_fen(&self) -> char {
        match self {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        }
    }

    pub fn piece_index(&self, color: Color) -> usize {
        let offset = color.color_index();
        self.clone() as usize + offset
    }
}
