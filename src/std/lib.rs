use crate::tty::color::{Color, ColorByte};
use super::STDOUT;

use core::fmt::{self, Write};
use core::panic::PanicInfo;
use core::arch::asm;


#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!(""));
    ($($arg:tt)*) => ($crate::eprint!("{}", format_args!($($arg)*)));
}

#[macro_export]
#[doc(hidden)]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::std::lib::_print_msg(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::std::lib::_print_err(format_args!($($arg)*)));
}

#[panic_handler]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    let loc = _info.location().unwrap();

    eprintln!("[ERROR at {}:{}:{}]: {}", loc.file(), loc.line(), loc.column(), _info.message());
    halt();
    loop {}
}

fn _write_color(args: fmt::Arguments, bg: Color, fg: Color) {
    let stdout = &raw mut STDOUT;
    unsafe {
        (*stdout).set_color(ColorByte::new(bg, fg));
        (*stdout).write_fmt(args).unwrap();
        (*stdout).reset_color();
    }
}

#[doc(hidden)]
pub fn _print_err(args: fmt::Arguments) {
    _write_color(args, Color::Red, Color::White);
}

#[doc(hidden)]
pub fn _print_msg(args: fmt::Arguments) {
    _write_color(args, Color::White, Color::LightBlue);
}


pub fn halt() {
    unsafe {
        asm!("hlt")
    }
}
