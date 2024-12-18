use crate::{color::Color, file::File, square::Square};

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn new() -> CastlingRights {
        CastlingRights(0b0000)
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut castling_rights = CastlingRights::new();
        
        if fen.contains('K') {
            castling_rights = castling_rights.add(&Color::White, true);
        }
        if fen.contains('Q') {
            castling_rights = castling_rights.add(&Color::White, false);
        }
        if fen.contains('k') {
            castling_rights = castling_rights.add(&Color::Black, true);
        }
        if fen.contains('q') {
            castling_rights = castling_rights.add(&Color::Black, false);
        }

        castling_rights
    }

    pub fn square_to_castle_rights(color: &Color, square: Square) -> CastlingRights {
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

    #[rustfmt::skip]
    pub fn remove(&self, remove: CastlingRights) -> CastlingRights {
        CastlingRights(self.0 & !remove.0)
    }

    pub fn can_castle(&self, color: &Color, kingside: bool) -> bool {
        let offset = if color == &Color::Black { CastlingRights::OFFSET } else { 0 };
        let castle_right = if kingside { CastlingRights::KINGSIDE } else { CastlingRights::QUEENSIDE };

        (self.0 & (castle_right << offset)) != 0
    }

    pub fn add(&mut self, color: &Color, kingside: bool) -> CastlingRights {
        let mut castling_rights = *self;

        let offset = if color == &Color::Black { CastlingRights::OFFSET } else { 0 };
        let castle_right = if kingside { CastlingRights::KINGSIDE } else { CastlingRights::QUEENSIDE };

        castling_rights.0 |= castle_right << offset;

        castling_rights
    }

    pub fn revoke(&mut self, color: &Color, kingside: bool) {
        let mut castling_rights = *self;

        let offset = if color == &Color::Black { CastlingRights::OFFSET } else { 0 };
        let castle_right = if kingside { CastlingRights::KINGSIDE } else { CastlingRights::QUEENSIDE };

        castling_rights.0 &= !(castle_right << offset);
    }

    pub fn revoke_all(&mut self, color: &Color) {
        self.revoke(color, true);
        self.revoke(color, false);
    }
}
