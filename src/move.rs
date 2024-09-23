use crate::piece::Piece;

#[rustfmt::skip]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl TryFrom<usize> for Square {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0..64 => Ok(unsafe { std::mem::transmute::<u8, Square>(value as u8) }),
            64.. => Err("Index out of bounds, must be between 0 and 63!"),
        }
    }
}

impl Into<usize> for Square {
    fn into(self) -> usize {
        unsafe { std::mem::transmute::<Square, u8>(self).into() }
    }
}

pub struct Move {
    pub from: usize,
    pub to: usize,
    pub move_type: MoveType,
}

impl Move {
    pub fn new<T: Into<usize>>(from: T, to: T) -> Move {
        Move {
            from: from.into(),
            to: to.into(),
            move_type: MoveType::Quiet,
        }
    }

    pub fn get_move(&self) -> (usize, usize) {
        (self.from, self.to)
    }

    pub fn get_move_type(&self) -> &MoveType {
        &self.move_type
    }

    pub fn new_promotion<T: Into<usize>>(from: T, to: T, promotion: Piece) -> Move {
        Move {
            from: from.into(),
            to: to.into(),
            move_type: MoveType::Promotion(promotion),
        }
    }

    pub fn new_capture<T: Into<usize>>(from: T, to: T) -> Move {
        Move {
            from: from.into(),
            to: to.into(),
            move_type: MoveType::Capture,
        }
    }

    pub fn new_capture_promotion<T: Into<usize>>(from: T, to: T, promotion: Piece) -> Move {
        Move {
            from: from.into(),
            to: to.into(),
            move_type: MoveType::CapturePromotion(promotion),
        }
    }

    pub fn new_en_passant<T: Into<usize>>(from: T, to: T) -> Move {
        Move {
            from: from.into(),
            to: to.into(),
            move_type: MoveType::EnPassant,
        }
    }
}

pub enum MoveType {
    Quiet,
    Capture,
    Castle,
    EnPassant,
    Promotion(Piece),
    CapturePromotion(Piece),
    Check,
}
