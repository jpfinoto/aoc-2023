use itertools::Itertools;
use num::integer::binomial;
use advent_of_code::utils::parsing::{get_big_signed_numbers};
advent_of_code::solution!(9);

fn get_differences(numbers: &Vec<i64>) -> Vec<i64> {
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect()
}

fn get_coefficients(numbers: &Vec<i64>) -> Vec<i64> {
    let mut line = numbers.clone();
    let mut coeff: Vec<i64> = vec![];

    while line.iter().any(|k| *k != 0) {
        coeff.push(line[0]);
        line = get_differences(&line);
    }

    coeff
}

fn extrapolate(coeff: &Vec<i64>, n: i64) -> i64 {
    // I thought that you'd need to extrapolate super far in the future for part 2
    // so I ended up deriving the closed formula for the nth element
    // turns out that wasn't needed lol

    coeff
        .iter()
        .enumerate()
        .map(|(i, c)| n.signum() * binomial(n.abs(), i as i64) * c)
        .sum()
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .split("\n")
        .map(str::trim)
        .map(get_big_signed_numbers)
        .filter(|n| n.len() > 0)
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<i64> {
    let readings = parse(input);

    Some(
        readings
            .iter()
            .map(|r| extrapolate(&get_coefficients(r), r.len() as i64))
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let readings = parse(input);

    Some(
        readings
            .iter()
            .map(get_coefficients)
            .map(|coeff| coeff.into_iter().rev().reduce(|acc, c| c - acc))
            .flatten()
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
