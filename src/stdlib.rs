use super::vga::*;

use core::panic::PanicInfo;
use core::arch::asm;

const DEFAULT_COLOR: Color = Color::LightPurple;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn write(fd: &mut VGA, buff: &[u8]) {
    for (i, &byte) in buff.iter().enumerate() {
        fd.offset(i, byte, DEFAULT_COLOR);
    }
}

pub fn halt() {
    unsafe {
        asm!("hlt")
    }
}
