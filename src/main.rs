use crate::renderer::Renderer;
use tracing::debug;

mod ppm;
mod renderer;
mod testing;

fn main() {
    tracing_subscriber::fmt::init();
    debug!("Debug logging is enabled");
    let mut renderer = Renderer::new();
    renderer.render();
}
