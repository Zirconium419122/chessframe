use crate::bitboard::BitBoard;

include!("tables.rs");

#[derive(Debug, Clone, Copy, Default)]
pub struct Magic {
    pub mask: u64,
    pub magic: u64,
    pub relevant_bits: u8,
}

pub fn magic_index(magic: Magic, blockers: BitBoard) -> usize {
    let blockers = blockers & BitBoard(magic.mask);
    let hash = blockers.0.wrapping_mul(magic.magic);
    (hash >> (64 - magic.relevant_bits)) as usize
}

pub fn get_rook_moves(square: usize, blockers: BitBoard) -> BitBoard {
    let magic = &ROOK_MAGICS[square];
    let moves = &ROOK_MOVES_TABLE[square];
    BitBoard(moves[magic_index(*magic, blockers)])
}
