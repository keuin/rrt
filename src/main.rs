use crate::renderer::Renderer;
use tracing::debug;

mod ppm;
mod ray;
mod renderer;
mod scene;
mod testing;
mod types;

fn main() {
    tracing_subscriber::fmt::init();
    debug!("Debug logging is enabled");
    let mut renderer = Renderer::new();
    renderer.render();
}
