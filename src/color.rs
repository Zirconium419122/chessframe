#[derive(Clone)]
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

    pub fn color_index(&self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 6,
        }
    }
}
