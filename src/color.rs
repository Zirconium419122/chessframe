#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn toggle(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn flip(&mut self) {
        *self = self.toggle();
    }

    pub fn to_offset(&self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 6,
        }
    }

    pub fn color_index(&self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 1,
        }
    }
}
