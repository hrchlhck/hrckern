const MAX_SCREEN_WIDTH: usize = 128;
const MAX_SCREEN_HEIGHT: usize = 128;

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
    buffer: *mut u8,
    col_pos: usize,
    row_pos: usize,
    max_rows: usize,
    max_cols: usize,
    cursor: usize,
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

    fn newline(&mut self) {
        self.col_pos = 0;
        self.row_pos += 1;
    }

    fn calc_pos(&mut self, offset: usize) {
        if self.col_pos == self.max_cols - 1 {
            self.newline();
        }

        self.cursor = (offset + self.col_pos) + (self.row_pos * self.max_rows);
        self.col_pos += 1;
    }

    pub fn offset(&mut self, i: usize, byte: u8, color: Color) {
        unsafe {
            self.calc_pos(i);
            *self.buffer.offset(self.cursor as isize) = byte;
            *self.buffer.offset(self.cursor as isize + 1) = color as u8;
        }
    }
}