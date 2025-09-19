#![no_std]
#![no_main]

use core::panic::PanicInfo;

struct VGA {
    buffer: *mut u8
}

impl VGA {
    fn new() -> Self {
        VGA { buffer: 0xB8000 as *mut u8 }
    }

    fn offset(&mut self, i: usize, byte: u8) {
        unsafe{
            *self.buffer.offset(i as isize * 2) = byte;
            *self.buffer.offset(i as isize * 2 + 1) = 0xF;
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn write(fd: &mut VGA, buff: &[u8]) {
    for (i, &byte) in buff.iter().enumerate() {
        fd.offset(i, byte);
    }
}

fn halt() -> ! {
    loop { }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut stdout = VGA::new();

    write(&mut stdout, b"Hello, world!");
    halt()
}


