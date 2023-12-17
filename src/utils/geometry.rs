use std::fmt::Formatter;
use std::{fmt, ops};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct XY(pub i64, pub i64);

impl fmt::Display for XY {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let x = self.0;
        let y = self.1;

        f.write_fmt(format_args!("{x},{y}"))
    }
}

impl XY {
    pub fn as_tuple(&self) -> (i64, i64) {
        (self.0, self.1)
    }

    // should this be Option<XY>?
    pub fn normalize(&self) -> XY {
        if *self == XY(0, 0) {
            *self
        } else if self.0 == 0 {
            XY(0, self.1 / self.1.abs())
        } else if self.1 == 0 {
            XY(self.0 / self.0.abs(), 0)
        } else {
            todo!()
        }
    }
}

impl ops::Add for XY {
    type Output = XY;

    fn add(self, rhs: Self) -> Self::Output {
        XY(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub for XY {
    type Output = XY;

    fn sub(self, rhs: Self) -> Self::Output {
        XY(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul<i64> for XY {
    type Output = XY;

    fn mul(self, rhs: i64) -> Self::Output {
        XY(self.0 * rhs, self.1 * rhs)
    }
}
