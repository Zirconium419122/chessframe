use core::fmt;

use crate::{piece::Piece, square::Square};

#[derive(Debug, Clone, PartialEq)]
pub struct ChessMove {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Piece>,
}

impl fmt::Display for ChessMove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (rank_from, file_from) = (
            unsafe { self.from.get_rank().to_index().unchecked_add(1) },
            self.from.get_file().to_index(),
        );
        let (rank_to, file_to) = (
            unsafe { self.to.get_rank().to_index().unchecked_add(1) },
            self.to.get_file().to_index(),
        );

        let file_from_char = (file_from as u8 + b'a') as char;
        let file_to_char = (file_to as u8 + b'a') as char;

        if let Some(promotion) = self.promotion {
            return write!(
                f,
                "{}{}{}{}{}",
                file_from_char,
                rank_from,
                file_to_char,
                rank_to,
                promotion.to_fen()
            );
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
            promotion: None,
        }
    }

    pub fn get_move(&self) -> (&Square, &Square) {
        (&self.from, &self.to)
    }

    pub fn get_promotion(&self) -> Option<Piece> {
        self.promotion
    }

    pub fn new_promotion(from: Square, to: Square, promotion: Piece) -> ChessMove {
        ChessMove {
            from,
            to,
            promotion: Some(promotion),
        }
    }
}

#[deprecated(
    since = "0.0.0",
    note = "MoveType has been phased out of the make_move and generate_moves_vec methods and is therefore not needed any longer for move handling."
)]
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
