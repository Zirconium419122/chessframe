use crate::piece::Piece;

pub struct Move {
    pub from: usize,
    pub to: usize,
    pub move_type: MoveType,
}

pub enum MoveType {
    Quiet,
    Capture,
    Castle,
    EnPassant,
    Promotion(Piece),
    Check,
}
