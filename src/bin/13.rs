use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(13);

#[derive(Eq, PartialEq, Debug)]
enum Tile {
    Ash,
    Rock,
}

struct MirrorArray {
    tiles: Vec<Vec<Tile>>,
}

enum Symmetry {
    None,
    Column(usize),
    Row(usize),
}

impl MirrorArray {
    fn parse_line(line: &str) -> Option<Vec<Tile>> {
        let tiles = line
            .chars()
            .flat_map(|c| match c {
                '#' => Some(Tile::Rock),
                '.' => Some(Tile::Ash),
                _ => None,
            })
            .collect_vec();

        if tiles.len() > 0 {
            Some(tiles)
        } else {
            None
        }
    }

    fn parse(input: &str) -> Vec<MirrorArray> {
        input
            .split("\n")
            .map(str::trim)
            .map(MirrorArray::parse_line)
            .group_by(|l| l.is_some())
            .into_iter()
            .filter_map(|(s, g)| {
                if s {
                    Some(MirrorArray {
                        tiles: g.flatten().collect(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_row(&self, row: usize) -> Option<Vec<&Tile>> {
        Some(self.tiles.get(row)?.iter().by_ref().collect())
    }

    fn get_col(&self, col: usize) -> Option<Vec<&Tile>> {
        self.tiles[0]
            .get(col)
            .and_then(|_| Some(self.tiles.iter().map(|row| &row[col]).collect()))
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn find_symmetrical_col(&self) -> Option<usize> {
        let len = self.width();
        (0..len - 1)
            .filter(|i| is_symmetrical(self, *i, MirrorArray::get_col))
            .next()
    }

    fn find_symmetrical_row(&self) -> Option<usize> {
        let len = self.height();
        (0..len - 1)
            .filter(|i| is_symmetrical(self, *i, MirrorArray::get_row))
            .next()
    }

    fn find_symmetry(&self) -> Symmetry {
        self.find_symmetrical_col()
            .and_then(|r| Some(Symmetry::Column(r)))
            .or_else(|| {
                self.find_symmetrical_row()
                    .and_then(|r| Some(Symmetry::Row(r)))
            })
            .or(Some(Symmetry::None))
            .unwrap()
    }
}

fn is_symmetrical(
    mirrors: &MirrorArray,
    i: usize,
    getter: fn(&MirrorArray, usize) -> Option<Vec<&Tile>>,
) -> bool {
    (0..=i).all(|j| {
        let a = getter(mirrors, i - j);
        let b = getter(mirrors, i + j + 1);

        match (a, b) {
            (Some(row_a), Some(row_b)) => row_a == row_b,
            _ => true,
        }
    })
}

fn differences(
    mirrors: &MirrorArray,
    i: usize,
    getter: fn(&MirrorArray, usize) -> Option<Vec<&Tile>>,
) -> Vec<(usize, usize)> {
    (0..=i)
        .flat_map(|j| {
            let a = getter(mirrors, i - j);
            let b = getter(mirrors, i + j + 1);

            match (a, b) {
                (Some(row_a), Some(row_b)) => Some(
                    row_a
                        .iter()
                        .zip_eq(row_b.iter())
                        .enumerate()
                        .filter(|(_, (tile_a, tile_b))| tile_a != tile_b)
                        .map(|(x, _)| (j, x))
                        .collect_vec(),
                ),
                _ => None,
            }
        })
        .flatten()
        .collect()
}

fn mutate_one(mirrors: &MirrorArray) -> Symmetry {
    let width = mirrors.width();
    let height = mirrors.height();

    for symmetry_col in 0..width - 1 {
        let diffs = differences(mirrors, symmetry_col, MirrorArray::get_col);
        if diffs.len() == 1 {
            return Symmetry::Column(symmetry_col);
        }
    }

    for symmetry_row in 0..height - 1 {
        let diffs = differences(mirrors, symmetry_row, MirrorArray::get_row);
        if diffs.len() == 1 {
            return Symmetry::Row(symmetry_row);
        }
    }

    Symmetry::None
}

pub fn part_one(input: &str) -> Option<usize> {
    let mirrors = MirrorArray::parse(input);

    Some(
        mirrors
            .iter()
            .map(MirrorArray::find_symmetry)
            .map(|symmetry| match symmetry {
                Symmetry::None => panic!("Not symmetric?"),
                Symmetry::Column(r) => r + 1,
                Symmetry::Row(r) => 100 * (r + 1),
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mirrors = MirrorArray::parse(input);

    Some(
        mirrors
            .par_iter()
            .map(mutate_one)
            .map(|symmetry| match symmetry {
                Symmetry::None => panic!("Not symmetric?"),
                Symmetry::Column(r) => r + 1,
                Symmetry::Row(r) => 100 * (r + 1),
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
