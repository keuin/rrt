use crate::types::{PositionVec, Time};

pub struct Ray {
    pub origin: PositionVec,
    pub direction: PositionVec,
}

impl Ray {
    /// get the ray's position at given time `t`
    pub fn at(&self, t: Time) -> PositionVec {
        self.origin + self.direction.scale(t)
    }
}
