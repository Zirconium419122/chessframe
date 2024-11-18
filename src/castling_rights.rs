use crate::color::Color;

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

    pub fn can_castle(&self, color: &Color, kingside: bool) -> bool {
        match (color, kingside) {
            (Color::White, true) => self.white_kingside,
            (Color::White, false) => self.white_queenside,
            (Color::Black, true) => self.black_kingside,
            (Color::Black, false) => self.black_queenside,
        }
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
