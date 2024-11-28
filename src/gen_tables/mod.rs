mod king;
mod magic;
mod pawn;

pub use self::king::write_king_moves;
pub use self::magic::{write_bishop_moves, write_rook_moves};
pub use self::pawn::{write_pawn_moves, write_pawn_attacks};
