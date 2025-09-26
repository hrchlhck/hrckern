use super::color::{Color, ColorByte};

pub struct VGA {
    pub buffer: *mut u8,
    pub col_pos: u16,
    pub row_pos: u16,
    pub max_rows: u16,
    pub max_cols: u16,
    pub cursor: u16,
    pub bg: Color,
    pub fg: Color,
}

impl VGA {
    pub fn clear(&mut self) {
        for i in 0..(self.max_cols * self.max_rows) {
            unsafe {
                *self.buffer.offset(i as isize * 2) = b'\0';
                *self.buffer.offset(i as isize * 2 + 1) = ColorByte::new(self.bg, self.fg).into();
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
        self.cursor = self.get_pos(self.col_pos, self.row_pos);
    }

    fn get_pos(&self, x: u16, y: u16) -> u16 {
        ((x + self.max_cols * y) * 2) as u16
    }

    fn step(&mut self) {
        self.col_pos += 1;
        if self.col_pos >= self.max_cols {
            self.col_pos = 0;
            self.row_pos += 1;
        }
        self.cursor = self.get_pos(self.col_pos, self.row_pos);
    }

    pub fn set_char_at(&mut self, c: u8, color: Color, x: u16, y: u16) {
        let offset = self.get_pos(x, y);
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

    pub fn putc(&mut self, byte: u8) {
        let color = ColorByte::new(self.bg, self.fg);

        unsafe {
            *self.buffer.offset((self.cursor) as isize) = byte;
            *self.buffer.offset((self.cursor + 1) as isize) = color.into();
        }
        self.step();
    }

    pub fn putc_color(&mut self, byte: u8, background: Color, foreground: Color) {
        let color = ColorByte::new(background, foreground);

        unsafe {
            *self.buffer.offset((self.cursor) as isize) = byte;
            *self.buffer.offset((self.cursor + 1) as isize) = color.into();
        }
        self.step();
    }
}