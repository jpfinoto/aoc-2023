use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::ops::{Range, RangeInclusive};
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
    #[inline]
    pub fn x(&self) -> &i64 {
        &self.0
    }

    #[inline]
    pub fn y(&self) -> &i64 {
        &self.1
    }

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

    pub fn update_min(&mut self, other: &XY) {
        self.0 = self.0.min(other.0);
        self.1 = self.1.min(other.1);
    }

    pub fn update_max(&mut self, other: &XY) {
        self.0 = self.0.max(other.0);
        self.1 = self.1.max(other.1);
    }

    pub fn cross_z(&self, other: &XY) -> i64 {
        -other.x() * self.y() + self.x() * other.y()
    }

    pub fn range_x(&self, other: &XY) -> Range<i64> {
        let min_x = self.x().min(other.x());
        let max_x = self.x().max(other.x());

        *min_x..*max_x
    }

    pub fn range_y(&self, other: &XY) -> Range<i64> {
        let min_y = self.y().min(other.y());
        let max_y = self.y().max(other.y());

        *min_y..*max_y
    }

    pub fn range_x_inclusive(&self, other: &XY) -> RangeInclusive<i64> {
        let range = self.range_x(other);
        range.start..=range.end
    }

    pub fn range_y_inclusive(&self, other: &XY) -> RangeInclusive<i64> {
        let range = self.range_y(other);
        range.start..=range.end
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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    UpDown,
    LeftRight,
    Corner(i64),
}

pub fn get_odd<XT, YT>(boundary: &HashMap<XY, Direction>, x_range: XT, y_range: YT) -> HashSet<XY>
where
    XT: Iterator<Item = i64> + Clone,
    YT: Iterator<Item = i64>,
{
    let mut inner_tiles = HashSet::new();

    for y in y_range {
        let mut boundary_crossings = 0usize;
        let mut last_corner = None;

        for x in x_range.clone() {
            let p = XY(x, y);

            if let Some(boundary_dir) = boundary.get(&p) {
                match boundary_dir {
                    Direction::UpDown => {
                        boundary_crossings += 1;
                        last_corner = None;
                    }
                    Direction::LeftRight => {}
                    Direction::Corner(dir) => {
                        if let Some(last_dir) = last_corner {
                            if *dir != last_dir {
                                boundary_crossings += 1;
                            }
                        }

                        last_corner = Some(*dir)
                    }
                }
            } else {
                last_corner = None;
                if boundary_crossings % 2 == 1 {
                    inner_tiles.insert(p);
                }
            }
        }
    }

    inner_tiles
}

#[allow(dead_code)]
pub fn print_grid(boundary: &HashMap<XY, Direction>, inner: &HashSet<XY>, p1: &XY, p2: &XY) {
    for y in p1.range_y_inclusive(p2) {
        for x in p1.range_x_inclusive(p2) {
            let p = XY(x, y);
            print!(
                "{}",
                match (boundary.contains_key(&p), inner.contains(&p)) {
                    (true, false) => match boundary.get(&p).unwrap() {
                        Direction::UpDown => '|',
                        Direction::LeftRight => '-',
                        Direction::Corner(i) =>
                            if *i > 0 {
                                '+'
                            } else {
                                '~'
                            },
                    },
                    (false, true) => 'I',
                    (false, false) => '.',
                    (true, true) => 'X',
                }
            );
        }
        println!();
    }
}
