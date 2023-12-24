use crate::ppm::ColorChannel;
use nalgebra::Vector3;
use std::fmt::{Display, Formatter};

pub type NumColor = u8;
pub type NumColorRatio = f64;

pub const NUM_COLOR_MAX: NumColor = 255;

pub type NumPosition = f64;
pub type Time = f64;
pub type PositionVec = Vector3<NumPosition>;

// TODO this is a quick abstraction for 8bit image rendering.
// Generalize the color depth in the future.
pub trait Pixel: Send + Sync + Copy {
    fn red(&self) -> NumColorRatio;
    fn green(&self) -> NumColorRatio;
    fn blue(&self) -> NumColorRatio;
    fn red8(&self) -> NumColor;
    fn green8(&self) -> NumColor;
    fn blue8(&self) -> NumColor;
    fn from_rgb_normalized(r: NumColorRatio, g: NumColorRatio, b: NumColorRatio) -> Self;
    fn from_rgb8(r: ColorChannel, g: ColorChannel, b: ColorChannel) -> Self;
    fn black() -> Self;
    fn from<T: Pixel>(value: &T) -> Self;
}

#[derive(Copy, Clone)]
pub struct PixelU8 {
    rgb: Vector3<NumColor>,
}

impl Display for PixelU8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Pixel8(r={}, g={}, b={})",
            self.rgb.x, self.rgb.y, self.rgb.z
        ))
    }
}

impl AsRef<Vector3<NumColor>> for PixelU8 {
    fn as_ref(&self) -> &Vector3<NumColor> {
        &self.rgb
    }
}

impl Into<Vector3<NumColor>> for PixelU8 {
    fn into(self) -> Vector3<NumColor> {
        self.rgb
    }
}

impl Pixel for PixelU8 {
    fn red(&self) -> NumColorRatio {
        self.rgb.x as NumColorRatio / NUM_COLOR_MAX as NumColorRatio
    }

    fn green(&self) -> NumColorRatio {
        self.rgb.y as NumColorRatio / NUM_COLOR_MAX as NumColorRatio
    }

    fn blue(&self) -> NumColorRatio {
        self.rgb.z as NumColorRatio / NUM_COLOR_MAX as NumColorRatio
    }

    fn red8(&self) -> NumColor {
        self.rgb.x
    }

    fn green8(&self) -> NumColor {
        self.rgb.y
    }

    fn blue8(&self) -> NumColor {
        self.rgb.z
    }

    fn from_rgb_normalized(r: NumColorRatio, g: NumColorRatio, b: NumColorRatio) -> Self {
        PixelU8 {
            rgb: Vector3::new(
                (r * 255.999) as NumColor,
                (g * 255.999) as NumColor,
                (b * 255.999) as NumColor,
            ),
        }
    }

    fn from_rgb8(r: ColorChannel, g: ColorChannel, b: ColorChannel) -> Self {
        PixelU8 {
            rgb: Vector3::new(r, g, b),
        }
    }

    fn black() -> Self {
        PixelU8 {
            rgb: Vector3::new(0, 0, 0),
        }
    }

    fn from<T: Pixel>(value: &T) -> Self {
        PixelU8 {
            rgb: Vector3::new(value.red8(), value.green8(), value.blue8()),
        }
    }
}

#[derive(Copy, Clone)]
pub struct PixelF64 {
    rgb: Vector3<NumColorRatio>,
}

impl PixelF64 {
    pub fn new(red: NumColorRatio, green: NumColorRatio, blue: NumColorRatio) -> PixelF64 {
        PixelF64 {
            rgb: Vector3::new(red, green, blue),
        }
    }
    pub fn black() -> Self {
        PixelF64 {
            rgb: Vector3::zeros(),
        }
    }
}

impl Pixel for PixelF64 {
    fn red(&self) -> NumColorRatio {
        self.rgb.x
    }

    fn green(&self) -> NumColorRatio {
        self.rgb.y
    }

    fn blue(&self) -> NumColorRatio {
        self.rgb.z
    }

    fn red8(&self) -> NumColor {
        (self.rgb.x * 255.999) as NumColor
    }

    fn green8(&self) -> NumColor {
        (self.rgb.y * 255.999) as NumColor
    }

    fn blue8(&self) -> NumColor {
        (self.rgb.z * 255.999) as NumColor
    }

    fn from_rgb_normalized(r: NumColorRatio, g: NumColorRatio, b: NumColorRatio) -> Self {
        PixelF64 {
            rgb: Vector3::new(r, g, b),
        }
    }

    fn from_rgb8(r: ColorChannel, g: ColorChannel, b: ColorChannel) -> Self {
        PixelF64 {
            rgb: Vector3::new(
                r as NumColorRatio / 255.0,
                g as NumColorRatio / 255.0,
                b as NumColorRatio / 255.0,
            ),
        }
    }

    fn black() -> Self {
        PixelF64 {
            rgb: Vector3::zeros(),
        }
    }

    fn from<T: Pixel>(value: &T) -> Self {
        PixelF64 {
            rgb: Vector3::new(value.red(), value.green(), value.blue()),
        }
    }
}
