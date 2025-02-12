use std::str::FromStr;

use crate::error::Error;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl FromStr for File {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::InvalidFile);
        }

        match unsafe { s.chars().next().unwrap_unchecked() } {
            'a' => Ok(File::A),
            'b' => Ok(File::B),
            'c' => Ok(File::C),
            'd' => Ok(File::D),
            'e' => Ok(File::E),
            'f' => Ok(File::F),
            'g' => Ok(File::G),
            'h' => Ok(File::H),
            _ => Err(Error::InvalidFile),
        }
    }
}

impl File {
    /// Convert a `usize` to a `File`. If the index > 7 wrap around.
    ///
    /// # Example
    /// ```
    /// use chessframe::file::File;
    /// 
    /// assert_eq!(File::A, File::from_index(0));
    /// assert_eq!(File::H, File::from_index(7));
    /// ```
    #[inline]
    pub fn from_index(index: usize) -> File {
        match index % 8 {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }

    /// Get the file to the left of this one. If this is `File::A` wrap around.
    ///
    /// # Example
    /// ```
    /// use chessframe::file::File;
    ///
    /// let file = File::E;
    /// assert_eq!(File::D, file.left());
    ///
    /// let file = File::A;
    /// assert_eq!(File::H, file.left());
    /// ```
    #[inline]
    pub fn left(&self) -> File {
        File::from_index(self.to_index().wrapping_sub(1))
    }

    /// Get the file to the left of this one. If this is `File::H` wrap around.
    ///
    /// # Example
    /// ```
    /// use chessframe::file::File;
    ///
    /// let file = File::E;
    /// assert_eq!(File::F, file.right());
    ///
    /// let file = File::H;
    /// assert_eq!(File::A, file.right());
    /// ```
    #[inline]
    pub fn right(&self) -> File {
        File::from_index(unsafe { self.to_index().unchecked_add(1) })
    }

    /// Convert this `File` into a `usize` between 0 and 7.
    ///
    /// # Example
    /// ```
    /// use chessframe::file::File;
    ///
    /// let file = File::A;
    /// assert_eq!(0, file.to_index());
    ///
    /// let file = File::H;
    /// assert_eq!(7, file.to_index());
    /// ```
    #[inline]
    pub fn to_index(self) -> usize {
        self as usize
    }
}
