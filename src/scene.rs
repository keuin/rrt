use crate::ppm::{Image, ImageSize};
use crate::ray::Ray;
use crate::types::{NumPosition, Pixel, PositionVec};

/// Storing viewer's parameter.
pub struct Camera {
    /// position of the sensor center.
    /// The position origin is focus point.
    pub pos: PositionVec,
    // /// width/height, must be positive
    // wh_ratio: NumPosition,
    /// image width in pixels, even number
    pub width: ImageSize,
    /// image height in pixels, even number
    pub height: ImageSize,

    pub pixel_width: NumPosition,
    pub pixel_height: NumPosition,
    pub focus_length: NumPosition,
}

/// Scene describes how objects in the world is organized.
pub trait Scene: Send + Sync {
    fn get_color(&self, ray: Ray) -> Pixel {
        todo!()
    }
}

impl Camera {
    /// Deterministic method (random source is provided via arguments)
    /// to render a single-sampled image with viewer parameters for given scene.
    /// Say you want a 100-times-sampled image, you have to run get_image for
    /// 100 times and average them pixel by pixel to get the final image.
    /// rnd_x, rnd_y: 0 <= x < 1, random parameters for SSAA.
    pub fn get_image<T: Scene>(&self, scene: &T, rnd_x: f64, rnd_y: f64) -> Image {
        let mut image = Image::new(self.width, self.height);
        for (x, y, pixel) in (&mut image).into_iter() {
            // get a sample of those rays whose destination is current pixel
            let pos_pixel = self.get_pixel_pos(x, y);
            let origin = PositionVec::zeros();
            let bias = PositionVec::new(
                rnd_x * self.pixel_width,
                -rnd_y * self.pixel_height,
                0 as NumPosition,
            );
            let ray = Ray {
                color: Pixel::black(),
                origin,
                direction: pos_pixel - origin + bias,
            };
            *pixel = scene.get_color(ray);
        }
        image
    }

    /// Convert image pixel position (x, y) to 3D position in camera position system.
    /// Returns the position of the pixel's upper-left corner.
    fn get_pixel_pos(&self, x: ImageSize, y: ImageSize) -> PositionVec {
        let pos_sensor_center =
            PositionVec::new(0 as NumPosition, 0 as NumPosition, -self.focus_length);
        let pos_left_upper_pixel = pos_sensor_center
            + PositionVec::new(
                -((self.width as f64) * self.pixel_width / 2.0),
                (self.height as f64) * self.pixel_width / 2.0,
                0 as NumPosition,
            );
        let pixel_pos = pos_left_upper_pixel
            + PositionVec::new(
                self.pixel_width * (x as f64),
                -(self.pixel_height * (y as f64)),
                0 as NumPosition,
            );
        pixel_pos
    }
}

/// a sky scene for testing
pub struct DemoSkyScene {}

impl Scene for DemoSkyScene {
}

/// Immutable data describing the object space.
pub struct WorldScene {}
