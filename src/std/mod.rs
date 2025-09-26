use crate::tty::color::Color;
use crate::tty::vga::VGA;

const DEFAULT_BG_COLOR: Color = Color::Blue;
const DEFAULT_FG_COLOR: Color = Color::White;
const MAX_SCREEN_WIDTH: u8 = 80;
const MAX_SCREEN_HEIGHT: u8 = 25;

pub static mut STDOUT: VGA = VGA { 
    buffer: 0xB8000 as *mut u8,
    row_pos: 0, 
    col_pos: 0, 
    cursor: 0,
    max_rows: MAX_SCREEN_HEIGHT as u16, 
    max_cols: MAX_SCREEN_WIDTH as u16,
    fg: DEFAULT_FG_COLOR,
    bg: DEFAULT_BG_COLOR,
    default_fg: DEFAULT_FG_COLOR,
    default_bg: DEFAULT_BG_COLOR,
};

pub mod lib;