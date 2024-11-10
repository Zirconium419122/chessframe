use std::ops::Not;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Color;

    /// Get the opposite color
    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Color {
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub fn toggle(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[inline]
    pub fn flip(&mut self) {
        *self = self.toggle();
    }

    #[inline]
    pub fn to_offset(&self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 6,
        }
    }
}
