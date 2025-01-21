#![no_std]
#![no_main]

mod framebuffer;

use core::panic::PanicInfo;

use bootloader_api::info::FrameBufferInfo;
use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;

pub(crate) static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

#[cfg(target_arch = "x86_64")]
bootloader_api::entry_point!(main);

pub(crate) fn init_logger(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || LockedLogger::new(buffer, info, true, false));
    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Hello, Kernel Mode!");
}

fn main(bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = bootinfo.framebuffer.as_mut() {
        let mut display = framebuffer::Display::new(framebuffer);
        display.clear();
        let mut splash_screen = framebuffer::SplashScreen::new(10);
        splash_screen.run(&mut display).unwrap();
        let info = framebuffer.info();
        init_logger(framebuffer.buffer_mut(), info);
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
