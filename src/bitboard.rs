use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not, Shl, Shr};

use crate::r#move::Square;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitBoard(pub u64);

impl Default for BitBoard {
    fn default() -> Self {
        BitBoard(0)
    }
}

impl From<Square> for BitBoard {
    fn from(value: Square) -> Self {
        BitBoard(1 << value as usize)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl Shl<usize> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

impl Shr<usize> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

impl Iterator for BitBoard {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let lsb = self.0.trailing_zeros() as usize;
            self.0 &= self.0 - 1;
            Some(lsb)
        }
    }
}

impl BitBoard {
    pub fn new(bits: u64) -> BitBoard {
        BitBoard(bits)
    }

    pub fn set_bit(&mut self, square: usize) {
        self.0 |= 1 << square;
    }

    pub fn clear_bit(&mut self, square: usize) {
        self.0 &= !(1 << square);
    }

    pub fn is_set(&self, square: usize) -> bool {
        (self.0 & (1 << square)) != 0
    }

    pub fn is_not_set(&self, square: usize) -> bool {
        (self.0 & (1 << square)) == 0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn is_not_zero(&self) -> bool {
        self.0 != 0
    }
}
