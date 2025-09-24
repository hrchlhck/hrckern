use crate::constants::*;

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

pub struct VGA {
    pub buffer: *mut u8,
    pub col_pos: usize,
    pub row_pos: usize,
    pub max_rows: usize,
    pub max_cols: usize,
    pub cursor: usize,
}

impl Color {
    pub fn combine(background: Self, foreground: Self) -> u8 {
        let bg = background as u8;
        let fg = foreground as u8;

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

impl VGA {
    pub fn new() -> Self {
        VGA { 
            buffer: 0xB8000 as *mut u8,
            row_pos: 0, 
            col_pos: 0, 
            max_rows: MAX_SCREEN_HEIGHT, 
            max_cols: MAX_SCREEN_WIDTH,
            cursor: 0
        }
    }

    pub fn clear(&mut self) {
        for i in 0..(self.max_cols * self.max_rows) {
            unsafe {
                *self.buffer.offset(i as isize * 2) = b' ';
                *self.buffer.offset(i as isize * 2 + 1) = Color::combine(Color::Blue, Color::Black);
            }
        }
    }

    pub fn change_background_color(&mut self, bg_color: Color) {
        for i in 0..(self.max_cols * self.max_rows) {
            let i: isize = i as isize * 2 + 1;

            unsafe {
                let foreground: u8 = *self.buffer.offset(i) & 0xF;
                *self.buffer.offset(i) = Color::combine(bg_color, Color::from(foreground));
            }
        }
    }

    pub fn newline(&mut self) {
        self.row_pos += 1;
        self.col_pos = 0;
        self.cursor = (self.row_pos * self.max_cols) * 2;
    }

    fn step(&mut self) {
        self.col_pos += 1;
        if self.col_pos >= self.max_cols {
            self.col_pos = 0;
            self.row_pos += 1;
        }
        self.cursor = (self.row_pos * self.max_cols + self.col_pos) * 2;
    }

    pub fn offset(&mut self, byte: u8, background: Color, foreground: Color) {
        let color = Color::combine(background, foreground);

        unsafe {
            *self.buffer.offset((self.cursor) as isize) = byte;
            *self.buffer.offset((self.cursor + 1) as isize) = color as u8;
        }
        self.step();
    }
}