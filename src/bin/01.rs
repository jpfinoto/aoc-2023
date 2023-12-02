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

static REPLACEMENTS: &[(&str, u32)] = &[
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
];

fn get_numbers(line: String) -> Vec<u32> {
    let mut entries: Vec<(usize, u32)> = Vec::new();

    for &(from, to) in REPLACEMENTS {
        let indices: Vec<_> = line.match_indices(from).map(|(i, _)| (i, to)).collect();
        entries.extend_from_slice(indices.as_slice());
    }

    entries.sort_by(|(a_idx, _), (b_idx, _)| a_idx.partial_cmp(b_idx).unwrap());

    return entries.into_iter().map(|(_, x)| x).collect();
}

pub fn part_two(input: &String) -> Option<u32> {
    let lines: Vec<_> = input.split("\n").map(String::from).collect();
    let number_lines: Vec<Vec<u32>> = lines
        .into_iter()
        .map(get_numbers)
        .filter(|v: &Vec<u32>| v.len() >= 1)
        .collect();

    let calibration_factors: Vec<u32> = number_lines.iter().map(|nums| nums.first().unwrap() * 10 + nums.last().unwrap()).collect();

    return Some(calibration_factors.iter().sum());
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
