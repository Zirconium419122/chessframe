use crate::bitboard::BitBoard;

include!("tables.rs");

#[derive(Debug, Clone, Copy, Default)]
pub struct Magic {
    pub mask: u64,
    pub magic: u64,
    pub shift: u8,
}

pub fn magic_index(magic: Magic, blockers: BitBoard) -> usize {
    let blockers = blockers & BitBoard(magic.mask);
    let hash = blockers.0.wrapping_mul(magic.magic);
    (hash >> magic.shift) as usize
}

pub fn get_bishop_moves(square: usize, blockers: BitBoard) -> BitBoard {
    let magic = &BISHOP_MAGICS[square];
    let moves = &BISHOP_MOVES_TABLE[square];
    BitBoard(moves[magic_index(*magic, blockers)])
}

pub fn get_rook_moves(square: usize, blockers: BitBoard) -> BitBoard {
    let magic = &ROOK_MAGICS[square];
    let moves = &ROOK_MOVES_TABLE[square];
    BitBoard(moves[magic_index(*magic, blockers)])
}
