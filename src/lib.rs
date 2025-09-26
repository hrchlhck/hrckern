#![no_std]
#![no_main]

mod stdlib;
mod tty;
mod constants;

use tty::color::Color;
use stdlib::{write, halt, STDOUT};

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    let stdout = &raw mut STDOUT;

    unsafe { 
        (*stdout).clear();
    }

    unsafe { 
        (*stdout).change_background_color(Color::LightPurple);
        
        for i in 0..80 {
            (*stdout).set_char_at(b'-', Color::Black, 13, i);
        }
    }

    write(b"Hello, world!\n");

    kprintln!(b"hello!");

    kpanic!("aaaaa");
}


