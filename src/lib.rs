#![no_std]
#![no_main]

mod stdlib;
mod vga;
mod constants;

use core::arch::asm;
use vga::Color;
use stdlib::{write, halt, STDOUT};

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    let stdout = &raw mut STDOUT;

    unsafe { 
        (*stdout).clear();
    }

    write(b"Hello, world!\n\n");
    unsafe { 
        (*stdout).change_background_color(Color::LightPurple);
        let c = Color::combine(Color::LightPurple, Color::Black);
        
        for i in 0..80 {
            (*stdout).set_char_at(b'-', c, 13, i);
        }
    }

    write(b"Hello, world!");

    

    kpanic!("aaaaa");
}


