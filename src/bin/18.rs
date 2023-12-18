use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use tqdm::{tqdm, Iter};

use advent_of_code::utils::dense_grid::{DOWN, LEFT, RIGHT, UP};
use advent_of_code::utils::geometry;
use advent_of_code::utils::geometry::XY;
use advent_of_code::utils::sparse_grid::SparseGrid;

advent_of_code::solution!(18);

#[derive(Debug)]
struct Color {
    value: u32,
}

#[derive(Debug)]
struct Move {
    direction: XY,
    amount: i64,
}

lazy_static! {
    static ref MOVE_RE: Regex =
        Regex::new(r"^(?P<dir>[UDLR]) (?P<moves>\d+) \(#(?P<color>\w+)\)").unwrap();
}

impl Move {
    fn parse(line: &str) -> Option<Move> {
        let captures = MOVE_RE.captures(line)?;

        Some(Move {
            direction: match &captures["dir"] {
                "U" => UP,
                "D" => DOWN,
                "L" => LEFT,
                "R" => RIGHT,
                _ => panic!(),
            },
            amount: i64::from_str(&captures["moves"]).expect("invalid number"),
        })
    }

    fn parse_from_color(line: &str) -> Option<Move> {
        let captures = MOVE_RE.captures(line)?;
        let color = i64::from_str_radix(&captures["color"], 16).expect("invalid number");

        Some(Move {
            direction: match color & 0xF {
                0 => RIGHT,
                1 => DOWN,
                2 => LEFT,
                3 => UP,
                _ => panic!(),
            },
            amount: color >> 4,
        })
    }

    fn main_direction(&self) -> geometry::Direction {
        match self.direction {
            UP => geometry::Direction::UpDown,
            DOWN => geometry::Direction::UpDown,
            LEFT => geometry::Direction::LeftRight,
            RIGHT => geometry::Direction::LeftRight,
            _ => panic!(),
        }
    }
}

fn compute_boundary(moves: &Vec<Move>) -> (HashMap<XY, geometry::Direction>, SparseGrid<&Move>) {
    let mut p = XY(0, 0);
    let mut prev_move = moves.last();
    let mut boundary: HashMap<XY, geometry::Direction> = HashMap::new();
    let mut map = SparseGrid::new(None);

    for m in moves.iter().tqdm() {
        let main_direction = m.main_direction();

        let first_tile_dir = match prev_move
            .and_then(|p| Some(p.direction.cross_z(&m.direction)))
            .or(Some(0))
            .unwrap()
        {
            0 => main_direction,
            num => geometry::Direction::Corner(num),
        };

        // println!("Move is {m:?}, main dir is {main_direction:?} first dir is {first_tile_dir:?}");

        boundary.insert(p, first_tile_dir);
        map.insert(p, m);
        for _ in 1..=m.amount {
            p = p + m.direction;
            boundary.insert(p, main_direction);
        }

        prev_move = Some(m);
    }

    (boundary, map)
}

pub fn part_one(input: &str) -> Option<usize> {
    let moves = input
        .split("\n")
        .map(str::trim)
        .flat_map(Move::parse)
        .collect_vec();

    let (boundary, map) = compute_boundary(&moves);

    let inner = geometry::get_odd(
        &boundary,
        map.get_lower_corner()
            .range_x_inclusive(map.get_upper_corner()),
        map.get_lower_corner()
            .range_y_inclusive(map.get_upper_corner()),
    );

    // geometry::print_grid(
    //     &boundary,
    //     &inner,
    //     map.get_lower_corner(),
    //     map.get_upper_corner(),
    // );

    Some(inner.len() + boundary.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    // figure out how to get the external boundary
    // use the shoelace formula to get the area

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
