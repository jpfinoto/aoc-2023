use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(1);

fn make_number(first: Option<u32>, last: Option<u32>) -> Option<u32> {
    first.or(last).and_then(|_| Some(first.or(last).unwrap() * 10 + last.or(first).unwrap()))
}

fn get_numbers_simple(line: &str) -> Option<u32> {
    let first = line.chars().filter_map(|x| x.to_digit(10)).next();
    let last = line.chars().rev().filter_map(|x| x.to_digit(10)).next();

    make_number(first, last)
}

pub fn part_one(input: &str) -> Option<u32> {
    return Some(
        input
            .split("\n")
            .flat_map(get_numbers_simple)
            .sum()
    );
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

    make_number(first, last)
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
