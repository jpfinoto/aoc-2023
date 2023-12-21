use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use pathfinding::prelude::dijkstra_all;

use advent_of_code::utils::dense_grid::DenseGrid;
use advent_of_code::utils::geometry::XY;

advent_of_code::solution!(21);

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Ground,
    Rock,
    Start,
}

fn parse(input: &str) -> DenseGrid<Tile> {
    DenseGrid::parse(
        input,
        |c| match c {
            '#' => Tile::Rock,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!(),
        },
        None,
    )
}

fn find_target_steps(tiles: &DenseGrid<Tile>, target_steps: usize) -> HashSet<XY> {
    let start = tiles.find_one(&Tile::Start).expect("no start tile");
    let mut reachable = HashSet::from([start]);

    for _step in 1..=target_steps {
        let current_reachable = reachable
            .iter()
            .map(|p| {
                tiles
                    .cardinal_neighbours(p)
                    .filter_map(|(p, tile)| match tile? {
                        Tile::Ground => Some(p),
                        Tile::Rock => None,
                        Tile::Start => Some(p),
                    })
            })
            .flatten();
        reachable = HashSet::from_iter(current_reachable);
    }

    reachable
}

fn neighbours_wrap(p: &XY, tiles: &DenseGrid<Tile>) -> Vec<(XY, i64)> {
    tiles
        .cardinal_neighbours_with_wrapping(p)
        .filter_map(|(p, tile)| match tile? {
            Tile::Ground => Some((p, 1i64)),
            Tile::Rock => None,
            Tile::Start => Some((p, 1i64)),
        })
        .collect_vec()
}

fn can_reach_odd(
    target: &XY,
    current_distance: i64,
    target_steps: i64,
    even_reachable: &HashSet<&XY>,
    tiles: &DenseGrid<Tile>,
) -> Option<(XY, i64)> {
    // for the odd ones, we look at all even tiles
    // and see if any of them can reach the target in an even number of steps remaining
    let mut d = dijkstra_all(target, |p| neighbours_wrap(p, tiles));
    d.insert(*target, (*target, 0));

    let best_tile = d
        .iter()
        .filter_map(|(p, (_, cost))| {
            let total_distance = current_distance + cost + 1;
            let remaining_steps = target_steps - total_distance;

            if even_reachable.contains(p) && remaining_steps >= 0 && remaining_steps % 2 == 0 {
                Some((*p, cost + 1))
            } else {
                None
            }
        })
        .sorted_by(|(_, a), (_, b)| a.cmp(b))
        .rev()
        .next();

    best_tile
}

pub fn part_one(input: &str) -> Option<usize> {
    let tiles = parse(input);

    Some(find_target_steps(&tiles, 64).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    // TODO this doesn't actually work, the wrap around simplification is wrong

    let tiles = parse(input);
    let start = tiles.find_one(&Tile::Start).expect("no start tile");
    let target_steps = 10i64;
    let mut d = dijkstra_all(&start, |p| neighbours_wrap(p, &tiles));
    d.insert(start, (start, 0));

    // if the remaining number of steps when you reach a tile for the first time is even you can always reach it
    // this is because you can keep going between it and an adjacent tile indefinitely
    let even_reachable: HashSet<&XY> = HashSet::from_iter(d.iter().filter_map(|(p, (_, dist))| {
        if *dist <= target_steps && (target_steps - dist) % 2 == 0 {
            Some(p)
        } else {
            None
        }
    }));

    let reachable: HashMap<&XY, i64> = HashMap::from_iter(d.iter().filter_map(|(p, (_, dist))| {
        if even_reachable.contains(p) {
            println!("{p} in {dist} (even)");
            Some((p, target_steps / dist))
        } else if let Some((even_proxy, cost)) =
            can_reach_odd(p, *dist, target_steps, &even_reachable, &tiles)
        {
            let total_distance = dist + cost;
            println!("{p} in {total_distance} through {even_proxy} (odd)");
            Some((p, (target_steps - cost) / dist))
        } else {
            // println!("{p} -");
            None
        }
    }));

    println!("Total: {}", reachable.len());

    // Some(reachable.len())
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16733044));
    }
}
