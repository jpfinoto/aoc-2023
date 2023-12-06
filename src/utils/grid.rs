use itertools::Itertools;

pub trait WithNeighbours<P> {
    fn neighbours(&self) -> Vec<P>;
}

pub trait BoundedArea<P> {
    fn contains(&self, point: &P) -> bool;
}

pub trait DiscreetInterior<P> {
    fn interior(&self) -> Vec<P>;
}

pub trait Growable<P> {
    fn grow(&self, amount: P) -> Self;
}

pub trait Cellular {
    fn cell(&self) -> &GridCell;
}

pub type Point2Di = (i32, i32);

#[derive(Eq, PartialEq, Debug)]
pub struct GridCell {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl GridCell {
    fn intersects(&self, other: &GridCell) -> bool {
        self.left <= other.right && self.right >= other.left && self.top <= other.bottom && self.bottom >= other.top
    }
}

impl WithNeighbours<Point2Di> for GridCell {
    fn neighbours(&self) -> Vec<Point2Di> {
        let top_edge = (self.left - 1..=self.right + 1).map(|i| (self.top - 1, i));
        let bottom_edge = (self.left - 1..=self.right + 1).map(|i| (self.bottom + 1, i));
        let left_edge = (self.top..=self.bottom).map(|i| (self.left - 1, i));
        let right_edge = (self.top..=self.bottom).map(|i| (self.right + 1, i));

        Vec::from_iter(top_edge.chain(bottom_edge).chain(left_edge).chain(right_edge))
    }
}


impl BoundedArea<Point2Di> for GridCell {
    fn contains(&self, &(row, col): &Point2Di) -> bool {
        row >= self.top && row <= self.bottom && col >= self.left && col <= self.right
    }
}

impl DiscreetInterior<Point2Di> for GridCell {
    fn interior(&self) -> Vec<Point2Di> {
        (self.left..=self.right).cartesian_product(self.top..=self.bottom).collect()
    }
}

impl Growable<Point2Di> for GridCell {
    fn grow(&self, (lr_grow, td_grow): Point2Di) -> Self {
        let left_grow = ((lr_grow as f64) / 2.0).floor() as i32;
        let top_grow = ((td_grow as f64) / 2.0).floor() as i32;

        GridCell {
            left: self.left - left_grow,
            right: self.right + lr_grow - left_grow,
            top: self.top - top_grow,
            bottom: self.bottom + td_grow - top_grow,
        }
    }
}


pub fn find_neighbours<'a, I, G, C>(item: &I, grid: &'a Vec<&G>) -> Vec<&'a G>
    where
        I: WithNeighbours<C>,
        G: BoundedArea<C>,
{
    let neighbour_points = item.neighbours();

    grid
        .into_iter()
        .filter(
            |&&s| neighbour_points.iter().any(|p| s.contains(p)))
        .cloned()
        .collect()
}

pub fn find_intersections<'a, G: Cellular>(item: &GridCell, grid: &'a Vec<&G>) -> Vec<&'a G> {
    grid
        .into_iter()
        .filter(|&&s| s.cell().intersects(item))
        .cloned()
        .collect()
}

pub fn has_intersections<'a, G>(item: &GridCell, grid: &'a Vec<&G>) -> bool
    where G: Cellular
{
    grid.iter().any(|s| s.cell().intersects(item))
}