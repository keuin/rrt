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
    fn get_color(&self, ray: Ray) -> Pixel;
}

impl Camera {
    /// Deterministic method (random source is provided via arguments)
    /// to render a single-sampled image with viewer parameters for given scene.
    /// Say you want a 100-times-sampled image, you have to run get_image for
    /// 100 times and average them pixel by pixel to get the final image.
    /// rnd_x, rnd_y: 0 <= x < 1, random parameters for SSAA.
    pub fn get_image<T: Scene>(&self, scene: &T, rnd_x: f64, rnd_y: f64) -> Image {
        let mut image = Image::new(self.width, self.height);
        for x in 0..image.get_width() {
            for y in 0..image.get_height() {
                // get a sample of those rays whose destination is current pixel
                let pos_pixel = self.get_pixel_pos(x, y);
                let origin = PositionVec::zeros();
                let bias = PositionVec::new(
                    rnd_x * self.pixel_width,
                    -rnd_y * self.pixel_height,
                    0 as NumPosition,
                );
                let direction = (pos_pixel - origin + bias).normalize() as PositionVec;
                let ray = Ray { origin, direction };
                let color = scene.get_color(ray);
                image.set_pixel(x, y, color);
            }
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
                -((self.width as NumPosition) * self.pixel_width / 2.0),
                (self.height as NumPosition) * self.pixel_height / 2.0,
                0 as NumPosition,
            );
        let pixel_pos = pos_left_upper_pixel
            + PositionVec::new(
                self.pixel_width * (x as NumPosition),
                -(self.pixel_height * (y as NumPosition)),
                0 as NumPosition,
            );
        pixel_pos
    }
}

/// a sky scene for testing
pub struct DemoSkyScene {}

impl Scene for DemoSkyScene {
    fn get_color(&self, ray: Ray) -> Pixel {
        let a = 0.5 * (ray.direction.y as f64 + 1.0);
        Pixel::from_rgb_normalized(1.0 - 0.5 * a, 1.0 - 0.3 * a, 1.0)
    }
}

/// Immutable data describing the object space.
pub struct WorldScene {}

pub struct AbsoluteSphereScene {
    pub(crate) sphere_center: PositionVec,
    pub(crate) sphere_radius: NumPosition,
    pub(crate) sphere_color: Pixel,
}

impl Scene for AbsoluteSphereScene {
    fn get_color(&self, ray: Ray) -> Pixel {
        let oc = ray.origin - self.sphere_center;
        let a = ray.direction.norm_squared();
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.sphere_radius * self.sphere_radius;
        if b * b > 4.0 * a * c {
            return self.sphere_color.clone();
        }
        return DemoSkyScene {}.get_color(ray);
    }
}
