use crate::ppm::Error::IOError;
use crate::types::Pixel;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{io, slice};

pub type ImageSize = u32;

pub type ColorChannel = u8;

const PIXEL_DEPTH: usize = 255;

pub struct Image {
    width: ImageSize,
    height: ImageSize,
    data: Vec<Pixel>,
}

pub struct MutableImageIterator<'a> {
    iter_mut: slice::IterMut<'a, Pixel>,
    width: ImageSize,
    x: ImageSize,
    y: ImageSize,
}

impl<'a> Iterator for MutableImageIterator<'a> {
    type Item = (ImageSize, ImageSize, &'a mut Pixel);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter_mut.next() {
            None => None,
            Some(pixel) => {
                let ret = (self.x, self.y, pixel);
                self.x += 1;
                if self.x >= self.width {
                    self.x = 0;
                    self.y += 1;
                }
                Some(ret)
            }
        }
    }
}

pub struct ImageIterator<'a> {
    x: ImageSize,
    y: ImageSize,
    n: ImageSize,
    img: &'a Image,
}

impl<'a> Iterator for ImageIterator<'a> {
    type Item = (ImageSize, ImageSize, &'a Pixel);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.img.height {
            return None;
        }
        let pixel = (
            self.x,
            self.y,
            self.img
                .data
                .get(self.n as usize)
                .expect("expected pixel while iterating through image pixels"),
        );
        self.x += 1;
        if self.x >= self.img.width {
            self.x = 0;
            self.y += 1;
        }
        self.n += 1;
        Some(pixel)
    }
}

impl Image {
    pub fn get_width(&self) -> ImageSize {
        self.width
    }

    pub fn get_height(&self) -> ImageSize {
        self.height
    }
    pub fn new(width: ImageSize, height: ImageSize) -> Self {
        Image {
            width,
            height,
            data: vec![Pixel::black(); (width * height) as usize],
        }
    }

    pub fn iter(&self) -> ImageIterator {
        ImageIterator {
            x: 0,
            y: 0,
            n: 0,
            img: &self,
        }
    }

    pub fn iter_mut(&mut self) -> MutableImageIterator {
        MutableImageIterator {
            iter_mut: self.data.iter_mut(),
            width: self.width,
            x: 0,
            y: 0,
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let mut file = OpenOptions::new().write(true).create(true).open(path)?;

        // write file header
        file.write(format!("P3\n{} {}\n{}\n", self.width, self.height, PIXEL_DEPTH).as_bytes())?;

        // write pixels
        for (_, _, pix) in self.iter() {
            file.write(format!("{} {} {}\n", pix.red(), pix.green(), pix.blue()).as_bytes())?;
        }

        Ok(())
    }

    pub fn set_pixel(&mut self, x: ImageSize, y: ImageSize, pixel: Pixel) {
        self.data[(x + y * self.width) as usize] = pixel;
    }
}

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        IOError(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::ppm::{ImageSize, Pixel};
    use crate::{ppm, testing};
    use std::fs;
    use std::path::Path;
    use tracing::info;
    use tracing_test::traced_test;

    fn do_test_ppm_1(width: ImageSize, height: ImageSize) {
        let mut img = ppm::Image::new(width, height);
        for x in 0..width {
            for y in 0..height {
                img.set_pixel(
                    x,
                    y,
                    Pixel::from_rgb_normalized(
                        x as f64 / width as f64,
                        y as f64 / height as f64,
                        0.0,
                    ),
                );
            }
        }
        let s = format!("/tmp/rrt_ut_test_ppm_1.{width}x{height}");
        let path_actual = Path::new(&s);
        let path_expected = testing::path(format!("1.{width}x{height}.ppm").as_str());
        img.save(path_actual).expect("write image file");
        info!("Expected file: {:?}", path_expected);
        info!("Temp file: {:?}", path_actual);
        let expected = fs::read(path_expected).expect("read test resource");
        let actual = fs::read(path_actual).expect("read test generated temp file");
        assert_eq!(expected, actual, "unexpected generated ppm image file");
    }

    #[traced_test]
    #[test]
    fn test_ppm_1_100x100() {
        do_test_ppm_1(100, 100);
    }

    #[traced_test]
    #[test]
    fn test_ppm_1_300x200() {
        do_test_ppm_1(300, 200);
    }
}
