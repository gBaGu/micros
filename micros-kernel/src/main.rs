#![no_std]
#![no_main]

mod framebuffer;

use core::panic::PanicInfo;

#[cfg(target_arch = "x86_64")]
bootloader_api::entry_point!(main);

fn main(bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = bootinfo.framebuffer.as_mut() {
        let mut display = framebuffer::Display::new(framebuffer);
        display.clear();
        let mut splash_screen = framebuffer::SplashScreen::new(10);
        splash_screen.run(&mut display).unwrap();
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
