#![no_std]
#![no_main]

mod stdlib;
mod vga;
mod constants;

use stdlib::{write, halt, STDOUT};

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    let stdout = &raw mut STDOUT;

    unsafe{ 
        (*stdout).clear();
    }

    write(b"Hello, world!\n\n");
    write(b"Hello, world!");
    kpanic!("aaaaa");
}


