#[derive(Debug, Clone, PartialEq)]
/// A single character on the console screen.
pub struct Pixel {
    pub character: char,
    pub fg: Color,
    pub bg: Color,
}

impl Pixel {
    pub fn new(character: char, fg: Color, bg: Color) -> Self {
        Self { character, fg, bg }
    }
}

impl From<char> for Pixel {
    fn from(character: char) -> Self {
        Self::new(character, Color::White, Color::Black)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}
