use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split("\n").collect();
    let number_lines: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().filter_map(|x| x.to_digit(10)).collect())
        .filter(|v: &Vec<u32>| v.len() >= 1)
        .collect();
    let calibration_factors: Vec<u32> = number_lines.iter().map(|nums| nums.first().unwrap() * 10 + nums.last().unwrap()).collect();

    return Some(calibration_factors.iter().sum());
}


lazy_static! {
    static ref VALUE_MAP: HashMap<&'static str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    static ref FIND_FIRST_RE: Regex = Regex::new(
        r"^.*?(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)"
    ).unwrap();

    static ref FIND_LAST_RE: Regex = Regex::new(
        r".*(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9).*?$"
    ).unwrap();
}

fn capture_int(line: &str, re: &Regex) -> Option<u32> {
    re
        .captures(line)
        .and_then(|m|
            m.get(1)
                .and_then(|s| VALUE_MAP.get(s.as_str()))
        ).cloned()
}

fn get_numbers(line: &str) -> Option<u32> {
    let first = capture_int(line, &FIND_FIRST_RE);
    let last = capture_int(line, &FIND_LAST_RE);

    if first.is_none() && last.is_none() {
        None
    } else {
        Some(first.or(last).unwrap() * 10 + last.or(first).unwrap())
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.split("\n").collect();

    return Some(
        lines
            .into_iter()
            .flat_map(get_numbers)
            .sum()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
