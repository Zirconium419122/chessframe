use core::fmt;

use crate::{piece::Piece, square::Square};

#[derive(Debug, Clone, PartialEq)]
pub struct ChessMove {
    pub from: Square,
    pub to: Square,
    pub move_type: MoveType,
}

impl fmt::Display for ChessMove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (rank_from, file_from) = (
            self.from.get_rank().to_index() + 1,
            self.from.get_file().to_index(),
        );
        let (rank_to, file_to) = (
            self.to.get_rank().to_index() + 1,
            self.to.get_file().to_index(),
        );

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

impl ChessMove {
    pub fn new(from: Square, to: Square) -> ChessMove {
        ChessMove {
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

    pub fn new_promotion(from: Square, to: Square, promotion: Piece) -> ChessMove {
        ChessMove {
            from,
            to,
            move_type: MoveType::Promotion(promotion),
        }
    }

    pub fn new_capture(from: Square, to: Square) -> ChessMove {
        ChessMove {
            from,
            to,
            move_type: MoveType::Capture,
        }
    }

    pub fn new_capture_promotion(from: Square, to: Square, promotion: Piece) -> ChessMove {
        ChessMove {
            from,
            to,
            move_type: MoveType::CapturePromotion(promotion),
        }
    }

    pub fn new_en_passant(from: Square, to: Square) -> ChessMove {
        ChessMove {
            from,
            to,
            move_type: MoveType::EnPassant,
        }
    }

    pub fn new_castle(from: Square, to: Square) -> ChessMove {
        ChessMove {
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
