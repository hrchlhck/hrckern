use super::vga::*;
use super::constants::*;

use core::panic::PanicInfo;
use core::arch::asm;

pub static mut STDOUT: VGA = VGA { 
    buffer: 0xB8000 as *mut u8,
    row_pos: 0, 
    col_pos: 0, 
    max_rows: MAX_SCREEN_HEIGHT, 
    max_cols: MAX_SCREEN_WIDTH,
    cursor: 0
};

#[macro_export]
macro_rules! kpanic {
    ( $($x:expr),* ) => {
        panic!($($x)*);
    };
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
                    (*stdout).offset(byte, Color::Red, Color::White);
                }
            }
        }
    }
    halt();
    loop {}
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
                    (*stdout).offset(byte, Color::LightBlue, Color::White);
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
