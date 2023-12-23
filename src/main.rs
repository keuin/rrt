use std::path::Path;
use crate::ppm::{ColorChannel, ImageSize, Pixel};

mod ppm;
mod testing;

fn main() {
    const WIDTH: ImageSize = 100;
    const HEIGHT: ImageSize = 100;
    let mut img = ppm::Image::new(WIDTH, HEIGHT);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            img.set_pixel(x, y, Pixel::from_rgb(
                ((x as f64 / WIDTH as f64) * 255.0) as ColorChannel,
                ((y as f64 / HEIGHT as f64) * 255.0) as ColorChannel,
                0,
            ));
        }
    }
    img.save(Path::new("./image.ppm")).expect("write image file");
}
