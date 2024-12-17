use std::str::FromStr;

use crate::{color::Color, error::Error, file::File, rank::Rank};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Square(u8);

impl FromStr for Square {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(Error::InvalidSquare);
        }

        let chars: [char; 2] = unsafe { s.chars().collect::<Vec<char>>().try_into().unwrap_unchecked() };
        match chars[0] {
            'a'..='h' => {}
            _ => return Err(Error::InvalidSquare),
        }
        match chars[1] {
            '1'..='8' => {}
            _ => return Err(Error::InvalidSquare),
        }

        Ok(Square::make_square(
            Rank::from_index((chars[1] as usize) - ('1' as usize)),
            File::from_index((chars[0] as usize) - ('a' as usize)),
        ))
    }
}

impl Square {
    /// Create a new `Square` given an index.
    #[inline]
    pub fn new(square: u8) -> Square {
        Square(square % 64)
    }

    /// Make a `Square` given its rank and file.
    #[inline]
    pub fn make_square(rank: Rank, file: File) -> Square {
        Square((rank.to_index() as u8) << 3 ^ (file.to_index() as u8))
    }

    /// Return the `Rank` given this `Square`.
    #[inline]
    pub fn get_rank(&self) -> Rank {
        Rank::from_index((self.0 >> 3) as usize)
    }

    /// Return the `File` given this `Square`.
    #[inline]
    pub fn get_file(&self) -> File {
        File::from_index((self.0 % 8) as usize)
    }

    /// If there is a square above this one, return it. Otherwise, return `None`.
    #[inline]
    pub fn up(&self) -> Option<Square> {
        if self.get_rank() == Rank::Eighth {
            None
        } else {
            Some(Square::make_square(self.get_rank().up(), self.get_file()))
        }
    }

    /// If there is a square below this one, return it. Otherwise, return `None`.
    #[inline]
    pub fn down(&self) -> Option<Square> {
        if self.get_rank() == Rank::First {
            None
        } else {
            Some(Square::make_square(self.get_rank().down(), self.get_file()))
        }
    }

    /// If there is a square to the left of this one, return it. Otherwise, return `None`.
    #[inline]
    pub fn left(&self) -> Option<Square> {
        if self.get_file() == File::A {
            None
        } else {
            Some(Square::make_square(self.get_rank(), self.get_file().left()))
        }
    }

    /// If there is a square to the right of this one, return it. Otherwise, return `None`.
    #[inline]
    pub fn right(&self) -> Option<Square> {
        if self.get_file() == File::H {
            None
        } else {
            Some(Square::make_square(
                self.get_rank(),
                self.get_file().right(),
            ))
        }
    }

    /// If there is a square forwards, given a `Color`, go in that direction.
    #[inline]
    pub fn forward(&self, color: &Color) -> Option<Square> {
        match color {
            Color::White => self.up(),
            Color::Black => self.down(),
        }
    }

    /// If there is a square backwards, given a `Color`, go in that direction.
    #[inline]
    pub fn backwards(&self, color: &Color) -> Option<Square> {
        match color {
            Color::White => self.down(),
            Color::Black => self.up(),
        }
    }

    /// If there is a square above this one, return it. Otherwise, wrap around.
    #[inline]
    pub fn wrapping_up(&self) -> Square {
        Square::make_square(self.get_rank().up(), self.get_file())
    }

    /// If there is a square below this one, return it. Otherwise, wrap around.
    #[inline]
    pub fn wrapping_down(&self) -> Square {
        Square::make_square(self.get_rank().down(), self.get_file())
    }

    /// If there is a square to the left of this one, return it. Otherwise, wrap around.
    #[inline]
    pub fn wrapping_left(&self) -> Square {
        Square::make_square(self.get_rank(), self.get_file().left())
    }

    /// If there is a square to the right of this one, return it. Otherwise, wrap around.
    #[inline]
    pub fn wrapping_right(&self) -> Square {
        Square::make_square(self.get_rank(), self.get_file().right())
    }

    /// If there is a square forwards, given a `Color`, go in that direction. Otherwise, wrap around.
    #[inline]
    pub fn wrapping_forward(&self, color: &Color) -> Square {
        match color {
            Color::White => self.wrapping_up(),
            Color::Black => self.wrapping_down(),
        }
    }

    /// If there is a square backwards, given a `Color`, go in that direction. Otherwise, wrap around.
    #[inline]
    pub fn wrapping_backwards(&self, color: &Color) -> Square {
        match color {
            Color::White => self.wrapping_down(),
            Color::Black => self.wrapping_up(),
        }
    }

    /// Convert this `Square` to an integer (`u8`).
    #[inline]
    pub fn to_int(&self) -> u8 {
        self.0
    }

    /// Convert this `Square` to a `usize`.
    #[inline]
    pub fn to_index(&self) -> usize {
        self.0 as usize
    }

    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);
    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);
    pub const A3: Square = Square(16);
    pub const B3: Square = Square(17);
    pub const C3: Square = Square(18);
    pub const D3: Square = Square(19);
    pub const E3: Square = Square(20);
    pub const F3: Square = Square(21);
    pub const G3: Square = Square(22);
    pub const H3: Square = Square(23);
    pub const A4: Square = Square(24);
    pub const B4: Square = Square(25);
    pub const C4: Square = Square(26);
    pub const D4: Square = Square(27);
    pub const E4: Square = Square(28);
    pub const F4: Square = Square(29);
    pub const G4: Square = Square(30);
    pub const H4: Square = Square(31);
    pub const A5: Square = Square(32);
    pub const B5: Square = Square(33);
    pub const C5: Square = Square(34);
    pub const D5: Square = Square(35);
    pub const E5: Square = Square(36);
    pub const F5: Square = Square(37);
    pub const G5: Square = Square(38);
    pub const H5: Square = Square(39);
    pub const A6: Square = Square(40);
    pub const B6: Square = Square(41);
    pub const C6: Square = Square(42);
    pub const D6: Square = Square(43);
    pub const E6: Square = Square(44);
    pub const F6: Square = Square(45);
    pub const G6: Square = Square(46);
    pub const H6: Square = Square(47);
    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);
    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);
}

pub const SQUARES: [Square; 64] = [
    Square::A1,
    Square::B1,
    Square::C1,
    Square::D1,
    Square::E1,
    Square::F1,
    Square::G1,
    Square::H1,
    Square::A2,
    Square::B2,
    Square::C2,
    Square::D2,
    Square::E2,
    Square::F2,
    Square::G2,
    Square::H2,
    Square::A3,
    Square::B3,
    Square::C3,
    Square::D3,
    Square::E3,
    Square::F3,
    Square::G3,
    Square::H3,
    Square::A4,
    Square::B4,
    Square::C4,
    Square::D4,
    Square::E4,
    Square::F4,
    Square::G4,
    Square::H4,
    Square::A5,
    Square::B5,
    Square::C5,
    Square::D5,
    Square::E5,
    Square::F5,
    Square::G5,
    Square::H5,
    Square::A6,
    Square::B6,
    Square::C6,
    Square::D6,
    Square::E6,
    Square::F6,
    Square::G6,
    Square::H6,
    Square::A7,
    Square::B7,
    Square::C7,
    Square::D7,
    Square::E7,
    Square::F7,
    Square::G7,
    Square::H7,
    Square::A8,
    Square::B8,
    Square::C8,
    Square::D8,
    Square::E8,
    Square::F8,
    Square::G8,
    Square::H8,
];
