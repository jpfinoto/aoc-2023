use itertools::Itertools;

use crate::utils::geometry::XY;

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
        if x < 0 || y < 0 {
            self.filler.as_ref()
        } else {
            let index = y * (self.width as i64) + x;
            self.items.get(index as usize).or(self.filler.as_ref())
        }
    }

    pub fn find(&self, predicate: fn(item: &T) -> bool) -> Option<(&T, XY)> {
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
}
