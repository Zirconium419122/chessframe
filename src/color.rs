use std::ops::Not;

use crate::rank::Rank;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

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
    /// Convert `Color` to a `usize`.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    #[inline]
    pub fn to_second_rank(&self) -> Rank {
        match self {
            Color::White => Rank::Second,
            Color::Black => Rank::Seventh,
        }
    }

    #[inline]
    pub fn to_fourth_rank(&self) -> Rank {
        match self {
            Color::White => Rank::Fourth,
            Color::Black => Rank::Fifth,
        }
    }

    #[inline]
    pub fn flip(&mut self) {
        *self = !*self;
    }

    #[inline]
    pub fn to_offset(&self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 6,
        }
    }
}
