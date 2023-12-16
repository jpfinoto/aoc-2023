use itertools::Itertools;
use std::collections::HashMap;

use advent_of_code::utils::dense_grid::DenseGrid;

advent_of_code::solution!(14);

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
enum ReflectorTile {
    Obstacle,
    Rock,
    Ground,
}

fn parse(input: &str) -> DenseGrid<ReflectorTile> {
    DenseGrid::parse(
        input,
        |c| match c {
            'O' => ReflectorTile::Rock,
            '#' => ReflectorTile::Obstacle,
            '.' => ReflectorTile::Ground,
            _ => panic!("Invalid tile"),
        },
        Some(ReflectorTile::Obstacle),
    )
}

fn move_all_the_way(v: &Vec<&ReflectorTile>) -> Vec<ReflectorTile> {
    let mut result = vec![];

    for (i, tile) in v.iter().enumerate() {
        match tile {
            ReflectorTile::Rock => result.push(ReflectorTile::Rock),
            ReflectorTile::Obstacle => result.extend(
                [ReflectorTile::Ground]
                    .repeat(i - result.len())
                    .iter()
                    .chain([ReflectorTile::Obstacle].iter()),
            ),
            ReflectorTile::Ground => {}
        }
    }

    result.extend([ReflectorTile::Ground].repeat(v.len() - result.len()));

    result
}

fn move_north(grid: &DenseGrid<ReflectorTile>) -> DenseGrid<ReflectorTile> {
    let moved = grid
        .columns_iter()
        .map(|col| move_all_the_way(&col))
        .collect_vec();

    DenseGrid::from_columns(&moved, Some(ReflectorTile::Obstacle)).expect("failed to assemble grid")
}

fn move_south(grid: &DenseGrid<ReflectorTile>) -> DenseGrid<ReflectorTile> {
    let moved = grid
        .columns_iter()
        .map(|col| col.iter().rev().cloned().collect_vec())
        .map(|col| move_all_the_way(&col))
        .map(|col| col.iter().rev().cloned().collect_vec())
        .collect_vec();

    DenseGrid::from_columns(&moved, Some(ReflectorTile::Obstacle)).expect("failed to assemble grid")
}

fn move_west(grid: &DenseGrid<ReflectorTile>) -> DenseGrid<ReflectorTile> {
    let moved = grid
        .rows_iter()
        .map(|col| move_all_the_way(&col.iter().collect_vec()))
        .collect_vec();

    DenseGrid::from_rows(&moved, Some(ReflectorTile::Obstacle)).expect("failed to assemble grid")
}

fn move_east(grid: &DenseGrid<ReflectorTile>) -> DenseGrid<ReflectorTile> {
    let moved = grid
        .rows_iter()
        .map(|col| col.iter().rev().cloned().collect_vec())
        .map(|col| move_all_the_way(&col.iter().collect_vec()))
        .map(|col| col.iter().rev().cloned().collect_vec())
        .collect_vec();

    DenseGrid::from_rows(&moved, Some(ReflectorTile::Obstacle)).expect("failed to assemble grid")
}

trait CalcLoad {
    fn calc_load(&self) -> usize;
}

impl CalcLoad for Vec<ReflectorTile> {
    fn calc_load(&self) -> usize {
        self.iter().collect_vec().calc_load()
    }
}

impl CalcLoad for Vec<&ReflectorTile> {
    fn calc_load(&self) -> usize {
        let len = self.len();

        self.iter()
            .enumerate()
            .map(|(i, tile)| match tile {
                ReflectorTile::Obstacle => 0,
                ReflectorTile::Rock => len - i,
                ReflectorTile::Ground => 0,
            })
            .sum::<usize>()
    }
}

impl CalcLoad for DenseGrid<ReflectorTile> {
    fn calc_load(&self) -> usize {
        self.columns_iter().map(|v| v.calc_load()).sum()
    }
}

#[allow(dead_code)]
fn print_grid(grid: &DenseGrid<ReflectorTile>) {
    grid.rows_iter().for_each(|r| {
        r.iter().for_each(|tile| match tile {
            ReflectorTile::Obstacle => print!("#"),
            ReflectorTile::Rock => print!("O"),
            ReflectorTile::Ground => print!("."),
        });
        println!();
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let sum = parse(input)
        .columns_iter()
        .map(|v| move_all_the_way(&v))
        .map(|v| v.calc_load())
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let cycle_moves = [move_north, move_west, move_south, move_east];
    let mut grid = parse(input);
    let target_cycles = 1000000000usize;
    let mut last_seen_grids = HashMap::new();
    last_seen_grids.insert(grid.clone(), 0usize);

    for i in 1usize.. {
        let new_grid = cycle_moves.iter().fold(grid.clone(), |g, cb| cb(&g));

        if let Some(last_grid_iter) = last_seen_grids.get(&new_grid) {
            let cycle_length = i - last_grid_iter;
            let target_grid_index = (target_cycles - cycle_length) % cycle_length;

            let Some((final_grid, _)) = last_seen_grids
                .iter()
                .find(|(_, &index)| index == target_grid_index)
            else {
                panic!("Where's my grid?")
            };

            return Some(final_grid.calc_load());
        }

        grid = new_grid.clone();
        last_seen_grids.insert(new_grid, i);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
