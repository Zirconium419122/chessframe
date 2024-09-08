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
}
