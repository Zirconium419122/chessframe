use crate::{color::Color, file::File, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub struct CastlingRights(u8);

impl Default for CastlingRights {
    fn default() -> CastlingRights {
        CastlingRights(0b1111)
    }
}

impl CastlingRights {
    const KINGSIDE: u8 = 0b0001;
    const QUEENSIDE: u8 = 0b0010;
    const OFFSET: u8 = 2;

    /// Create a new [`CastlingRights`] struct with no castling rights.
    pub fn new() -> CastlingRights {
        CastlingRights(0b0000)
    }

    /// Convert the [`CastlingRights`] struct to a [`usize`].
    pub fn to_index(self) -> usize {
        match self.0 {
            0b0000 => 0,
            0b0001 => 1,
            0b0010 => 2,
            0b0011 => 3,
            0b0100 => 4,
            0b0101 => 5,
            0b0110 => 6,
            0b0111 => 7,
            0b1000 => 8,
            0b1001 => 9,
            0b1010 => 10,
            0b1011 => 11,
            0b1100 => 12,
            0b1101 => 13,
            0b1110 => 14,
            0b1111 => 15,
            _ => usize::MAX,
        }
    }

    /// Convert the [`CastlingRights`] struct to a [`u8`].
    pub fn to_int(self) -> u8 {
        self.0
    }

    /// Create a new [`CastlingRights`] struct from a part of a FEN string.
    pub fn from_fen(fen: &str) -> Self {
        let mut castling_rights = CastlingRights::new();

        if fen.contains('K') {
            castling_rights = castling_rights.add(Color::White, true);
        }
        if fen.contains('Q') {
            castling_rights = castling_rights.add(Color::White, false);
        }
        if fen.contains('k') {
            castling_rights = castling_rights.add(Color::Black, true);
        }
        if fen.contains('q') {
            castling_rights = castling_rights.add(Color::Black, false);
        }

        castling_rights
    }

    /// Get the castling rights for a specific color.
    pub fn color(&self, color: Color) -> CastlingRights {
        const MASK: u8 = 0b0011;

        CastlingRights(self.0 & (MASK << (color as u8 * CastlingRights::OFFSET)))
    }

    /// Convert a square to the castling rights it represents.
    pub fn square_to_castle_rights(color: Color, square: Square) -> CastlingRights {
        if square == Square::make_square(color.to_backrank(), File::E) {
            CastlingRights::new().add(color, true).add(color, false)
        } else if square == Square::make_square(color.to_backrank(), File::H) {
            CastlingRights::new().add(color, true)
        } else if square == Square::make_square(color.to_backrank(), File::A) {
            CastlingRights::new().add(color, false)
        } else {
            CastlingRights::new()
        }
    }

    /// Remove castling rights provided by the `remove` argument.
    pub fn remove(&self, remove: CastlingRights) -> CastlingRights {
        CastlingRights(self.0 & !remove.0)
    }

    /// Check if a specific color can castle on a specific side.
    #[rustfmt::skip]
    pub fn can_castle(&self, color: Color, kingside: bool) -> bool {
        let offset = if color == Color::Black { CastlingRights::OFFSET } else { 0 };
        let castle_right = if kingside { CastlingRights::KINGSIDE } else { CastlingRights::QUEENSIDE };

        (self.0 & (castle_right << offset)) != 0
    }

    /// Add castling rights for a specific color and side.
    #[rustfmt::skip]
    pub fn add(&mut self, color: Color, kingside: bool) -> CastlingRights {
        let mut castling_rights = *self;

        let offset = if color == Color::Black { CastlingRights::OFFSET } else { 0 };
        let castle_right = if kingside { CastlingRights::KINGSIDE } else { CastlingRights::QUEENSIDE };

        castling_rights.0 |= castle_right << offset;

        castling_rights
    }

    /// Revoke castling rights for a specific color and side.
    #[rustfmt::skip]
    pub fn revoke(&mut self, color: Color, kingside: bool) {
        let mut castling_rights = *self;

        let offset = if color == Color::Black { CastlingRights::OFFSET } else { 0 };
        let castle_right = if kingside { CastlingRights::KINGSIDE } else { CastlingRights::QUEENSIDE };

        castling_rights.0 &= !(castle_right << offset);
    }

    /// Revoke all castling rights for a specific color.
    pub fn revoke_all(&mut self, color: Color) {
        self.revoke(color, true);
        self.revoke(color, false);
    }
}
