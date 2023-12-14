use itertools::Itertools;

use advent_of_code::utils::dense_grid::DenseGrid;

advent_of_code::solution!(14);

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
enum ReflectorTile {
    OBSTACLE,
    ROCK,
    GROUND,
}

fn parse(input: &str) -> DenseGrid<ReflectorTile> {
    DenseGrid::parse(
        input,
        |c| match c {
            'O' => ReflectorTile::ROCK,
            '#' => ReflectorTile::OBSTACLE,
            '.' => ReflectorTile::GROUND,
            _ => panic!("Invalid tile"),
        },
        Some(ReflectorTile::OBSTACLE),
    )
}

fn move_all_the_way(v: &Vec<&ReflectorTile>) -> Vec<ReflectorTile> {
    let mut result = vec![];

    for (i, tile) in v.iter().enumerate() {
        match tile {
            ReflectorTile::ROCK => result.push(ReflectorTile::ROCK),
            ReflectorTile::OBSTACLE => result.extend(
                [ReflectorTile::GROUND]
                    .repeat(i - result.len())
                    .iter()
                    .chain([ReflectorTile::OBSTACLE].iter()),
            ),
            ReflectorTile::GROUND => {}
        }
    }

    result.extend([ReflectorTile::GROUND].repeat(v.len() - result.len()));

    result
}

fn move_north(grid: &DenseGrid<ReflectorTile>) -> DenseGrid<ReflectorTile> {
    let moved = grid
        .columns_iter()
        .map(|col| move_all_the_way(&col))
        .collect_vec();

    DenseGrid::from_columns(&moved, Some(ReflectorTile::OBSTACLE)).expect("failed to assemble grid")
}

fn move_south(grid: &DenseGrid<ReflectorTile>) -> DenseGrid<ReflectorTile> {
    let moved = grid
        .columns_iter()
        .map(|col| col.iter().rev().cloned().collect_vec())
        .map(|col| move_all_the_way(&col))
        .map(|col| col.iter().rev().cloned().collect_vec())
        .collect_vec();

    DenseGrid::from_columns(&moved, Some(ReflectorTile::OBSTACLE)).expect("failed to assemble grid")
}

fn move_west(grid: &DenseGrid<ReflectorTile>) -> DenseGrid<ReflectorTile> {
    let moved = grid
        .rows_iter()
        .map(|col| move_all_the_way(&col.iter().collect_vec()))
        .collect_vec();

    DenseGrid::from_rows(&moved, Some(ReflectorTile::OBSTACLE)).expect("failed to assemble grid")
}

fn move_east(grid: &DenseGrid<ReflectorTile>) -> DenseGrid<ReflectorTile> {
    let moved = grid
        .rows_iter()
        .map(|col| col.iter().rev().cloned().collect_vec())
        .map(|col| move_all_the_way(&col.iter().collect_vec()))
        .map(|col| col.iter().rev().cloned().collect_vec())
        .collect_vec();

    DenseGrid::from_rows(&moved, Some(ReflectorTile::OBSTACLE)).expect("failed to assemble grid")
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
                ReflectorTile::OBSTACLE => 0,
                ReflectorTile::ROCK => len - i,
                ReflectorTile::GROUND => 0,
            })
            .sum::<usize>()
    }
}

#[allow(dead_code)]
fn print_grid(grid: &DenseGrid<ReflectorTile>) {
    grid.rows_iter().for_each(|r| {
        r.iter().for_each(|tile| match tile {
            ReflectorTile::OBSTACLE => print!("#"),
            ReflectorTile::ROCK => print!("O"),
            ReflectorTile::GROUND => print!("."),
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
    let mut repetitions = vec![];
    let target_cycles = 1000000000usize;

    loop {
        let new_grid = cycle_moves.iter().fold(grid.clone(), |g, cb| cb(&g));

        repetitions.push(
            new_grid
                .columns_iter()
                .map(|v| v.calc_load())
                .sum::<usize>(),
        );

        if let Some((offset, cycle)) = try_find_cycle(&repetitions) {
            return Some(cycle[(target_cycles - offset - 1) % cycle.len()]);
        }

        grid = new_grid;
    }
}

fn try_find_cycle(repetitions: &Vec<usize>) -> Option<(usize, Vec<usize>)> {
    for cycle_len in 2..1000 {
        let a = repetitions.iter().rev().take(cycle_len).collect_vec();

        let b = repetitions
            .iter()
            .rev()
            .skip(cycle_len)
            .take(cycle_len)
            .collect_vec();

        if a != b {
            continue;
        }

        let cycle = a.iter().rev().cloned().collect_vec();
        let steps = repetitions.len();

        // println!(
        //     "Found cycle: {cycle:?} of length {:?} after {steps} steps",
        //     cycle.len()
        // );

        for j in 0..steps / 2 {
            let local_slice = repetitions.iter().skip(j).take(cycle_len).collect_vec();

            if local_slice == cycle {
                // println!("Cycle starts at {j}");
                return Some((j, cycle.into_iter().cloned().collect()));
            }
        }
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
