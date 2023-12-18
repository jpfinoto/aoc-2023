use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use advent_of_code::utils::dense_grid::{DOWN, LEFT, ORIGIN, RIGHT, UP};
use advent_of_code::utils::geometry::{shoelace_area, XY};

advent_of_code::solution!(18);

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

    fn from_tuple((direction, amount): (XY, i64)) -> Self {
        Move { direction, amount }
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
}

fn get_points(moves: &[Move]) -> Vec<XY> {
    let mut points = vec![];

    for m in moves.into_iter() {
        points.push(m.direction * m.amount + points.last().or(Some(&ORIGIN)).unwrap())
    }

    points
}

fn calc_inner_area(moves: &[Move]) -> i64 {
    let perimeter: i64 = moves.iter().map(|m| m.amount).sum();
    let points = get_points(moves);
    let shoelace_area = shoelace_area(&points) as i64;

    shoelace_area + perimeter / 2 + 1
}

pub fn part_one(input: &str) -> Option<i64> {
    let moves = input
        .split("\n")
        .map(str::trim)
        .flat_map(Move::parse)
        .collect_vec();

    let area = calc_inner_area(&moves);

    Some(area)
}

pub fn part_two(input: &str) -> Option<i64> {
    let moves = input
        .split("\n")
        .map(str::trim)
        .flat_map(Move::parse_from_color)
        .collect_vec();

    let area = calc_inner_area(&moves);

    Some(area)
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
        assert_eq!(result, Some(952408144115));
    }

    #[test]
    fn test_area() {
        assert_eq!(
            calc_inner_area(&[(RIGHT, 1), (DOWN, 1), (LEFT, 1), (UP, 1)].map(Move::from_tuple)),
            4
        );

        assert_eq!(
            calc_inner_area(&[(RIGHT, 9), (DOWN, 9), (LEFT, 9), (UP, 9)].map(Move::from_tuple)),
            100
        );

        assert_eq!(
            calc_inner_area(
                &[
                    (RIGHT, 9),
                    (DOWN, 9),
                    (LEFT, 3),
                    (UP, 3),
                    (LEFT, 3),
                    (DOWN, 3),
                    (LEFT, 3),
                    (UP, 9)
                ]
                .map(Move::from_tuple)
            ),
            94
        );
    }
}
