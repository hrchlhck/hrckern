use super::tty::vga::VGA;
use super::tty::color::Color;

use core::panic::PanicInfo;
use core::arch::asm;

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
    bg: DEFAULT_BG_COLOR
};

#[macro_export]
macro_rules! kpanic {
    ( $($x:expr),* ) => {
        use crate::stdlib::write_color;
        write_color(b"ERROR: ", Color::Red, Color::White);
        panic!($($x)*);
    };
}

#[macro_export]
macro_rules! kprintln {
    ( $($x:expr),* ) => {
            write_color(b"INFO: ", Color::LightBlue, Color::White);
            write_color($($x)*, Color::LightBlue, Color::White);
            write_color(b"\n", Color::LightBlue, Color::White);
    }
}

#[panic_handler]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    let buffer: &[u8] = _info.message().as_str().unwrap().as_bytes();
    let stdout = &raw mut STDOUT;

    for &byte in buffer.iter() {
        unsafe {
            match &byte {
                b'\n' => {
                    (*stdout).newline();
                }
                _ => {
                    (*stdout).putc_color(byte, Color::Red, Color::White);
                }
            }
        }
    }
    halt();
    loop {}
}

pub fn write_color(buff: &[u8], bg: Color, fg: Color) {
    let stdout = &raw mut STDOUT;
    for &byte in buff.iter() {
        unsafe {
            match &byte {
                b'\n' => {
                    (*stdout).newline();
                }
                _ => {
                    (*stdout).putc_color(byte, bg, fg);
                }
            }
        }
    }
}

pub fn write(buff: &[u8]) {
    let stdout = &raw mut STDOUT;
    for &byte in buff.iter() {
        unsafe {
            match &byte {
                b'\n' => {
                    (*stdout).newline();
                }
                _ => {
                    (*stdout).putc(byte);
                }
            }
        }
    }
}



pub fn halt() {
    unsafe {
        asm!("hlt")
    }
}
