pub trait Gridded<T> {
    fn neighbours(&self) -> Vec<T>;
}

pub trait BoundedArea<T> {
    fn contains(&self, point: &T) -> bool;
}


pub struct GridCell {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}


impl Gridded<(i32, i32)> for GridCell {
    fn neighbours(&self) -> Vec<(i32, i32)> {
        let top_edge = (self.left - 1..=self.right + 1).map(|i| (self.top - 1, i));
        let bottom_edge = (self.left - 1..=self.right + 1).map(|i| (self.bottom + 1, i));
        let left_edge = (self.top..=self.bottom).map(|i| (self.left - 1, i));
        let right_edge = (self.top..=self.bottom).map(|i| (self.right + 1, i));

        Vec::from_iter(top_edge.chain(bottom_edge).chain(left_edge).chain(right_edge))
    }
}


impl BoundedArea<(i32, i32)> for GridCell {
    fn contains(&self, &(row, col): &(i32, i32)) -> bool {
        row >= self.top && row <= self.bottom && col >= self.left && col <= self.right
    }
}


pub fn find_neighbours<'a, I, G, C>(item: &I, grid: &'a Vec<&G>) -> Vec<&'a G>
    where
        I: Gridded<C>,
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