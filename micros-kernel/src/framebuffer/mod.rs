mod display;

use bootloader_api::info::{FrameBuffer, PixelFormat};

pub use display::Display;

pub enum ColorEncodingFailed {
    UnsupportedFormat(PixelFormat),
    BufferNotLongEnough,
}

pub trait Color {
    fn encode(&self, format: PixelFormat, dst: &mut [u8]) -> Result<(), ColorEncodingFailed>;
}

pub trait FrameBufferExt {
    fn set_pixel(&mut self, x: usize, y: usize, color: impl Color);
}

impl FrameBufferExt for FrameBuffer {
    fn set_pixel(&mut self, x: usize, y: usize, color: impl Color) {
        let pixel_offset = y * self.info().stride + x;
        let byte_offset = pixel_offset * self.info().bytes_per_pixel;

        let pixel_format = self.info().pixel_format;
        let pixel_buffer = &mut self.buffer_mut()[byte_offset..];
        if let Err(err) = color.encode(pixel_format, pixel_buffer) {
            match err {
                ColorEncodingFailed::UnsupportedFormat(format) => {
                    panic!("unsupported pixel format {:?}", format)
                }
                ColorEncodingFailed::BufferNotLongEnough => {
                    panic!("out of bounds write attempt")
                }
            }
        }
    }
}
