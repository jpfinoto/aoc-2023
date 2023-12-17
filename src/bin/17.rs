use std::fmt::{Debug, Formatter};

use itertools::Itertools;
use pathfinding::prelude::astar;

use advent_of_code::utils::dense_grid::{DenseGrid, DOWN, LEFT, RIGHT, UP};
use advent_of_code::utils::geometry::XY;

advent_of_code::solution!(17);

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct CrucibleConfig {
    min_moves: i64,
    max_moves: i64,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Node {
    config: CrucibleConfig,
    enter: XY,
    exit: XY,
    loss: i64,
    direction: XY,
    is_start: bool,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let enter = &self.enter;
        let exit = &self.exit;
        let dir = match self.direction {
            UP => "U",
            DOWN => "D",
            RIGHT => "R",
            LEFT => "L",
            _ => panic!(),
        };
        let loss = self.loss;

        f.write_fmt(format_args!("Node {dir} loss {loss} {enter} -> {exit}"))
    }
}

trait NextNodes {
    fn next_nodes(&self, board: &DenseGrid<i64>) -> Vec<Node>;
}

impl NextNodes for Node {
    fn next_nodes(&self, board: &DenseGrid<i64>) -> Vec<Node> {
        let mut nodes = vec![];

        if self.is_start {
            for dir in [UP, DOWN, LEFT, RIGHT] {
                nodes.extend(
                    Node {
                        config: self.config,
                        enter: self.enter,
                        exit: self.enter,
                        loss: 0,
                        direction: dir,
                        is_start: false,
                    }
                    .next_nodes(board),
                )
            }
        } else {
            for new_dir in [self.direction.turn_left(), self.direction.turn_right()] {
                let start = self.exit + new_dir;
                for moves in (self.config.min_moves - 1)..self.config.max_moves {
                    let end = start + new_dir * moves;
                    let loss = board
                        .rect_range_inclusive(start, end)
                        .flat_map(|(_, loss)| loss)
                        .sum();
                    if let Some(_) = board.get(end) {
                        let new_node = Node {
                            config: self.config,
                            enter: start,
                            exit: end,
                            loss,
                            direction: new_dir,
                            is_start: false,
                        };
                        nodes.push(new_node)
                    }
                }
            }
        }

        nodes
    }
}

fn find_shortest_path(
    boards: &DenseGrid<i64>,
    start: XY,
    target: XY,
    config: CrucibleConfig,
) -> Option<(Vec<Node>, i64)> {
    let start_node = Node {
        config,
        enter: start,
        exit: start,
        loss: 0,
        direction: XY(0, 0),
        is_start: true,
    };

    let Some((path, loss)) = astar(
        &start_node,
        |node| {
            node.next_nodes(boards)
                .iter()
                .map(|&node| (node, node.loss))
                .collect_vec()
        },
        |node| (node.exit - target).manhattan_dist(),
        |node| node.exit == target,
    ) else {
        return None;
    };

    Some((path, loss))
}

fn parse(input: &str) -> DenseGrid<i64> {
    DenseGrid::parse(input, |c| c.to_digit(10).unwrap() as i64, None)
}

#[allow(dead_code)]
fn print_path(grid: &DenseGrid<i64>, path: &Vec<Node>) {
    let mut output_grid = DenseGrid::new_filled(grid.width, grid.height(), '?', None);

    for (coord, el) in grid.range(0..(grid.width as i64), 0..(grid.height() as i64)) {
        if let Some(loss) = el {
            output_grid.set_if_inbounds(coord, loss.to_string().chars().next().unwrap());
        }
    }

    for node in path {
        let dir_char = match node.direction {
            UP => '^',
            DOWN => 'v',
            RIGHT => '>',
            LEFT => '<',
            _ => '!',
        };

        for xy in node.enter.rect_range_inclusive(node.exit) {
            output_grid.set_if_inbounds(xy.clone(), dir_char);
        }
    }

    println!("{output_grid}");
}

pub fn part_one(input: &str) -> Option<i64> {
    let grid = parse(input);
    let start_pos = XY(0, 0);
    let exit_pos = XY((grid.width - 1) as i64, (grid.height() - 1) as i64);

    let (_, loss) = find_shortest_path(
        &grid,
        start_pos,
        exit_pos,
        CrucibleConfig {
            min_moves: 1,
            max_moves: 3,
        },
    )?;

    // print_path(&grid, &path);

    Some(loss)
}

pub fn part_two(input: &str) -> Option<i64> {
    let grid = parse(input);
    let start_pos = XY(0, 0);
    let exit_pos = XY((grid.width - 1) as i64, (grid.height() - 1) as i64);

    let (_, loss) = find_shortest_path(
        &grid,
        start_pos,
        exit_pos,
        CrucibleConfig {
            min_moves: 4,
            max_moves: 10,
        },
    )?;

    // print_path(&grid, &path);

    Some(loss)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_starting_node() {
        let board = DenseGrid::new_filled(10, 10, 1i64, None);

        let starting_node = Node {
            config: CrucibleConfig {
                min_moves: 1,
                max_moves: 3,
            },
            enter: XY(0, 0),
            exit: XY(0, 0),
            loss: 0,
            direction: XY(0, 0),
            is_start: true,
        };

        for node in starting_node.next_nodes(&board) {
            println!("{node:?}");
        }
    }
}
