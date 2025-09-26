use super::color::{Color, ColorByte};

pub struct VGA {
    pub buffer: *mut u8,
    pub col_pos: usize,
    pub row_pos: usize,
    pub max_rows: usize,
    pub max_cols: usize,
    pub cursor: usize,
}

impl VGA {
    pub fn clear(&mut self) {
        for i in 0..(self.max_cols * self.max_rows) {
            unsafe {
                *self.buffer.offset(i as isize * 2) = b'\0';
                *self.buffer.offset(i as isize * 2 + 1) = ColorByte::new(Color::Blue, Color::Black).into();
            }
        }
    }

    pub fn change_background_color(&mut self, bg_color: Color) {
        for i in 0..(self.max_cols * self.max_rows) {
            let i: isize = i as isize * 2 + 1;

            unsafe {
                let foreground: u8 = *self.buffer.offset(i) & 0xF;
                *self.buffer.offset(i) = ColorByte::new(bg_color, foreground.into()).into();
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

    pub fn set_char_at(&mut self, c: u8, color: Color, x: usize, y: usize) {
        let offset = (x * self.max_cols + y) * 2;
        let current_bg_color: Color = self.get_current_bg_color();
        let new_color: ColorByte = ColorByte::new(current_bg_color, color);

        unsafe {
            *self.buffer.offset(offset as isize) = c;
            *self.buffer.offset((offset + 1) as isize) = new_color.into();
        }
    }

    fn get_current_bg_color(&self) -> Color {
        unsafe {
            (*self.buffer.offset((self.cursor + 1) as isize) >> 4).into()
        }
    }

    pub fn offset(&mut self, byte: u8, foreground: Color) {
        let current_bg_color: Color = self.get_current_bg_color();
        let color = ColorByte::new(current_bg_color, foreground);

        unsafe {
            *self.buffer.offset((self.cursor) as isize) = byte;
            *self.buffer.offset((self.cursor + 1) as isize) = color.into();
        }
        self.step();
    }

    pub fn offset_err(&mut self, byte: u8) {
        let color = ColorByte::new(Color::Red, Color::White);

        unsafe {
            *self.buffer.offset((self.cursor) as isize) = byte;
            *self.buffer.offset((self.cursor + 1) as isize) = color.into();
        }
        self.step();
    }
}