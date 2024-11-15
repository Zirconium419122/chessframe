use core::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not, Shl, Shr};

use crate::square::Square;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BitBoard(pub u64);

pub const EMPTY: BitBoard = BitBoard(0);

impl BitAnd for BitBoard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl Not for BitBoard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl Shl<usize> for BitBoard {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: usize) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

impl Shr<usize> for BitBoard {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: usize) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

/// For the `BitBoard`, iterate over every `square` set
impl Iterator for BitBoard {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let lsb = self.to_square();
            self.0 &= self.0 - 1;
            Some(lsb)
        }
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s: String = "".to_string();
        for square in 0..64 {
            if self.is_set(Square::new(square)) {
                s.push_str("X ");
            } else {
                s.push_str(". ");
            }
            if square % 8 == 7 {
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}

impl BitBoard {
    /// Construct a new bitboard from a `u64`
    #[inline]
    pub fn new(bits: u64) -> BitBoard {
        BitBoard(bits)
    }

    /// Construct a new `BitBoard` with a `Square` set
    #[inline]
    pub fn from_square(square: Square) -> BitBoard {
        BitBoard(1 << square.to_int())
    }

    /// Convert a `BitBoard` to a `Square`
    #[inline]
    pub fn to_square(&self) -> Square {
        Square::new(self.0.trailing_zeros() as u8)
    }

    /// Set the bit at `square`
    #[inline]
    pub fn set_bit(&mut self, square: Square) {
        self.0 |= 1 << square.to_int();
    }

    /// Clear the bit at `square`
    #[inline]
    pub fn clear_bit(&mut self, square: Square) {
        self.0 &= !(1 << square.to_int());
    }

    /// Check if a bit is set at `Square`
    #[inline]
    pub fn is_set(&self, square: Square) -> bool {
        (self.0 & (1 << square.to_int())) != 0
    }

    /// Check if a bit is not set at `Square`
    #[inline]
    pub fn is_not_set(&self, square: Square) -> bool {
        (self.0 & (1 << square.to_int())) == 0
    }

    /// Check if the `BitBoard` is zero
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Check if the `BitBoard` is not zero
    #[inline]
    pub fn is_not_zero(&self) -> bool {
        self.0 != 0
    }
}
