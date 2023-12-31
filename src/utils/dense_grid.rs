use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::utils::geometry::{wrap_number, XY};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct DenseGrid<T> {
    pub width: usize,
    pub filler: Option<T>,
    pub items: Vec<T>,
}

pub const UP: XY = XY(0, -1);
pub const DOWN: XY = XY(0, 1);
pub const LEFT: XY = XY(-1, 0);
pub const RIGHT: XY = XY(1, 0);
pub const ORIGIN: XY = XY(0, 0);

impl<T> DenseGrid<T>
where
    T: Copy,
{
    pub fn parse(block: &str, cell_parser: fn(c: char) -> T, filler: Option<T>) -> DenseGrid<T> {
        let width = block.splitn(2, "\n").map(str::trim).next().unwrap().len();
        let items = block
            .split("\n")
            .map(str::trim)
            .flat_map(str::chars)
            .map(cell_parser)
            .collect_vec();

        DenseGrid {
            width,
            filler,
            items,
        }
    }

    pub fn get(&self, xy: XY) -> Option<&T> {
        let (x, y) = xy.as_tuple();

        if x < 0 || y < 0 || x >= (self.width as i64) || y >= (self.height() as i64) {
            self.filler.as_ref()
        } else {
            let index = y * (self.width as i64) + x;
            self.items.get(index as usize).or(self.filler.as_ref())
        }
    }

    pub fn set_if_inbounds(&mut self, xy: XY, value: T) {
        let (x, y) = xy.as_tuple();

        if x >= 0 && y >= 0 && x < (self.width as i64) && y < (self.height() as i64) {
            let index = y * (self.width as i64) + x;
            self.items[index as usize] = value;
        }
    }

    pub fn get_mut(&mut self, xy: XY) -> Option<&mut T> {
        let (x, y) = xy.as_tuple();

        if x < 0 || y < 0 || x >= (self.width as i64) || y >= (self.height() as i64) {
            None
        } else {
            let index = y * (self.width as i64) + x;
            self.items.get_mut(index as usize).or(None)
        }
    }

    pub fn find<F>(&self, predicate: F) -> Option<(&T, XY)>
    where
        F: Fn(&T) -> bool,
    {
        self.items
            .iter()
            .enumerate()
            .find(|(_, item)| predicate(*item))
            .and_then(|(i, item)| Some((item, self.index_to_xy(i))))
    }

    pub fn index_to_xy(&self, idx: usize) -> XY {
        XY((idx % self.width) as i64, (idx / self.width) as i64)
    }

    pub fn height(&self) -> usize {
        self.items.len() / self.width
    }

    #[allow(dead_code)]
    pub fn rows_iter(&self) -> impl Iterator<Item = &[T]> {
        (0..self.height()).map(move |y| &self.items[y * self.width..(y + 1) * self.width])
    }

    pub fn columns_iter(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.width).map(|x| {
            (0..self.height())
                .map(|y| self.get(XY(x as i64, y as i64)).unwrap())
                .collect_vec()
        })
    }

    pub fn range<R>(&self, x_range: R, y_range: R) -> impl Iterator<Item = (XY, Option<&T>)>
    where
        R: Clone + Iterator<Item = i64>,
    {
        y_range
            .map(move |y| {
                x_range.clone().map(move |x| {
                    let p = XY(x, y);
                    (p, self.get(p))
                })
            })
            .flatten()
    }

    pub fn rect_range_inclusive(&self, a: XY, b: XY) -> impl Iterator<Item = (XY, Option<&T>)> {
        let min_x = a.0.min(b.0);
        let max_x = a.0.max(b.0);
        let min_y = a.1.min(b.1);
        let max_y = a.1.max(b.1);

        self.range(min_x..=max_x, min_y..=max_y)
    }

    pub fn from_columns(
        columns: &Vec<Vec<T>>,
        filler: Option<T>,
    ) -> Result<DenseGrid<T>, &'static str> {
        let height = match columns.iter().map(Vec::len).dedup().collect_vec().len() {
            0 => Err("no columns"),
            1 => Ok(columns[0].len()),
            _ => Err("columns are not uniform"),
        }?;
        let width = columns.len();

        let items = (0..height)
            .flat_map(|y| (0..width).map(move |x| columns[x][y]))
            .collect_vec();

        Ok(DenseGrid {
            width: columns.len(),
            filler,
            items,
        })
    }

    pub fn from_rows(
        columns: &Vec<Vec<T>>,
        filler: Option<T>,
    ) -> Result<DenseGrid<T>, &'static str> {
        let width = match columns.iter().map(Vec::len).dedup().collect_vec().len() {
            0 => Err("no rows"),
            1 => Ok(columns[0].len()),
            _ => Err("rows are not uniform"),
        }?;

        let items = columns.iter().flatten().cloned().collect();

        Ok(DenseGrid {
            width,
            filler,
            items,
        })
    }

    pub fn new_filled(width: usize, height: usize, base: T, filler: Option<T>) -> DenseGrid<T> {
        DenseGrid {
            width,
            filler,
            items: vec![base; width * height],
        }
    }

    pub fn find_one(&self, target: &T) -> Option<XY>
    where
        T: Eq + PartialEq,
    {
        Some(self.find(|el| el == target)?.1)
    }

    pub fn cardinal_neighbours<'a>(
        &'a self,
        pos: &'a XY,
    ) -> impl Iterator<Item = (XY, Option<&T>)> + 'a {
        [UP, DOWN, LEFT, RIGHT].iter().map(|d| {
            let p = *pos + d;
            (p, self.get(p))
        })
    }

    pub fn cardinal_neighbours_with_wrapping<'a>(
        &'a self,
        pos: &'a XY,
    ) -> impl Iterator<Item = (XY, Option<&T>)> + 'a {
        [UP, DOWN, LEFT, RIGHT].iter().map(|d| {
            let p = self.wrap(&(*pos + d));
            (p, self.get(p))
        })
    }
    fn wrap(&self, p: &XY) -> XY {
        XY(
            wrap_number(*p.x(), self.width as i64),
            wrap_number(*p.y(), self.height() as i64),
        )
    }
}

impl<T> Display for DenseGrid<T>
where
    T: Display + Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.rows_iter() {
            for tile in line {
                f.write_fmt(format_args!("{tile}"))?
            }
            f.write_str("\n")?
        }
        Ok(())
    }
}
