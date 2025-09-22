use super::vga::*;

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn write(fd: &mut VGA, buff: &[u8]) {
    for &byte in buff.iter() {
        match &byte {
            b'\n' => {fd.newline();}
            _ => {fd.offset(byte, Color::LightBlue, Color::White);}
        }
    }
}

pub fn halt() {
    unsafe {
        asm!("hlt")
    }
}
