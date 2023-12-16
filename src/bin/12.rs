use std::str::FromStr;

use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(12);

#[derive(PartialEq, Debug, Copy, Clone)]
enum RepairStatus {
    Unknown,
    Operational,
    Damaged,
}

struct SpringsMap {
    springs: Vec<RepairStatus>,
    groups: Vec<u32>,
}

fn parse_line(line: &str) -> Option<SpringsMap> {
    let (springs_str, nums_str) = line.split(" ").next_tuple()?;
    let groups = nums_str.split(",").flat_map(u32::from_str).collect_vec();
    let springs = springs_str
        .chars()
        .flat_map(|c| match c {
            '#' => Some(RepairStatus::Damaged),
            '.' => Some(RepairStatus::Operational),
            '?' => Some(RepairStatus::Unknown),
            _ => None,
        })
        .collect_vec();

    Some(SpringsMap { groups, springs })
}

fn parse(input: &str) -> impl Iterator<Item = SpringsMap> + '_ {
    input.split("\n").flat_map(parse_line)
}

fn find_groups(springs: &Vec<RepairStatus>) -> Vec<u32> {
    springs
        .iter()
        .group_by(|e| *e)
        .into_iter()
        .flat_map(|(status, group)| match status {
            RepairStatus::Unknown => None,
            RepairStatus::Operational => None,
            RepairStatus::Damaged => Some(group.collect_vec().len() as u32),
        })
        .collect()
}

fn is_valid(springs: &Vec<RepairStatus>, expected_groups: &Vec<u32>) -> bool {
    *expected_groups == find_groups(springs)
}

fn apply_unknowns(
    base: &Vec<RepairStatus>,
    replacements: &Vec<&RepairStatus>,
) -> Vec<RepairStatus> {
    let mut result: Vec<RepairStatus> = vec![];
    let mut replacements_iter = replacements.iter();

    for base_status in base {
        result.push(match base_status {
            RepairStatus::Unknown => **replacements_iter.next().unwrap(),
            RepairStatus::Operational => *base_status,
            RepairStatus::Damaged => *base_status,
        });
    }

    result
}

fn bruteforce_count_options(sm: &SpringsMap) -> u32 {
    let total_unknowns = sm
        .springs
        .iter()
        .filter(|s| match s {
            RepairStatus::Unknown => true,
            _ => false,
        })
        .count();
    let mut valid_count = 0u32;

    for replacements in (0..total_unknowns)
        .map(|_| [RepairStatus::Operational, RepairStatus::Damaged].iter())
        .multi_cartesian_product()
    {
        let r = apply_unknowns(&sm.springs, &replacements);
        if is_valid(&r, &sm.groups) {
            valid_count += 1;
        }
    }

    valid_count
}

#[allow(dead_code)]
fn unfold(sm: &SpringsMap) -> SpringsMap {
    let groups = sm.groups.repeat(5);
    let springs = vec![
        sm.springs.clone(),
        vec![RepairStatus::Unknown],
        sm.springs.clone(),
        vec![RepairStatus::Unknown],
        sm.springs.clone(),
        vec![RepairStatus::Unknown],
        sm.springs.clone(),
        vec![RepairStatus::Unknown],
        sm.springs.clone(),
    ]
    .concat();

    return SpringsMap { groups, springs };
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .par_bridge()
            .map(|s| bruteforce_count_options(&s))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // Some(
    //     parse(input)
    //         .map(|s| bruteforce_count_options(&unfold(&s)))
    //         .sum(),
    // )

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
