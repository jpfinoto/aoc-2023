use std::cmp::Ordering;
use std::iter;
use std::iter::{Chain, Once};
use std::ops::RangeInclusive;

use itertools::{Itertools, MinMaxResult};

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
    mappers: Vec<Mapper>,
}

impl RangeMap {
    fn map_value(&self, source_id: u64) -> u64 {
        if source_id < self.source || source_id >= self.source + self.length {
            panic!("Invalid mapping of {source_id}")
        }

        self.dest + source_id - self.source
    }

    fn bounds(&self) -> Chain<Once<u64>, Once<u64>> {
        iter::once(self.source).chain(iter::once(self.source + self.length))
    }

    fn inverse(&self) -> RangeMap {
        RangeMap {
            source: self.dest,
            dest: self.source,
            length: self.length,
        }
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
        self.get_range_map(source_id)
            .map_or(source_id, |m| m.map_value(source_id))
    }

    fn get_range_map(&self, source_id: u64) -> Option<&RangeMap> {
        match self.ranges.binary_search_by(range_finder(source_id)) {
            Ok(idx) => Some(&self.ranges[idx]),
            Err(_) => None,
        }
    }

    fn map_range(&self, range: &RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
        // the idea is to map a range of input values into several output ranges
        // then we'll recursively break down these ranges

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
            })
            .collect_vec()
    }

    #[allow(dead_code)]
    fn inverse(&self) -> Mapper {
        let mut ranges = self
            .ranges
            .iter()
            .map(RangeMap::inverse)
            .rev()
            .collect_vec();
        ranges.sort();

        Mapper { ranges }
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

    Mapper { ranges }
}

fn parse(input: &str) -> Inputs {
    let parts = input.split(":").collect_vec();

    if parts.len() != 9 {
        panic!("Invalid input");
    }

    Inputs {
        seeds: get_big_numbers(parts[1].split("\n").next().unwrap()),
        mappers: parts[2..=8].iter().cloned().map(parse_block).collect(),
    }
}

fn map_forward(input: u64, mappers: &Vec<Mapper>) -> u64 {
    mappers.iter().fold(input, |prev, mapper| {
        let res = mapper.map_value(prev);
        // println!("Map {prev} into {res} using {:?}", mapper);
        res
    })
}

fn map_range(input: RangeInclusive<u64>, mappers: &Vec<Mapper>) -> Vec<RangeInclusive<u64>> {
    mappers.iter().fold(vec![input], |prev, mapper| {
        let res = prev.iter().flat_map(|r| mapper.map_range(r)).collect_vec();
        // println!("Map {:?} into {:?} using {:?}", prev, res, mapper);
        res
    })
}

#[allow(dead_code)]
fn compile(mappers: &Vec<Mapper>) -> Mapper {
    // this is really bad and makes everything slower

    let inverse_mappers = mappers.iter().map(Mapper::inverse).rev().collect_vec();

    let (min, max) = match mappers
        .last()
        .unwrap()
        .ranges
        .iter()
        .flat_map(|m| m.bounds())
        .minmax()
    {
        MinMaxResult::NoElements => panic!(),
        MinMaxResult::OneElement(min) => (min, min),
        MinMaxResult::MinMax(min, max) => (min, max),
    };

    let final_ranges = map_range(min..=max, mappers);
    let initial_ranges = final_ranges
        .into_iter()
        .flat_map(|r| map_range(r, &inverse_mappers))
        .collect_vec();

    let ranges = initial_ranges
        .into_iter()
        .map(|r| {
            let final_ranges = map_range(r.clone(), mappers);
            assert_eq!(1, final_ranges.len());
            let final_range = final_ranges.first().unwrap();

            RangeMap {
                source: *r.start(),
                dest: *final_range.start(),
                length: *r.end() - *r.start(),
            }
        })
        .filter(|m| m.source != m.dest)
        .collect_vec();

    println!("compiled: {:?}", ranges);

    Mapper { ranges }
}

pub fn part_one(input: &str) -> Option<u64> {
    let inputs = parse(input);

    inputs
        .seeds
        .iter()
        .map(|&seed| map_forward(seed, &inputs.mappers))
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let inputs = parse(input);

    inputs
        .seeds
        .chunks_exact(2)
        .map(|range| range[0]..=range[0] + range[1] - 1)
        .map(|range| map_range(range, &inputs.mappers))
        .flatten()
        .map(|range| *range.start())
        .min()
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

    #[test]
    fn test_compile() {
        let inputs = parse(&advent_of_code::template::read_file("examples", DAY));
        let mappers = compile(&inputs.mappers);
        println!("{:?}", mappers)
    }
}
