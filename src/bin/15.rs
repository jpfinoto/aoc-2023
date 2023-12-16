use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(15);

lazy_static! {
    static ref PARSE_OPERATION: Regex =
        Regex::new(r"^(?P<label>[a-z]+)(?P<op>[=\-])(?P<fl>\d)?$").unwrap();
}

#[derive(Debug)]
enum Operation {
    Insert(u32),
    Remove,
}

#[derive(Debug)]
struct InitStep {
    label: String,
    operation: Operation,
}

trait AsciiHash {
    fn ascii_hash(&self) -> u32;
}

impl InitStep {
    fn as_string(&self) -> String {
        match self.operation {
            Operation::Insert(focal_length) => format!("{}={focal_length}", self.label),
            Operation::Remove => format!("{}-", self.label),
        }
    }

    fn parse(input: &str) -> Option<InitStep> {
        let cap = PARSE_OPERATION.captures(input)?;
        let op = cap.name("op")?.as_str();
        let label = cap.name("label")?.as_str().to_string();
        let operation = match op {
            "=" => Some(Operation::Insert(
                u32::from_str(cap.name("fl")?.as_str()).ok()?,
            )),
            "-" => Some(Operation::Remove),
            _ => None,
        }?;

        Some(InitStep { label, operation })
    }

    fn parse_steps(input: &str) -> Vec<InitStep> {
        input
            .split(",")
            .map(str::trim)
            .flat_map(InitStep::parse)
            .collect_vec()
    }

    fn label_hash(&self) -> u32 {
        self.label.ascii_hash()
    }
}

impl AsciiHash for InitStep {
    fn ascii_hash(&self) -> u32 {
        self.as_string().ascii_hash()
    }
}

impl AsciiHash for String {
    fn ascii_hash(&self) -> u32 {
        self.chars()
            .map(|c| c as u32)
            .fold(0, |acc, c| (acc + c) * 17 % 256)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = InitStep::parse_steps(input)
        .iter()
        .map(InitStep::ascii_hash)
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let steps = InitStep::parse_steps(input);
    let mut boxes: Vec<Vec<(String, u32)>> = vec![];
    for _ in 0..256usize {
        boxes.push(vec![]);
    }

    for step in steps {
        let box_index = step.label_hash() as usize;
        let current_box = &mut boxes[box_index];
        match step.operation {
            Operation::Insert(focal_length) => {
                if let Some((previous_lens_index, _)) = current_box
                    .iter()
                    .find_position(|(lens_label, _)| *lens_label == step.label)
                {
                    current_box[previous_lens_index] = (step.label.clone(), focal_length)
                } else {
                    current_box.push((step.label.clone(), focal_length))
                }
            }
            Operation::Remove => {
                boxes[box_index] = current_box
                    .iter()
                    .filter(|(lens_label, _)| *lens_label != step.label)
                    .cloned()
                    .collect()
            }
        }
    }

    let sum = boxes
        .iter()
        .enumerate()
        .map(|(box_number, b)| {
            (box_number + 1)
                * b.iter()
                    .enumerate()
                    .map(|(slot_number, (_, focal_length))| {
                        (slot_number + 1) * (*focal_length as usize)
                    })
                    .sum::<usize>()
        })
        .sum::<usize>();

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
