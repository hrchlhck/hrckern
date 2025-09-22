#![no_std]
#![no_main]

mod stdlib;
mod vga;

use stdlib::{write, halt};
use vga::VGA;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    let mut stdout = VGA::new();

    stdout.clear();

    write(&mut stdout, b"Hello, world!\n\n");
    write(&mut stdout, b"Hello, world!");
    halt()
}


