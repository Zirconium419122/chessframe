use crate::{
    bitboard::BitBoard, castling_rights::CastlingRights, color::Color, file::File, piece::Piece,
    rank::Rank, square::Square,
};

include!(concat!(env!("OUT_DIR"), "/tables.rs"));
include!(concat!(env!("OUT_DIR"), "/magic_tables.rs"));

#[derive(Debug, Clone, Copy, Default)]
pub struct Magic {
    pub mask: BitBoard,
    pub magic: u64,
    pub shift: u8,
    pub offset: u32,
}

#[inline]
pub fn magic_index(magic: Magic, blockers: BitBoard) -> usize {
    let blockers = blockers & magic.mask;
    let hash = blockers.0.wrapping_mul(magic.magic);
    let index = (hash >> magic.shift) as usize;
    magic.offset as usize + index
}

#[inline]
pub fn get_pawn_moves(square: Square, color: Color) -> BitBoard {
    unsafe {
        *PAWN_MOVES
            .get_unchecked(color.to_index())
            .get_unchecked(square.to_index())
    }
}

#[inline]
pub fn get_pawn_attacks(square: Square, color: Color) -> BitBoard {
    unsafe {
        *PAWN_ATTACKS
            .get_unchecked(color.to_index())
            .get_unchecked(square.to_index())
    }
}

#[inline]
pub fn get_knight_moves(square: Square) -> BitBoard {
    unsafe { *KNIGHT_MOVES.get_unchecked(square.to_index()) }
}

#[inline]
pub fn get_bishop_moves(square: Square, blockers: BitBoard) -> BitBoard {
    unsafe {
        let magic = BISHOP_MAGICS.get_unchecked(square.to_index());
        let moves = BISHOP_MOVES_TABLE.get_unchecked(magic_index(*magic, blockers));
        *moves
    }
}

#[inline]
pub fn get_rook_moves(square: Square, blockers: BitBoard) -> BitBoard {
    unsafe {
        let magic = ROOK_MAGICS.get_unchecked(square.to_index());
        let moves = ROOK_MOVES_TABLE.get_unchecked(magic_index(*magic, blockers));
        *moves
    }
}

#[inline]
pub fn get_king_moves(square: Square) -> BitBoard {
    unsafe { *KING_MOVES.get_unchecked(square.to_index()) }
}

#[inline]
pub fn get_castle_moves() -> BitBoard {
    CASTLE_MOVES
}

#[inline]
pub fn get_bishop_rays(square: Square) -> BitBoard {
    unsafe { *BISHOP_RAYS.get_unchecked(square.to_index()) }
}

#[inline]
pub fn get_rook_rays(square: Square) -> BitBoard {
    unsafe { *ROOK_RAYS.get_unchecked(square.to_index()) }
}

#[inline]
pub fn get_file(file: File) -> BitBoard {
    unsafe { *FILES.get_unchecked(file.to_index()) }
}

#[inline]
pub fn get_adjacent_files(file: File) -> BitBoard {
    unsafe { *ADJACENT_FILES.get_unchecked(file.to_index()) }
}

#[inline]
pub fn get_rank(rank: Rank) -> BitBoard {
    unsafe { *RANKS.get_unchecked(rank.to_index()) }
}

#[inline]
pub fn get_between(start: Square, end: Square) -> BitBoard {
    unsafe {
        *BETWEEN
            .get_unchecked(start.to_index())
            .get_unchecked(end.to_index())
    }
}

#[inline]
pub fn get_tangent(start: Square, end: Square) -> BitBoard {
    unsafe {
        *TANGENT
            .get_unchecked(start.to_index())
            .get_unchecked(end.to_index())
    }
}

pub struct Zobrist;

impl Zobrist {
    #[inline]
    pub fn get_side_to_move() -> u64 {
        ZOBRIST_SIDE_TO_MOVE
    }

    #[inline]
    pub fn get_piece(piece: Piece, square: Square, color: Color) -> u64 {
        unsafe {
            *ZOBRIST_PIECES
                .get_unchecked(color.to_index())
                .get_unchecked(piece.to_index())
                .get_unchecked(square.to_index())
        }
    }

    #[inline]
    pub fn get_castle(castling_rights: CastlingRights, color: Color) -> u64 {
        unsafe {
            *ZOBRIST_CASTLE
                .get_unchecked(color.to_index())
                .get_unchecked(castling_rights.color(color).to_index() >> (2 * color.to_index()))
        }
    }

    #[inline]
    pub fn get_en_passant(file: File, color: Color) -> u64 {
        unsafe {
            *ZOBRIST_ENPASSANT
                .get_unchecked(color.to_index())
                .get_unchecked(file.to_index())
        }
    }
}
