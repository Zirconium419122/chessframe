use crate::bitboard::BitBoard;

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

pub fn get_bishop_moves(square: usize, blockers: BitBoard) -> BitBoard {
    let magic = &BISHOP_MAGICS[square];
    let moves = &BISHOP_MOVES_TABLE[magic_index(*magic, blockers)];
    *moves
}

pub fn get_rook_moves(square: usize, blockers: BitBoard) -> BitBoard {
    let magic = &ROOK_MAGICS[square];
    let moves = &ROOK_MOVES_TABLE[magic_index(*magic, blockers)];
    *moves
}

pub fn get_king_moves(square: usize) -> BitBoard {
    KING_MOVES[square]
}
