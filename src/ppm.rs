use crate::ppm::Error::IOError;
use crate::types::Pixel;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{io, ops, slice};

pub type ImageSize = u32;

pub type ColorChannel = u8;

const PIXEL_DEPTH: usize = 255;

pub struct Image<T: Pixel> {
    width: ImageSize,
    height: ImageSize,
    data: Vec<T>,
}

pub struct MutableImageIterator<'a, T: Pixel> {
    iter_mut: slice::IterMut<'a, T>,
    width: ImageSize,
    x: ImageSize,
    y: ImageSize,
}

impl<'a, T: Pixel> Iterator for MutableImageIterator<'a, T> {
    type Item = (ImageSize, ImageSize, &'a mut T);

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

pub struct ImageIterator<'a, T: Pixel> {
    x: ImageSize,
    y: ImageSize,
    n: ImageSize,
    img: &'a Image<T>,
}

impl<'a, T: Pixel> Iterator for ImageIterator<'a, T> {
    type Item = (ImageSize, ImageSize, &'a T);

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

impl<T: Pixel> Image<T> {
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
            data: vec![T::black(); (width * height) as usize],
        }
    }

    pub fn iter(&self) -> ImageIterator<T> {
        ImageIterator {
            x: 0,
            y: 0,
            n: 0,
            img: &self,
        }
    }

    pub fn iter_mut(&mut self) -> MutableImageIterator<T> {
        MutableImageIterator {
            iter_mut: self.data.iter_mut(),
            width: self.width,
            x: 0,
            y: 0,
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        // TODO generalize this function to allow generate images with different color depth
        let mut file = OpenOptions::new().write(true).create(true).open(path)?;

        // write file header
        file.write(format!("P3\n{} {}\n{}\n", self.width, self.height, PIXEL_DEPTH).as_bytes())?;

        // write pixels
        for (_, _, pix) in self.iter() {
            file.write(format!("{} {} {}\n", pix.red8(), pix.green8(), pix.blue8()).as_bytes())?;
        }

        Ok(())
    }

    fn index(&self, x: ImageSize, y: ImageSize) -> usize {
        (x + y * self.width) as usize
    }

    pub fn set_pixel(&mut self, x: ImageSize, y: ImageSize, pixel: T) {
        let i = self.index(x, y);
        self.data[i] = pixel;
    }

    pub fn get_pixel(&mut self, x: ImageSize, y: ImageSize) -> T {
        let i = self.index(x, y);
        self.data[i]
    }
}

impl<T: Pixel> ops::MulAssign<f64> for Image<T> {
    fn mul_assign(&mut self, rhs: f64) {
        for pixel in self.data.iter_mut() {
            *pixel *= rhs;
        }
    }
}

impl<T: Pixel> ops::AddAssign<Self> for Image<T> {
    fn add_assign(&mut self, rhs: Self) {
        if self.data.len() != rhs.data.len() || self.width != rhs.width {
            panic!("attempted to add two images with different shape");
        }
        let n = self.data.len();
        for i in 0..n {
            self.data[i] += rhs.data[i];
        }
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
    use crate::types::PixelU8;
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
                    PixelU8::from_rgb_normalized(
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
