use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::primitives::{PointsIter, PrimitiveStyle, Rectangle, StyledDrawable};
use embedded_graphics::transform::Transform;
use embedded_graphics::{Drawable, Pixel};

use super::Display;

struct Bouncer {
    object: Rectangle,
    vector: Point,
}

impl Bouncer {
    pub fn new(rect: Rectangle) -> Self {
        Self {
            object: rect,
            vector: Point::new(1, 1),
        }
    }

    pub fn update(&mut self) {
        self.object.translate_mut(self.vector);
    }

    pub fn bounce(&mut self, bounding_box: Rectangle) {
        if self.object.top_left.x < bounding_box.top_left.x
            || (self.object.top_left + self.object.size).x
                > (bounding_box.top_left + bounding_box.size).x
        {
            self.vector.x = -self.vector.x;
        }
        if self.object.top_left.y < bounding_box.top_left.y
            || (self.object.top_left + self.object.size).y
                > (bounding_box.top_left + bounding_box.size).y
        {
            self.vector.y = -self.vector.y;
        }
    }
}

pub struct SplashScreen {
    bouncer_ratio: u32,
}

impl SplashScreen {
    pub fn new(bouncer_ratio: u32) -> Self {
        Self { bouncer_ratio }
    }

    pub fn run<T: DrawTarget<Color = Rgb888>>(&mut self, target: &mut T) -> Result<(), T::Error> {
        let bounding_box = target.bounding_box();
        let bouncer_rect = Rectangle::with_center(
            bounding_box.top_left + bounding_box.size / 2,
            bounding_box.size / self.bouncer_ratio,
        );
        let mut bouncer = Bouncer::new(bouncer_rect);
        let background_color = Display::default_color();
        let bouncer_color = Rgb888::new(168, 87, 128);
        let bouncer_style = PrimitiveStyle::with_fill(bouncer_color);
        bouncer.object.draw_styled(&bouncer_style, target)?;
        loop {
            let old = bouncer.object;
            bouncer.update();
            let new = bouncer.object;
            let background_pixel_iter = old
                .points()
                .filter(|p| !new.contains(*p))
                .zip(core::iter::repeat(background_color));
            let bouncer_new_pixel_iter = new
                .points()
                .filter(|p| !old.contains(*p))
                .zip(core::iter::repeat(bouncer_color));
            for (point, color) in background_pixel_iter.chain(bouncer_new_pixel_iter) {
                Pixel(point, color).draw(target)?;
            }

            bouncer.bounce(bounding_box);
        }
    }
}
