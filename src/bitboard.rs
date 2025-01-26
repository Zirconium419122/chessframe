use core::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

use crate::{file::File, rank::Rank, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Default)]
pub struct BitBoard(pub u64);

pub const EMPTY: BitBoard = BitBoard(0);

impl BitAnd for BitBoard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAnd for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAnd<&BitBoard> for BitBoard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: &Self) -> Self::Output {
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

impl BitOr for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOr<&BitBoard> for BitBoard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: &Self) -> Self::Output {
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

impl BitXor for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitXor<&BitBoard> for BitBoard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: &Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for BitBoard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl Not for &BitBoard {
    type Output = BitBoard;

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

/// For the `BitBoard`, iterate over every `Square` set.
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
    /// Construct a new bitboard from a `u64`.
    #[inline]
    pub fn new(bits: u64) -> BitBoard {
        BitBoard(bits)
    }

    /// Construct a new `BitBoard` from a `Rank` and `File`.
    #[inline]
    pub fn set(rank: Rank, file: File) -> BitBoard {
        BitBoard::from_square(Square::make_square(rank, file))
    }

    /// Construct a new `BitBoard` with a `Square` set.
    #[inline]
    pub fn from_square(square: Square) -> BitBoard {
        BitBoard(1 << square.to_int())
    }

    /// Convert a `BitBoard` to a `Square`.
    #[inline]
    pub fn to_square(self) -> Square {
        Square::new(self.0.trailing_zeros() as u8)
    }

    /// Get the number of set bits in the `BitBoard`.
    #[inline]
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    /// Set the bit at `Square`.
    #[inline]
    pub fn set_bit(&mut self, square: Square) {
        self.0 |= 1 << square.to_int();
    }

    /// Clear the bit at `Square`.
    #[inline]
    pub fn clear_bit(&mut self, square: Square) {
        self.0 &= !(1 << square.to_int());
    }

    /// Check if a bit is set at `Square`.
    #[inline]
    pub fn is_set(&self, square: Square) -> bool {
        (self.0 & (1 << square.to_int())) != 0
    }

    /// Check if a bit is not set at `Square`.
    #[inline]
    pub fn is_not_set(&self, square: Square) -> bool {
        (self.0 & (1 << square.to_int())) == 0
    }

    /// Check if the `BitBoard` is zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self == &EMPTY
    }

    /// Check if the `BitBoard` is not zero.
    #[inline]
    pub fn is_not_zero(&self) -> bool {
        self != &EMPTY
    }
}
