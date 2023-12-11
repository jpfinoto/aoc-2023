use std::collections::{HashMap, HashSet};
use std::ops::Range;

use itertools::Itertools;

use advent_of_code::utils::geometry::XY;

advent_of_code::solution!(10);

struct DenseGrid<T> {
    width: usize,
    filler: Option<T>,
    items: Vec<T>,
}

#[derive(Eq, PartialEq)]
enum Direction {
    UpDown,
    LeftRight,
    Corner,
}

const UP: XY = XY(0, -1);
const DOWN: XY = XY(0, 1);
const LEFT: XY = XY(-1, 0);
const RIGHT: XY = XY(1, 0);

#[derive(Debug, Eq, PartialEq)]
enum Pipe {
    TwoWay(XY, XY),
    Empty,
    Start,
}

fn parse_pipe(c: char) -> Pipe {
    match c {
        '|' => Pipe::TwoWay(UP, DOWN),
        '-' => Pipe::TwoWay(LEFT, RIGHT),
        'L' => Pipe::TwoWay(UP, RIGHT),
        'J' => Pipe::TwoWay(UP, LEFT),
        '7' => Pipe::TwoWay(DOWN, LEFT),
        'F' => Pipe::TwoWay(DOWN, RIGHT),
        '.' => Pipe::Empty,
        'S' => Pipe::Start,
        _ => panic!("Invalid char {c}")
    }
}

impl<T> DenseGrid<T> {
    fn parse(block: &str, cell_parser: fn(c: char) -> T, filler: Option<T>) -> DenseGrid<T> {
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

    fn get(&self, xy: XY) -> Option<&T> {
        let (x, y) = xy.as_tuple();
        if x < 0 || y < 0 {
            self.filler.as_ref()
        } else {
            let index = y * (self.width as i64) + x;
            self.items.get(index as usize).or(self.filler.as_ref())
        }
    }

    fn find(&self, predicate: fn(item: &T) -> bool) -> Option<(&T, XY)> {
        self
            .items
            .iter()
            .enumerate()
            .find(|(_, item)| predicate(*item))
            .and_then(|(i, item)| {
                Some((item, self.index_to_xy(i)))
            })
    }

    fn index_to_xy(&self, idx: usize) -> XY {
        XY((idx % self.width) as i64, (idx / self.width) as i64)
    }

    fn height(&self) -> usize {
        self.items.len() / self.width
    }

    #[allow(dead_code)]
    fn rows_iter(&self) -> impl Iterator<Item=&[T]> {
        (0..self.height())
            .map(move |y| &self.items[y * self.width..(y + 1) * self.width])
    }
}

fn find_cycle(start_xy: XY, grid: &DenseGrid<Pipe>) -> (u32, HashMap<XY, Direction>) {
    let mut boundary_pipes = HashMap::new();

    for start_dir in [UP, DOWN, LEFT, RIGHT] {
        boundary_pipes.clear();

        let mut steps = 1u32;
        let mut xy = start_xy + start_dir;
        let mut prev_xy = start_xy;

        while let Some(pipe) = grid.get(xy) {
            match pipe {
                Pipe::TwoWay(a, b) => {
                    let dest_a = xy + *a;
                    let dest_b = xy + *b;

                    let next_step = if dest_a == prev_xy {
                        dest_b
                    } else if dest_b == prev_xy {
                        dest_a
                    } else {
                        // entered from the wrong direction
                        break;
                    };

                    let pipe_direction = match (a, b) {
                        (&UP, &DOWN) => Direction::UpDown,
                        (&LEFT, &RIGHT) => Direction::LeftRight,
                        _ => Direction::Corner,
                    };

                    prev_xy = xy;
                    xy = next_step;

                    boundary_pipes.insert(xy, pipe_direction);
                }
                Pipe::Empty => break,
                Pipe::Start => return (steps, boundary_pipes),
            };

            steps += 1;
        }
    }

    panic!("Couldn't find route");
}

fn get_odd(boundary: &HashMap<XY, Direction>, row_first: bool, x_range: Range<i32>, y_range: Range<i32>) -> HashSet<XY> {
    // doesn't work lol

    let mut inner_candidates = HashSet::new();

    let (first_range, second_range) = if row_first {
        (y_range, x_range)
    } else {
        (x_range, y_range)
    };

    let filter_type = if row_first {
        Direction::UpDown
    } else {
        Direction::LeftRight
    };

    // println!("row first? {row_first} check ranges first={:?} second={:?}", first_range, second_range);

    for first in first_range.clone() {
        let mut boundary_crossings = 0usize;
        let mut batch_candidates = HashSet::new();
        let mut is_crossing = false;

        for second in second_range.clone() {
            let p = if row_first {
                XY(second as i64, first as i64)
            } else {
                XY(first as i64, second as i64)
            };

            if let Some(boundary_dir) = boundary.get(&p) {
                inner_candidates.extend(batch_candidates.iter());
                batch_candidates.clear();

                if boundary_dir == &filter_type || boundary_dir == &Direction::Corner {
                    boundary_crossings += 1;
                    is_crossing = false;

                    // print!("*");
                } else if !is_crossing {
                    // boundary_crossings += 1;
                    is_crossing = true;

                    // print!("!");
                } else {
                    // print!("?");
                }
            } else {
                is_crossing = false;
                if boundary_crossings % 2 == 1 {
                    // print!("I");
                    batch_candidates.insert(p);
                } else {
                    // print!("O");
                }
            }
        }
        // println!();
    }

    inner_candidates
}

#[allow(dead_code)]
fn print_grid(boundary: &HashMap<XY, Direction>, inner: &HashSet<XY>, w: i32, h: i32) {
    for y in 0..h {
        for x in 0..w {
            let p = XY(x as i64, y as i64);
            print!("{}", match (boundary.contains_key(&p), inner.contains(&p)) {
                (true, false) => '*',
                (false, true) => 'I',
                (false, false) => '.',
                (true, true) => 'X',
            });
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = DenseGrid::parse(input, parse_pipe, Some(Pipe::Empty));
    let Some((_, start_xy)) = grid.find(|pipe| *pipe == Pipe::Start) else { panic!() };

    let (cycle_length, _) = find_cycle(start_xy, &grid);

    Some(cycle_length / 2)
}

#[allow(unused)]
pub fn part_two(input: &str) -> Option<u32> {
    let grid = DenseGrid::parse(input, parse_pipe, Some(Pipe::Empty));
    let Some((_, start_xy)) = grid.find(|pipe| *pipe == Pipe::Start) else { panic!() };

    let (_, boundary) = find_cycle(start_xy, &grid);
    let row_wise_candidates = get_odd(
        &boundary,
        true,
        0..(grid.width as i32),
        0..(grid.height() as i32),
    );

    let col_wise_candidates = get_odd(
        &boundary,
        false,
        0..(grid.width as i32),
        0..(grid.height() as i32),
    );

    // let inside = HashSet::from_iter(row_wise_candidates.iter().filter(|p| col_wise_candidates.contains(p)).cloned());
    //
    // println!("Row candidates:");
    // print_grid(&boundary, &row_wise_candidates, grid.width as i32, grid.height() as i32);
    // println!("Col candidates:");
    // print_grid(&boundary, &col_wise_candidates, grid.width as i32, grid.height() as i32);
    // println!("Inner:");
    // print_grid(&boundary, &inside, grid.width as i32, grid.height() as i32);

    let total = row_wise_candidates.iter().filter(|p| col_wise_candidates.contains(p)).count();

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
