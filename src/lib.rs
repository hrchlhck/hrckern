#![no_std]
#![no_main]

mod std;
mod tty;

use tty::color::Color;
use std::STDOUT;

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

    println!("hello {}", "ola");

    panic!("ee");
}


