use crate::objects::sphere::NormalVectorVisualizedSphere;
use crate::scene::Hittable;
use crate::types::PositionVec;
use tracing::debug;

mod objects;
mod ppm;
mod ray;
mod renderer;
mod scene;
mod testing;
mod types;

fn main() {
    tracing_subscriber::fmt::init();
    debug!("Debug logging is enabled");
    let sphere = NormalVectorVisualizedSphere {
        center: PositionVec::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let objects: Vec<&dyn Hittable> = vec![&sphere];
    let renderer = renderer::new_skied_world(objects);
    renderer.render(1);
}
