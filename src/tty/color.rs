#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Purple,
    Brown,
    Gray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightPurple,
    Yellow,
    White
}

pub struct ColorByte {
    background: Color,
    foreground: Color
}

impl ColorByte {
    pub fn new(background: Color, foreground: Color) -> Self {
        ColorByte{background, foreground}
    }

    pub fn bg(&self) -> Color {
        self.background
    }

    pub fn fg(&self) -> Color {
        self.foreground
    }
}

impl Into<u8> for ColorByte {
    fn into(self) -> u8 {
        let bg = self.background as u8;
        let fg = self.foreground as u8;

        (bg << 4) | fg
    }
}

impl Into<u8> for Color {
    fn into(self) -> u8 {
        match self {
            Self::Black => 0x0,
            Self::Blue => 0x1,
            Self::Green => 0x2,
            Self::Cyan => 0x3,
            Self::Red => 0x4,
            Self::Purple => 0x5,
            Self::Brown => 0x6,
            Self::Gray => 0x7,
            Self::DarkGray => 0x8,
            Self::LightBlue => 0x9,
            Self::LightGreen => 0xA,
            Self::LightCyan => 0xB,
            Self::LightRed => 0xC,
            Self::LightPurple => 0xD,
            Self::Yellow => 0xE,
            Self::White => 0xF,
        }
    }
}

impl From<u8> for Color {
    fn from(item: u8) -> Color {
        match item {
            0x0 => Color::Black,
            0x1 => Color::Blue,
            0x2 => Color::Green,
            0x3 => Color::Cyan,
            0x4 => Color::Red,
            0x5 => Color::Purple,
            0x6 => Color::Brown,
            0x7 => Color::Gray,
            0x8 => Color::DarkGray,
            0x9 => Color::LightBlue,
            0xA => Color::LightGreen,
            0xB => Color::LightCyan,
            0xC => Color::LightRed,
            0xD => Color::LightPurple,
            0xE => Color::Yellow,
            0xF => Color::White,
            _ => Color::White
        }
    }
}
