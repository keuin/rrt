use crate::ppm::{Image, ImageSize};
use crate::ray::Ray;
use crate::types::{NumPosition, Pixel, PositionVec, Time};
use num_traits::float::FloatCore;
use std::marker::PhantomData;

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
    type T: Pixel;
    fn get_color(&self, ray: Ray) -> Self::T;
}

impl Camera {
    /// Deterministic method (random source is provided via arguments)
    /// to render a single-sampled image with viewer parameters for given scene.
    /// Say you want a 100-times-sampled image, you have to run get_image for
    /// 100 times and average them pixel by pixel to get the final image.
    /// rnd_x, rnd_y: 0 <= x < 1, random parameters for SSAA.
    pub fn get_image<T: Scene>(&self, scene: &T, rnd_x: f64, rnd_y: f64) -> Image<T::T> {
        let mut image = Image::new(self.width, self.height);
        for (x, y, pixel) in image.iter_mut() {
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
pub struct DemoSkyScene<T: Pixel> {
    _marker: PhantomData<T>,
}

impl<T: Pixel> DemoSkyScene<T> {
    pub fn new() -> Self {
        DemoSkyScene {
            _marker: PhantomData::default(),
        }
    }
}

impl<T: Pixel> Scene for DemoSkyScene<T> {
    type T = T;

    fn get_color(&self, ray: Ray) -> Self::T {
        let a = 0.5 * (ray.direction.y as f64 + 1.0);
        T::from_rgb_normalized(1.0 - 0.5 * a, 1.0 - 0.3 * a, 1.0)
    }
}

pub struct AbsoluteSphereScene<T: Pixel> {
    sphere_center: PositionVec,
    sphere_radius: NumPosition,
    sphere_color: T,
}

impl<T: Pixel> AbsoluteSphereScene<T> {
    pub fn new(sphere_center: PositionVec, sphere_radius: NumPosition, sphere_color: T) -> Self {
        AbsoluteSphereScene {
            sphere_center,
            sphere_radius,
            sphere_color,
        }
    }
}

impl<T: Pixel> Scene for AbsoluteSphereScene<T> {
    type T = T;
    fn get_color(&self, ray: Ray) -> T {
        let oc = ray.origin - self.sphere_center;
        let a = ray.direction.norm_squared();
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.sphere_radius * self.sphere_radius;
        if b * b > 4.0 * a * c {
            return self.sphere_color.clone();
        }
        return DemoSkyScene::new().get_color(ray);
    }
}

pub struct NormVectorVisualizedSphereScene<T: Pixel> {
    sphere_center: PositionVec,
    sphere_radius: NumPosition,
    _marker: PhantomData<T>,
}

impl<T: Pixel> NormVectorVisualizedSphereScene<T> {
    pub fn new(sphere_center: PositionVec, sphere_radius: NumPosition) -> Self {
        NormVectorVisualizedSphereScene {
            sphere_center,
            sphere_radius,
            _marker: Default::default(),
        }
    }
}

impl<T: Pixel> Scene for NormVectorVisualizedSphereScene<T> {
    type T = T;
    fn get_color(&self, ray: Ray) -> T {
        let oc = ray.origin - self.sphere_center;
        let a = ray.direction.norm_squared();
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.sphere_radius * self.sphere_radius;
        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            // does not hit the sphere
            return DemoSkyScene::new().get_color(ray);
        }
        // hit time, the smaller root
        let t = (-b - delta.sqrt()) / (2.0 * a);
        let surface_normal = (ray.at(t) - self.sphere_center).normalize();
        let color = 0.5 * (surface_normal + PositionVec::new(1.0, 1.0, 1.0));
        T::from_rgb_normalized(color.x, color.y, color.z)
    }
}

/// the result of a hit
pub struct HitEvent<T: Pixel> {
    /// hit point position
    pub hit_pos: PositionVec,
    /// hit surface normal vector, pointing to outer surface
    pub surface_nv: PositionVec,
    /// hit time
    pub t: Time,
    /// color of the hit point
    pub color: T,
}

pub trait Hittable<T: Pixel>: Send + Sync {
    /// test whether the given ray will hit this object in time range `t1` <= t < `t2`,
    /// returning the smallest `t` that hits the object and satisfy the range constraint
    fn try_hit(&self, ray: &Ray, t1: Time, t2: Time) -> Option<HitEvent<T>>;
}

pub struct SkiedWorld<'a, T: Pixel> {
    pub(crate) objects: Vec<&'a dyn Hittable<T>>,
}

impl<'a, T: Pixel> Scene for SkiedWorld<'a, T> {
    type T = T;

    fn get_color(&self, ray: Ray) -> T {
        let mut last_hit: Option<HitEvent<T>> = None;
        let mut t_max = Time::infinity();
        for obj in &self.objects {
            if let Some(hit) = obj.try_hit(&ray, 0.0, t_max) {
                if hit.t < t_max {
                    t_max = hit.t;
                    last_hit = Some(hit);
                }
            }
        }
        match last_hit {
            None => DemoSkyScene::new().get_color(ray),
            Some(hit) => hit.color,
        }
    }
}
