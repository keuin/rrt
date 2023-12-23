use crate::types::{Pixel, PositionVec};

pub struct Ray {
    pub color: Pixel,
    pub origin: PositionVec,
    pub direction: PositionVec,
}
