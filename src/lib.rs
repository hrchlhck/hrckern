#![no_std]
#![no_main]

mod std;
mod tty;
mod multiboot;

use tty::color::Color;
use std::STDOUT;
use multiboot::EAXE820;

use core::arch::asm;

fn teste() {
    let rbp: u64;
    let rsp: u64;
    unsafe {
        asm!(
            "mov {}, rbp",
            "mov {}, rsp",
            out(reg) rbp,
            out(reg) rsp
        );
        println!("RSP: {rsp:x}");
        println!("RBP: {rbp:x}");
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn kmain(magic: u64) {
    let stdout = &raw mut STDOUT;

    
    unsafe { 
        (*stdout).clear();
    }
    
    if magic == multiboot::MULTIBOOT2_MAGIC_NUMBER {
        println!("Multiboot2 compliant bootloader");
    }

    
    unsafe { 
        for i in 0..80 {
            (*stdout).set_char_at(b'-', Color::White, i, 13);
        }
    }

    println!("hello {}", "ola");
    teste();
    panic!("ee");
}


