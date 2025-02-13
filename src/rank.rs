use std::str::FromStr;

use crate::error::Error;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum Rank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eighth = 7,
}

impl FromStr for Rank {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::InvalidRank);
        }

        match unsafe { s.chars().next().unwrap_unchecked() } {
            '1' => Ok(Rank::First),
            '2' => Ok(Rank::Second),
            '3' => Ok(Rank::Third),
            '4' => Ok(Rank::Fourth),
            '5' => Ok(Rank::Fifth),
            '6' => Ok(Rank::Sixth),
            '7' => Ok(Rank::Seventh),
            '8' => Ok(Rank::Eighth),
            _ => Err(Error::InvalidRank),
        }
    }
}

impl Rank {
    /// Convert a `usize` to a `Rank`. If the index is > 7 wrap around.
    ///
    /// # Example
    /// ```
    /// use chessframe::rank::Rank;
    ///
    /// assert_eq!(Rank::from_index(0), Rank::First);
    /// assert_eq!(Rank::from_index(7), Rank::Eighth);
    /// ```
    #[inline]
    pub fn from_index(index: usize) -> Rank {
        match index % 8 {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => unreachable!(),
        }
    }

    /// Get the rank below this one. If the rank is `First` wrap around.
    ///
    /// # Example
    /// ```
    /// use chessframe::rank::Rank;
    ///
    /// assert_eq!(Rank::First, Rank::Second.down());
    /// assert_eq!(Rank::Eighth, Rank::First.down());
    /// ```
    #[inline]
    pub fn down(&self) -> Rank {
        Rank::from_index(self.to_index().wrapping_sub(1))
    }

    /// Get the rank above this one. If the rank is `Eighth` wrap around.
    ///
    /// # Example
    /// ```
    /// use chessframe::rank::Rank;
    ///
    /// assert_eq!(Rank::Second, Rank::First.up());
    /// assert_eq!(Rank::First, Rank::Eighth.up());
    /// ```
    #[inline]
    pub fn up(&self) -> Rank {
        Rank::from_index(unsafe { self.to_index().unchecked_add(1) })
    }

    /// Convert this `Rank` into a `usize` between 0 and 7.
    ///
    /// # Example
    /// ```
    /// use chessframe::rank::Rank;
    ///
    /// assert_eq!(0, Rank::First.to_index());
    /// assert_eq!(7, Rank::Eighth.to_index());
    /// ```
    #[inline]
    pub fn to_index(self) -> usize {
        self as usize
    }
}
