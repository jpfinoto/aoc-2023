use itertools::Itertools;

use advent_of_code::utils::parsing::get_big_numbers;

advent_of_code::solution!(6);

fn parse(input: &str) -> Vec<(u64, u64)> {
    let parts = input.split("\n").collect_vec();
    get_big_numbers(parts[0].split(":").last().unwrap())
        .into_iter()
        .zip_eq(get_big_numbers(parts[1].split(":").last().unwrap()).into_iter())
        .collect()
}

fn parse2(input: &str) -> (u64, u64) {
    let parts = input.split("\n").collect_vec();
    let line1 = parts[0].split(":").last().unwrap().replace(" ", "");
    let line2 = parts[1].split(":").last().unwrap().replace(" ", "");
    (
        u64::from_str_radix(&line1, 10).unwrap(),
        u64::from_str_radix(&line2, 10).unwrap(),
    )
}

fn hold_time_range(total_time: u64, total_distance: u64) -> (u64, u64) {
    // d = hold_time * a * (total_time - hold_time) > total_distance  (a = 1)
    // t * h - h^2 > d/a
    // h^2 - t*h + d/a > 0
    // solutions: [t +- sqrt(t^2 - 4 * d/a)]/2

    let discriminant = f64::powi(total_time as f64, 2) - 4f64 * (total_distance as f64);
    let root = discriminant.sqrt();
    let mut sol_1 = ((total_time as f64) - root) / 2f64;
    let mut sol_2 = ((total_time as f64) + root) / 2f64;

    if sol_1 == sol_1.ceil() {
        sol_1 += 1f64;
    }

    if sol_2 == sol_2.floor() {
        sol_2 -= 1f64;
    }

    (sol_1.ceil() as u64, sol_2.floor() as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .iter()
            .map(|&(total_time, total_dist)| hold_time_range(total_time, total_dist))
            .map(|(min_time, max_time)| max_time - min_time + 1)
            .product()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (time, distance) = parse2(input);
    let (min, max) = hold_time_range(time, distance);

    Some(max - min + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(71503));
    }
}
