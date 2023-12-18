use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use advent_of_code::utils::dense_grid::{DOWN, LEFT, RIGHT, UP};
use advent_of_code::utils::geometry::XY;

advent_of_code::solution!(18);

struct Move {
    direction: XY,
    amount: i64,
}

struct Segment {
    from: XY,
    to: XY,
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
}

pub fn part_one(input: &str) -> Option<i64> {
    let moves = input
        .split("\n")
        .map(str::trim)
        .flat_map(Move::parse)
        .collect_vec();

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
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
