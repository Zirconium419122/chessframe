use crate::{bitboard::BitBoard, color::Color, square::Square};

include!("tables.rs");

#[derive(Debug, Clone, Copy, Default)]
pub struct Magic {
    pub mask: BitBoard,
    pub magic: u64,
    pub shift: u8,
    pub offset: u32,
}

pub fn magic_index(magic: Magic, blockers: BitBoard) -> usize {
    let blockers = blockers & magic.mask;
    let hash = blockers.0.wrapping_mul(magic.magic);
    let index = (hash >> magic.shift) as usize;
    magic.offset as usize + index
}

pub fn get_pawn_moves(square: Square, color: Color) -> BitBoard {
    unsafe {
        *PAWN_MOVES
            .get_unchecked(color.to_index())
            .get_unchecked(square.to_index())
    }
}

pub fn get_pawn_attacks(square: Square, color: Color) -> BitBoard {
    unsafe {
        *PAWN_ATTACKS
            .get_unchecked(color.to_index())
            .get_unchecked(square.to_index())
    }
}

pub fn get_knight_moves(square: Square) -> BitBoard {
    unsafe { *KNIGHT_MOVES.get_unchecked(square.to_index()) }
}

pub fn get_bishop_moves(square: Square, blockers: BitBoard) -> BitBoard {
    unsafe {
        let magic = BISHOP_MAGICS.get_unchecked(square.to_index());
        let moves = BISHOP_MOVES_TABLE.get_unchecked(magic_index(*magic, blockers));
        *moves
    }
}

pub fn get_rook_moves(square: Square, blockers: BitBoard) -> BitBoard {
    unsafe {
        let magic = ROOK_MAGICS.get_unchecked(square.to_index());
        let moves = ROOK_MOVES_TABLE.get_unchecked(magic_index(*magic, blockers));
        *moves
    }
}

pub fn get_king_moves(square: Square) -> BitBoard {
    unsafe { *KING_MOVES.get_unchecked(square.to_index()) }
}

pub fn get_castle_moves() -> BitBoard {
    CASTLE_MOVES
}
