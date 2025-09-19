#![no_std]
#![no_main]

mod stdlib;
mod vga;

use stdlib::{write, halt};
use vga::VGA;

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    let mut stdout = VGA::new();

    write(&mut stdout, b"Hello, world!\n");
    write(&mut stdout, b"Hello, world! 2");
    halt()
}


