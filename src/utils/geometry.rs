use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct XY(pub i64, pub i64);

impl XY {
    pub fn as_tuple(&self) -> (i64, i64) {
        (self.0, self.1)
    }
}

impl Add for XY {
    type Output = XY;

    fn add(self, rhs: Self) -> Self::Output {
        XY(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<i64> for XY {
    type Output = XY;

    fn mul(self, rhs: i64) -> Self::Output {
        XY(self.0 * rhs, self.1 * rhs)
    }
}
