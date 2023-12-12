use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;

advent_of_code::solution!(8);

enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Node {
    label: String,
    id: u32,
    left: u32,
    right: u32,
}

lazy_static! {
    static ref NODE_RE: Regex =
        Regex::new(r"^(?P<label>\w+) = \((?P<left>\w+), (?P<right>\w+)\)$").unwrap();
}

fn char_idx(c: char) -> u32 {
    c as u32
}

fn to_id(label: &str) -> u32 {
    let (a, b, c) = label.chars().collect_tuple().unwrap();

    (char_idx(a) << 16) + (char_idx(b) << 8) + char_idx(c)
}

impl Node {
    fn parse(line: &str) -> Option<Node> {
        NODE_RE.captures(line).and_then(|cap| {
            Some(Node {
                label: cap["label"].into(),
                id: to_id(&cap["label"]),
                left: to_id(&cap["left"]),
                right: to_id(&cap["right"]),
            })
        })
    }
}

fn parse(input: &str) -> (Vec<Direction>, HashMap<u32, Node>) {
    let (dir_line, _, map_block) = input.splitn(3, "\n").collect_tuple().unwrap();

    let map = HashMap::from_iter(
        map_block
            .split("\n")
            .flat_map(Node::parse)
            .map(|node| (node.id, node)),
    );

    let moves = dir_line
        .chars()
        .flat_map(|c| match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        })
        .collect_vec();

    (moves, map)
}

fn id_ends_with(id: u32, c: char) -> bool {
    (id & 0xff) == char_idx(c)
}

fn get_cycle(
    moves: &Vec<Direction>,
    map: &HashMap<u32, Node>,
    node: &Node,
    target_cond: fn(&Node) -> bool,
) -> u64 {
    let mut current_node = node;
    let total_moves = moves.len();
    let mut last_cycle = 0u64;

    for (i, dir) in moves.iter().cycle().enumerate() {
        if target_cond(current_node) && i % total_moves == 0 {
            last_cycle = i as u64;
            break;
        }

        match dir {
            Direction::Left => current_node = &map[&current_node.left],
            Direction::Right => current_node = &map[&current_node.right],
        }
    }

    last_cycle
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (moves, map) = parse(input);
    let target = to_id("ZZZ");

    let mut current_node = &map[&to_id("AAA")];
    let mut steps = 0;
    for dir in moves.iter().cycle() {
        match dir {
            Direction::Left => current_node = &map[&current_node.left],
            Direction::Right => current_node = &map[&current_node.right],
        }

        steps += 1;

        if current_node.id == target {
            break;
        }
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (moves, map) = parse(input);

    let starting_nodes = map
        .values()
        .filter(|&v| id_ends_with(v.id, 'A'))
        .collect_vec();
    let cycles: Vec<u64> = starting_nodes
        .par_iter()
        .map(|node| get_cycle(&moves, &map, node, |node| id_ends_with(node.id, 'Z')))
        .collect();

    Some(cycles.into_iter().reduce(lcm).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
