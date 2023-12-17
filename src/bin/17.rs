use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

use itertools::Itertools;
use pathfinding::prelude::{build_path, dijkstra_partial};
use rayon::prelude::*;

use advent_of_code::utils::dense_grid::{DenseGrid, DOWN, LEFT, RIGHT, UP};
use advent_of_code::utils::geometry::XY;

advent_of_code::solution!(17);

#[derive(Eq, PartialEq, Hash)]
struct Node {
    id: usize,
    tiles: Vec<XY>,
    loss: i64,
    enter_direction: XY,
    exits: Vec<usize>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let id = self.id;
        let tiles = &self.tiles;
        let dir = match self.enter_direction {
            UP => "U",
            DOWN => "D",
            RIGHT => "R",
            LEFT => "L",
            _ => panic!(),
        };
        let loss = self.loss;
        let exits = &self.exits;
        let (first, last) = self.get_first_last_tile();

        f.write_fmt(format_args!(
            "Node#{id} {dir} loss {loss} into {tiles:?} -> {exits:?} // {first} -> {last}"
        ))
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Connection {
    position: XY,
    direction: XY,
}

impl Node {
    fn get_entrance(&self) -> Connection {
        let (first, _) = self.get_first_last_tile();

        Connection {
            position: first,
            direction: self.enter_direction,
        }
    }

    fn get_first_last_tile(&self) -> (XY, XY) {
        if self.tiles.len() == 1 {
            (self.tiles[0], self.tiles[0])
        } else {
            let first = *self.tiles.first().unwrap();
            let last = *self.tiles.last().unwrap();
            let delta = first - last;
            if delta.normalize() == self.enter_direction {
                (last, first)
            } else {
                (first, last)
            }
        }
    }

