use crate::{color::Color, file::File, square::Square};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}

impl CastlingRights {
    pub fn new() -> CastlingRights {
        CastlingRights {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut castling_rights = CastlingRights::default();

        if !fen.contains('K') {
            castling_rights.white_kingside = false;
        }
        if !fen.contains('Q') {
            castling_rights.white_queenside = false;
        }
        if !fen.contains('k') {
            castling_rights.black_kingside = false;
        }
        if !fen.contains('q') {
            castling_rights.black_queenside = false;
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
        let mut castling_rights = *self;

        castling_rights.white_kingside = if remove.white_kingside { false } else { castling_rights.white_kingside };
        castling_rights.white_queenside = if remove.white_queenside { false } else { castling_rights.white_queenside };
        castling_rights.black_kingside = if remove.black_kingside { false } else { castling_rights.black_kingside };
        castling_rights.black_queenside = if remove.black_queenside { false } else { castling_rights.black_queenside };

        castling_rights
    }

    pub fn can_castle(&self, color: &Color, kingside: bool) -> bool {
        match (color, kingside) {
            (Color::White, true) => self.white_kingside,
            (Color::White, false) => self.white_queenside,
            (Color::Black, true) => self.black_kingside,
            (Color::Black, false) => self.black_queenside,
        }
    }

    pub fn add(&mut self, color: &Color, kingside: bool) -> CastlingRights {
        let mut castling_rights = *self;
        match (color, kingside) {
            (Color::White, true) => castling_rights.white_kingside = true,
            (Color::White, false) => castling_rights.white_queenside = true,
            (Color::Black, true) => castling_rights.black_kingside = true,
            (Color::Black, false) => castling_rights.black_queenside = true,
        }

        castling_rights
    }

    pub fn revoke(&mut self, color: &Color, kingside: bool) {
        match (color, kingside) {
            (Color::White, true) => self.white_kingside = false,
            (Color::White, false) => self.white_queenside = false,
            (Color::Black, true) => self.black_kingside = false,
            (Color::Black, false) => self.black_queenside = false,
        }
    }

    pub fn revoke_all(&mut self, color: &Color) {
        match color {
            Color::White => {
                self.white_kingside = false;
                self.white_queenside = false;
            }
            Color::Black => {
                self.black_kingside = false;
                self.black_queenside = false;
            }
        }
    }
}
