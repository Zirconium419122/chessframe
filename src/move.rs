use core::fmt;

use crate::{
    bitboard::BitBoard, board::Board, castling_rights::CastlingRights, color::Color, piece::Piece,
};

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

impl From<Square> for usize {
    fn from(value: Square) -> Self {
        unsafe { std::mem::transmute::<Square, u8>(value).into() }
    }
}

#[derive(Clone)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub move_type: MoveType,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (rank_from, file_from) = (self.from / 8 + 1, self.from % 8);
        let (rank_to, file_to) = (self.to / 8 + 1, self.to % 8);

        let file_from_char = (file_from as u8 + b'a') as char;
        let file_to_char = (file_to as u8 + b'a') as char;

        write!(
            f,
            "{}{}{}{}",
            file_from_char, rank_from, file_to_char, rank_to
        )
    }
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

    pub fn new_castle<T: Into<usize>>(from: T, to: T) -> Move {
        Move {
            from: from.into(),
            to: to.into(),
            move_type: MoveType::Castle,
        }
    }
}

#[derive(Clone)]
pub enum MoveType {
    Quiet,
    Capture,
    Castle,
    EnPassant,
    Promotion(Piece),
    CapturePromotion(Piece),
    Check,
}

#[derive(Clone)]
pub struct BoardHistory {
    pub pieces: [BitBoard; 12],
    pub occupancy: [BitBoard; 2],
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<BitBoard>,
    pub half_move_clock: u32,
    pub full_move_clock: u32,
}

impl From<Board> for BoardHistory {
    fn from(value: Board) -> Self {
        BoardHistory {
            pieces: value.pieces,
            occupancy: value.occupancy,
            side_to_move: value.side_to_move,
            castling_rights: value.castling_rights,
            en_passant_square: value.en_passant_square,
            half_move_clock: value.half_move_clock,
            full_move_clock: value.full_move_clock,
        }
    }
}
