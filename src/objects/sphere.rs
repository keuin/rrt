use crate::ray::Ray;
use crate::scene::{HitEvent, Hittable};
use crate::types::{NumPosition, Pixel, PositionVec, Time};

pub struct NormalVectorVisualizedSphere {
    pub center: PositionVec,
    pub radius: NumPosition,
}

impl<T: Pixel> Hittable<T> for NormalVectorVisualizedSphere {
    fn try_hit(&self, ray: &Ray, t1: Time, t2: Time) -> Option<HitEvent<T>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            // does not hit the sphere
            return None;
        }
        // hit time, try the smaller root first
        let mut t = (-b - delta.sqrt()) / (2.0 * a);
        if t < t1 || t >= t2 {
            t = (-b + delta.sqrt()) / (2.0 * a);
            if t < t1 || t >= t2 {
                // no viable solution in range [t1,t2)
                return None;
            }
        }
        let hit_pos = ray.at(t);
        let surface_nv = (hit_pos - self.center).normalize();
        let color = 0.5 * (surface_nv + PositionVec::new(1.0, 1.0, 1.0));
        Some(HitEvent {
            hit_pos,
            surface_nv,
            t,
            color: T::from_rgb_normalized(color.x, color.y, color.z),
        })
    }
}
