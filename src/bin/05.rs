use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter;
use std::iter::{Chain, Once};
use std::ops::{Range, RangeInclusive};

use itertools::Itertools;

use advent_of_code::utils::parsing::get_big_numbers;

advent_of_code::solution!(5);

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct RangeMap {
    source: u64,
    dest: u64,
    length: u64,
}

#[derive(Debug)]
struct Mapper {
    ranges: Vec<RangeMap>,
}

struct Inputs {
    seeds: Vec<u64>,
    seed_to_soil: Mapper,
    soil_to_fertilizer: Mapper,
    fertilizer_to_water: Mapper,
    water_to_light: Mapper,
    light_to_temp: Mapper,
    temp_to_humidity: Mapper,
    humidity_to_location: Mapper,
}

impl RangeMap {
    fn map_value(&self, source_id: u64) -> u64 {
        if source_id < self.source || source_id >= self.source + self.length {
            panic!("Invalid mapping of {source_id}")
        }

        self.dest + source_id - self.source
    }

    fn input_range(&self) -> Range<u64> {
        self.source..self.source + self.length
    }

    fn output_range(&self) -> Range<u64> {
        self.dest..self.dest + self.length
    }

    fn bounds(&self) -> Chain<Once<u64>, Once<u64>> {
        iter::once(self.input_range().start).chain(iter::once(self.input_range().end))
    }
}

impl Display for RangeMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{} -> {}..{}", self.input_range().start, self.input_range().end, self.output_range().start, self.output_range().end)
    }
}

fn range_finder(source_id: u64) -> Box<dyn Fn(&RangeMap) -> Ordering> {
    Box::new(move |range: &RangeMap| {
        if source_id >= range.source && source_id < range.source + range.length {
            Ordering::Equal
        } else if source_id < range.source {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    })
}

impl Mapper {
    fn map_value(&self, source_id: u64) -> u64 {
        self.get_range_map(source_id).map_or(source_id, |m| m.map_value(source_id))
    }

    fn get_range_map(&self, source_id: u64) -> Option<&RangeMap> {
        match self.ranges.binary_search_by(range_finder(source_id)) {
            Ok(idx) => Some(&self.ranges[idx]),
            Err(_) => None,
        }
    }

    fn map_range(&self, range: &RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
        // the idea is to map a range of input values into several output ranges
        // then we'll recursively map these ranges

        let breakpoints = iter::once(*range.start())
            .chain(self.ranges.iter().flat_map(|m| m.bounds()))
            .chain(iter::once(range.end() + 1))
            .filter(|x| x >= range.start() && *x <= *range.end() + 1)
            .dedup()
            .collect_vec();

        // at this point, ranges in the breakpoints are guaranteed to have at most one mapper
        // the pairing of two consecutive breakpoints forms an open range

        breakpoints
            .iter()
            .zip(breakpoints.iter().skip(1))
            .map(|(&start, &end)| {
                let mapper = self.get_range_map(start);
                let mapped_start = mapper.map_or(start, |m| m.map_value(start));
                let mapped_end = mapper.map_or(end - 1, |m| m.map_value(end - 1));

                mapped_start..=mapped_end
            }).collect_vec()
    }
}

fn parse_range(line: &str) -> Option<RangeMap> {
    let parts = line
        .split(" ")
        .flat_map(|s| u64::from_str_radix(s, 10))
        .collect_vec();

    if parts.len() == 3 {
        Some(RangeMap {
            dest: parts[0],
            source: parts[1],
            length: parts[2],
        })
    } else {
        None
    }
}

fn parse_block(block: &str) -> Mapper {
    let mut ranges = block.split("\n").flat_map(parse_range).collect_vec();
    ranges.sort();

    Mapper {
        ranges
    }
}

fn parse(input: &str) -> Inputs {
    let parts = input.split(":").collect_vec();

    if parts.len() != 9 {
        panic!("Invalid input");
    }

    Inputs {
        seeds: get_big_numbers(parts[1].split("\n").next().unwrap()),
        seed_to_soil: parse_block(parts[2]),
        soil_to_fertilizer: parse_block(parts[3]),
        fertilizer_to_water: parse_block(parts[4]),
        water_to_light: parse_block(parts[5]),
        light_to_temp: parse_block(parts[6]),
        temp_to_humidity: parse_block(parts[7]),
        humidity_to_location: parse_block(parts[8]),
    }
}

fn map_forward(input: u64, mappers: &Vec<&Mapper>) -> u64 {
    mappers.iter().fold(input, |prev, mapper| {
        let res = mapper.map_value(prev);
        // println!("Map {prev} into {res} using {:?}", mapper);
        res
    })
}

fn map_range(input: RangeInclusive<u64>, mappers: &Vec<&Mapper>) -> Vec<RangeInclusive<u64>> {
    mappers.iter().fold(vec![input], |prev, mapper| {
        let res = prev.iter().flat_map(|r| mapper.map_range(r)).collect_vec();
        // println!("Map {:?} into {:?} using {:?}", prev, res, mapper);
        res
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let inputs = parse(input);
    let mappers = vec![
        &inputs.seed_to_soil,
        &inputs.soil_to_fertilizer,
        &inputs.fertilizer_to_water,
        &inputs.water_to_light,
        &inputs.light_to_temp,
        &inputs.temp_to_humidity,
        &inputs.humidity_to_location,
    ];

    Some(inputs.seeds.iter().map(|&seed| {
        // println!("\n\nStart converting {seed}");
        map_forward(seed, &mappers)
    }).min().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let inputs = parse(input);
    let mappers = vec![
        &inputs.seed_to_soil,
        &inputs.soil_to_fertilizer,
        &inputs.fertilizer_to_water,
        &inputs.water_to_light,
        &inputs.light_to_temp,
        &inputs.temp_to_humidity,
        &inputs.humidity_to_location,
    ];


    Some(
        inputs
            .seeds
            .chunks_exact(2)
            .map(|range| range[0]..=range[0] + range[1] - 1)
            .map(|range| map_range(range, &mappers))
            .flatten()
            .map(|range| *range.start())
            .min()
            .unwrap()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
