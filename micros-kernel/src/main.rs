#![no_std]
#![no_main]

use core::panic::PanicInfo;

bootloader_api::entry_point!(main);

fn main(bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = bootinfo.framebuffer.as_mut() {
        for byte in framebuffer.buffer_mut() {
            *byte = 0x90;
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
