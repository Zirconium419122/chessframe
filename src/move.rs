use core::fmt;

use crate::{
    bitboard::BitBoard, board::Board, castling_rights::CastlingRights, piece::Piece, square::Square,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub move_type: MoveType,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (rank_from, file_from) = (self.from.get_rank().to_index(), self.from.get_file());
        let (rank_to, file_to) = (self.to.get_rank().to_index(), self.to.get_file());

        let file_from_char = (file_from as u8 + b'a') as char;
        let file_to_char = (file_to as u8 + b'a') as char;

        match &self.move_type {
            MoveType::Promotion(piece) | MoveType::CapturePromotion(piece) => {
                return write!(
                    f,
                    "{}{}{}{}{}",
                    file_from_char,
                    rank_from,
                    file_to_char,
                    rank_to,
                    piece.to_fen()
                );
            }
            _ => (),
        }

        write!(
            f,
            "{}{}{}{}",
            file_from_char, rank_from, file_to_char, rank_to
        )
    }
}

impl Move {
    pub fn new(from: Square, to: Square) -> Move {
        Move {
            from,
            to,
            move_type: MoveType::Quiet,
        }
    }

    pub fn get_move(&self) -> (&Square, &Square) {
        (&self.from, &self.to)
    }

    pub fn get_move_type(&self) -> &MoveType {
        &self.move_type
    }

    pub fn new_promotion(from: Square, to: Square, promotion: Piece) -> Move {
        Move {
            from,
            to,
            move_type: MoveType::Promotion(promotion),
        }
    }

    pub fn new_capture(from: Square, to: Square) -> Move {
        Move {
            from,
            to,
            move_type: MoveType::Capture,
        }
    }

    pub fn new_capture_promotion(from: Square, to: Square, promotion: Piece) -> Move {
        Move {
            from,
            to,
            move_type: MoveType::CapturePromotion(promotion),
        }
    }

    pub fn new_en_passant(from: Square, to: Square) -> Move {
        Move {
            from,
            to,
            move_type: MoveType::EnPassant,
        }
    }

    pub fn new_castle(from: Square, to: Square) -> Move {
        Move {
            from,
            to,
            move_type: MoveType::Castle,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveType {
    Quiet,
    Capture,
    Check,
    Castle,
    EnPassant,
    Promotion(Piece),
    CapturePromotion(Piece),
}

#[derive(Clone, Copy)]
pub struct BoardHistory {
    pub pieces: [BitBoard; 12],
    pub occupancy: [BitBoard; 2],
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<BitBoard>,
    pub half_move_clock: u32,
}

impl From<&Board> for BoardHistory {
    fn from(value: &Board) -> Self {
        BoardHistory {
            pieces: value.pieces,
            occupancy: value.occupancy,
            castling_rights: value.castling_rights,
            en_passant_square: value.en_passant_square,
            half_move_clock: value.half_move_clock,
        }
    }
}
