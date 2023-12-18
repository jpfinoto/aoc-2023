use std::fmt::Formatter;
use std::{fmt, ops};

use itertools::Itertools;

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

    pub fn manhattan_dist(&self) -> i64 {
        self.0.abs() + self.1.abs()
    }

    pub fn turn_left(&self) -> XY {
        XY(-self.1, self.0)
    }

    pub fn turn_right(&self) -> XY {
        XY(self.1, -self.0)
    }

    pub fn rect_range_inclusive(&self, other: XY) -> Vec<XY> {
        let min_x = self.0.min(other.0);
        let max_x = self.0.max(other.0);
        let min_y = self.1.min(other.1);
        let max_y = self.1.max(other.1);

        (min_x..=max_x)
            .cartesian_product(min_y..=max_y)
            .map(|(x, y)| XY(x, y))
            .collect_vec()
    }
}

impl ops::Add for XY {
    type Output = XY;

    fn add(self, rhs: Self) -> Self::Output {
        XY(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Add<&XY> for XY {
    type Output = XY;

    fn add(self, rhs: &Self) -> Self::Output {
        XY(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub for XY {
    type Output = XY;

    fn sub(self, rhs: Self) -> Self::Output {
        XY(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Sub<&XY> for XY {
    type Output = XY;

    fn sub(self, rhs: &Self) -> Self::Output {
        XY(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul<i64> for XY {
    type Output = XY;

    fn mul(self, rhs: i64) -> Self::Output {
        XY(self.0 * rhs, self.1 * rhs)
    }
}

pub fn index_wrap<T>(v: &Vec<T>, i: i64) -> &T {
    let len = v.len() as i64;
    let wrapped = i % len;
    let index = if wrapped < 0 { len + wrapped } else { wrapped };

    &v[index as usize]
}

pub fn shoelace_area(points: &Vec<XY>) -> f64 {
    let mut sum = 0i64;

    for (i, p) in points.iter().enumerate() {
        let p1 = index_wrap(points, i as i64 - 1);
        let p2 = index_wrap(points, i as i64 + 1);
        sum += p.1 * (p1.0 - p2.0);
    }

    (sum as f64) / 2.0
}
