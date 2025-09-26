#![no_std]
#![no_main]

mod stdlib;
mod tty;

use tty::color::Color;
use stdlib::{write, STDOUT};

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    let stdout = &raw mut STDOUT;

    unsafe { 
        (*stdout).clear();
    }

    unsafe { 
        for i in 0..80 {
            (*stdout).set_char_at(b'-', Color::White, i, 13);
        }
    }

    write(b"Hello, world!\n");

    kprintln!(b"hello!");

    kpanic!("aaaaa");
}


