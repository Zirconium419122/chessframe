use std::ops::Not;

use crate::rank::Rank;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Default)]
pub enum Color {
    #[default]
    White,
    Black,
}

pub const COLORS: [Color; 2] = [Color::White, Color::Black];

impl Not for Color {
    type Output = Color;

    /// Get the opposite color.
    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Color {
    /// Convert [`Color`] to a [`usize`].
    #[inline]
    pub fn to_index(self) -> usize {
        self as usize
    }

    /// Get the back [`Rank`] for the current [`Color`].
    #[inline]
    pub fn to_backrank(self) -> Rank {
        match self {
            Color::White => Rank::First,
            Color::Black => Rank::Eighth,
        }
    }

    /// Get the second [`Rank`] for the current [`Color`].
    #[inline]
    pub fn to_second_rank(self) -> Rank {
        match self {
            Color::White => Rank::Second,
            Color::Black => Rank::Seventh,
        }
    }

    /// Get the fourth [`Rank`] for the current [`Color`].
    #[inline]
    pub fn to_fourth_rank(self) -> Rank {
        match self {
            Color::White => Rank::Fourth,
            Color::Black => Rank::Fifth,
        }
    }

    /// Get the offset for the current [`Color`].
    #[inline]
    pub fn to_offset(self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 6,
        }
    }
}