    fn get_exits(&self) -> Vec<Connection> {
        let (_, last) = self.get_first_last_tile();

        // you can't go out on the same line you came in (backwards or forwards)
        // going forwards multiple spaces is handled by the tile spans
        let out_directions = match self.enter_direction {
            UP => vec![LEFT, RIGHT],
            DOWN => vec![LEFT, RIGHT],
            RIGHT => vec![UP, DOWN],
            LEFT => vec![UP, DOWN],
            _ => panic!(),
        };

        out_directions
            .iter()
            .map(|dir| Connection {
                position: last + *dir,
                direction: *dir,
            })
            .collect()
    }
}

/// Generates all possible nodes give the min and max spans.
/// The loss is accumulated over a span.
fn generate_nodes(grid: &DenseGrid<i64>, min_span: usize, max_span: usize) -> Vec<Node> {
    // this function is terrible

    let horizontal_spans = grid
        .rows_iter()
        .enumerate()
        .par_bridge()
        .map(|(y, row)| {
            let mut nodes = vec![];
            for base_x in 0..row.len() {
                for target_len in min_span..=max_span {
                    let mut total_loss = 0i64;
                    let mut tiles = vec![];
                    for offset in 0..target_len {
                        let x = base_x + offset;
                        if let Some(loss) = row.get(x) {
                            total_loss += loss;
                            tiles.push(XY(x as i64, y as i64));
                        }
                    }

                    if tiles.len() < min_span {
                        continue;
                    }

                    if tiles.len() == 1 {
                        for enter_direction in [UP, DOWN, LEFT, RIGHT] {
                            nodes.push(Node {
                                id: 0,
                                tiles: tiles.clone(),
                                loss: total_loss,
                                enter_direction,
                                exits: vec![],
                            })
                        }
                    } else {
                        for enter_direction in [LEFT, RIGHT] {
                            nodes.push(Node {
                                id: 0,
                                tiles: tiles.clone(),
                                loss: total_loss,
                                enter_direction,
                                exits: vec![],
                            })
                        }
                    }
                }
            }

            nodes
        })
        .flatten();

    let vertical_spans = grid
        .columns_iter()
        .enumerate()
        .par_bridge()
        .map(|(x, col)| {
            let mut nodes = vec![];
            for base_y in 0..col.len() {
                for target_len in min_span..=max_span {
                    let mut total_loss = 0i64;
                    let mut tiles = vec![];
                    for offset in 0..target_len {
                        let y = base_y + offset;
                        if let Some(loss) = col.get(y) {
                            total_loss += *loss;
                            tiles.push(XY(x as i64, y as i64));
                        }
                    }
                    if tiles.len() < min_span {
                        continue;
                    }

                    if tiles.len() == 1 {
                        for enter_direction in [UP, DOWN, LEFT, RIGHT] {
                            nodes.push(Node {
                                id: 0,
                                tiles: tiles.clone(),
                                loss: total_loss,
                                enter_direction,
                                exits: vec![],
                            })
                        }
                    } else {
                        for enter_direction in [UP, DOWN] {
                            nodes.push(Node {
                                id: 0,
                                tiles: tiles.clone(),
                                loss: total_loss,
                                enter_direction,
                                exits: vec![],
                            })
                        }
                    }
                }
            }

            nodes
        })
        .flatten();

    // a lazy way to dedup the nodes
    let nodes: HashSet<Node> = horizontal_spans.chain(vertical_spans).collect();

    nodes.into_iter().collect()
}

fn compute_graph(grid: &DenseGrid<i64>, min_span: usize, max_span: usize) -> Vec<Node> {
    let mut nodes = generate_nodes(grid, min_span, max_span);

    // this map is (entrance_pos, enter_direction) -> Vec<node_id>
    let node_entrances = nodes
        .iter()
        .enumerate()
        .map(|(id, node)| (node.get_entrance(), id))
        .into_group_map();

    for (id, node) in &mut nodes.iter_mut().enumerate() {
        // so far the nodes don't know their ids, so we assign it here
        node.id = id;
        node.exits = node
            .get_exits()
            .iter()
            .flat_map(|exit| node_entrances.get(exit))
            .flatten()
            .cloned()
            .collect();
    }

    nodes
}

fn find_shortest_path(
    nodes: &Vec<Node>,
    start_node_id: usize,
    target_node_ids: &HashSet<usize>,
) -> Option<(Vec<usize>, i64)> {
    let (parents, Some(end)) = dijkstra_partial(
        &start_node_id,
        |id| {
            nodes[*id]
                .exits
                .iter()
                .map(|next_node_id| (*next_node_id, nodes[*next_node_id].loss))
        },
        |id| target_node_ids.contains(id),
    ) else {
        return None;
    };

    let path = build_path(&end, &parents);

    Some((path, parents[&end].1))
}

fn parse(input: &str) -> DenseGrid<i64> {
    DenseGrid::parse(input, |c| c.to_digit(10).unwrap() as i64, None)
}

#[allow(dead_code)]
fn print_path(grid: &DenseGrid<i64>, nodes: &Vec<Node>, path: &Vec<usize>) {
    let mut output_grid = DenseGrid::new_filled(grid.width, grid.height(), '?', None);

    for (coord, el) in grid.range(0..(grid.width as i64), 0..(grid.height() as i64)) {
        if let Some(loss) = el {
            output_grid.set_if_inbounds(coord, loss.to_string().chars().next().unwrap());
        }
    }

    for node_id in path.into_iter().skip(1) {
        let node = &nodes[*node_id];
        let dir_char = match node.enter_direction {
            UP => '^',
            DOWN => 'v',
            RIGHT => '>',
            LEFT => '<',
            _ => '!',
        };
        for xy in &node.tiles {
            output_grid.set_if_inbounds(*xy, dir_char);
        }
    }

    println!("{output_grid}");
}

fn get_min_loss(grid: &DenseGrid<i64>, nodes: &Vec<Node>, start_pos: XY, exit_pos: XY) -> i64 {
    let start_nodes = nodes
        .iter()
        .filter(|node| {
            let (first, _) = node.get_first_last_tile();
            first == start_pos
        })
        .map(|node| (node.id, node.loss - grid.get(start_pos).unwrap()))
        .collect_vec();

    let exit_nodes = nodes
        .iter()
        .filter(|node| node.get_first_last_tile().1 == exit_pos)
        .collect_vec();

    let exit_node_ids = exit_nodes.iter().map(|node| node.id).collect();

    // for node in &start_nodes {
    //     println!("> Possible entrance: {node:?}");
    // }
    //
    // for node in &exit_nodes {
    //     println!("< Possible exit: {node:?}");
    // }

    let min_loss = start_nodes
        .par_iter()
        .map(|(node_id, loss_offset)| {
            if let Some((_, loss)) = find_shortest_path(&nodes, *node_id, &exit_node_ids) {
                let total_loss = loss + loss_offset;

                // println!("! total loss: {total_loss} through path {path:?}");
                // print_path(&grid, &nodes, &path);

                total_loss
            } else {
                panic!("unreachable?")
            }
        })
        .min()
        .unwrap();

    min_loss
}

pub fn part_one(input: &str) -> Option<i64> {
    let grid = parse(input);
    let nodes = compute_graph(&grid, 1, 3);
    let start_pos = XY(0, 0);
    let exit_pos = XY((grid.width - 1) as i64, (grid.height() - 1) as i64);

    println!("Total nodes: {}", nodes.len());

    let min_loss = get_min_loss(&grid, &nodes, start_pos, exit_pos);

    Some(min_loss)
}

pub fn part_two(input: &str) -> Option<i64> {
    let grid = parse(input);
    let nodes = compute_graph(&grid, 4, 10);
    let start_pos = XY(0, 0);
    let exit_pos = XY((grid.width - 1) as i64, (grid.height() - 1) as i64);

    println!("Total nodes: {}", nodes.len());

    let min_loss = get_min_loss(&grid, &nodes, start_pos, exit_pos);

    Some(min_loss)
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
}
