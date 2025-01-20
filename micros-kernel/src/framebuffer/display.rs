use bootloader_api::info::{FrameBuffer, PixelFormat};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Dimensions, OriginDimensions, Size},
    pixelcolor::{Rgb888, RgbColor},
    Pixel,
};

use super::{Color, ColorEncodingFailed, FrameBufferExt};

const DEFAULT_DISPLAY_COLOR: Rgb888 = Rgb888::new(87, 127, 168);

impl<T: RgbColor> Color for T {
    fn encode(&self, format: PixelFormat, dst: &mut [u8]) -> Result<(), ColorEncodingFailed> {
        match format {
            PixelFormat::Rgb | PixelFormat::Bgr if dst.len() < 3 => {
                return Err(ColorEncodingFailed::BufferNotLongEnough)
            }
            PixelFormat::U8 if dst.len() < 1 => {
                return Err(ColorEncodingFailed::BufferNotLongEnough)
            }
            PixelFormat::Rgb => {
                dst[0] = self.r();
                dst[1] = self.g();
                dst[2] = self.b();
            }
            PixelFormat::Bgr => {
                dst[0] = self.b();
                dst[1] = self.g();
                dst[2] = self.r();
            }
            PixelFormat::U8 => {
                // use a simple average-based grayscale transform
                let gray = self.r() / 3 + self.g() / 3 + self.b() / 3;
                dst[0] = gray;
            }
            other => return Err(ColorEncodingFailed::UnsupportedFormat(other)),
        };
        Ok(())
    }
}

pub struct Display<'f> {
    framebuffer: &'f mut FrameBuffer,
}

impl<'f> Display<'f> {
    pub fn new(framebuffer: &mut FrameBuffer) -> Display {
        Display { framebuffer }
    }

    pub fn default_color() -> Rgb888 {
        DEFAULT_DISPLAY_COLOR
    }

    pub fn clear(&mut self) {
        // SAFETY: Self::Error is Infallible
        self.fill_solid(&self.bounding_box(), DEFAULT_DISPLAY_COLOR).unwrap();
    }

    fn draw_pixel(&mut self, Pixel(coordinates, color): Pixel<Rgb888>) {
        let (x, y) = {
            let c: (i32, i32) = coordinates.into();
            (c.0 as usize, c.1 as usize)
        };

        if (0..self.framebuffer.info().width).contains(&x)
            && (0..self.framebuffer.info().height).contains(&y)
        {
            self.framebuffer.set_pixel(x, y, color);
        }
    }
}

impl<'f> OriginDimensions for Display<'f> {
    fn size(&self) -> Size {
        Size::new(
            self.framebuffer.info().width as u32,
            self.framebuffer.info().height as u32,
        )
    }
}

impl<'f> DrawTarget for Display<'f> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels.into_iter() {
            self.draw_pixel(pixel);
        }
        Ok(())
    }
}
