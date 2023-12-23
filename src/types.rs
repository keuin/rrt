use nalgebra::Vector3;
use crate::ppm::ColorChannel;

pub type NumColor = u8;

pub const NUM_COLOR_MAX: NumColor = 255;

pub type NumPosition = f64;
pub type Time = f64;
pub type PositionVec = Vector3<NumPosition>;

#[derive(Clone)]
pub struct Pixel {
    rgb: Vector3<NumColor>
}

impl AsRef<Vector3<NumColor>> for Pixel {
    fn as_ref(&self) -> &Vector3<NumColor> {
        &self.rgb
    }
}

impl Into<Vector3<NumColor>> for Pixel {
    fn into(self) -> Vector3<NumColor> {
        self.rgb
    }
}

impl Pixel {
    pub fn red(&self) -> NumColor {
        self.rgb.x
    }
    pub fn green(&self) -> NumColor {
        self.rgb.y
    }
    pub fn blue(&self) -> NumColor {
        self.rgb.z
    }

    pub fn black() -> Self {
        Pixel{
            rgb: Vector3::new(
                0 as NumColor,
                0 as NumColor,
                0 as NumColor,
            ),
        }
    }

    pub fn white() -> Self {
        Pixel{
            rgb: Vector3::new(
                    NUM_COLOR_MAX,
                    NUM_COLOR_MAX,
                    NUM_COLOR_MAX,
            ),
        }
    }

    pub fn from_rgb(red: ColorChannel, green:  ColorChannel, blue: ColorChannel) -> Self {
        Pixel{
            rgb: Vector3::new(red, green, blue),
        }
    }
}