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
    let renderer = renderer::new_demo_renderer();
    renderer.render(1);
}
