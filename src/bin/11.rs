use itertools::Itertools;

use advent_of_code::utils::geometry::XY;

advent_of_code::solution!(11);

struct Galaxy {
    pos: XY,
}

fn parse_galaxies(block: &str) -> Vec<Galaxy> {
    block
        .split("\n")
        .map(str::trim)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| match c {
                '#' => Some(Galaxy {
                    pos: XY(x as i64, y as i64),
                }),
                _ => None,
            })
        })
        .collect_vec()
}

fn expand_vector(vec: &Vec<i64>, multiplier: i64) -> Vec<i64> {
    let mut result = vec![];
    let mut expansion = 0;

    for (i, curr) in vec.iter().enumerate() {
        if i > 0 {
            expansion += curr - vec[i - 1] - 1;
        }
        result.push(curr + expansion * multiplier);
    }

    result
}

fn interp_1d(x: &Vec<i64>, y: &Vec<i64>, value: &i64) -> i64 {
    if let Ok(i) = x.binary_search(value) {
        y[i]
    } else {
        panic!()
    }
}

fn expand(galaxies: &Vec<Galaxy>, multiplier: i64) -> Vec<Galaxy> {
    let all_x = galaxies
        .iter()
        .map(|g| g.pos.0)
        .sorted()
        .dedup()
        .collect_vec();
    let expanded_x = expand_vector(&all_x, multiplier);

    let all_y = galaxies
        .iter()
        .map(|g| g.pos.1)
        .sorted()
        .dedup()
        .collect_vec();
    let expanded_y = expand_vector(&all_y, multiplier);

    galaxies
        .iter()
        .map(|g| Galaxy {
            pos: XY(
                interp_1d(&all_x, &expanded_x, &g.pos.0),
                interp_1d(&all_y, &expanded_y, &g.pos.1),
            ),
        })
        .collect()
}

fn distance(a: &Galaxy, b: &Galaxy) -> i64 {
    (a.pos.0 - b.pos.0).abs() + (a.pos.1 - b.pos.1).abs()
}

pub fn part_one(input: &str) -> Option<i64> {
    let galaxies = parse_galaxies(input);
    let expanded_galaxies = expand(&galaxies, 1);

    let sum = expanded_galaxies
        .iter()
        .combinations(2)
        .map(|g| distance(g[0], g[1]))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let galaxies = parse_galaxies(input);
    let expanded_galaxies = expand(&galaxies, 1000000 - 1);

    let sum = expanded_galaxies
        .iter()
        .combinations(2)
        .map(|g| distance(g[0], g[1]))
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
