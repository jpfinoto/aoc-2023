use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::{Match, Regex};

use advent_of_code::utils::grid::{Cellular, find_intersections, GridCell, Growable, has_intersections};

advent_of_code::solution!(3);

#[derive(Debug)]
struct NumberCell {
    location: GridCell,
    value: u32,
}

#[derive(Debug)]
struct SymbolCell {
    location: GridCell,
    symbol: char,
}

enum Cell {
    Number(NumberCell),
    Symbol(SymbolCell),
}

impl Cellular for SymbolCell {
    fn cell(&self) -> &GridCell {
        &self.location
    }
}

impl Cellular for NumberCell {
    fn cell(&self) -> &GridCell {
        &self.location
    }
}


fn match_to_cell(m: &Match, row_number: i32) -> Option<Cell> {
    if m.as_str().trim().len() == 0 {
        return None;
    }

    let location = GridCell {
        top: row_number,
        bottom: row_number,
        left: m.start() as i32,
        right: m.end() as i32 - 1,
    };

    if let Ok(value) = u32::from_str_radix(m.as_str(), 10) {
        Some(Cell::Number(NumberCell { location, value }))
    } else if let Some(symbol) = m.as_str().chars().next() {
        Some(Cell::Symbol(SymbolCell { location, symbol }))
    } else {
        None
    }
}

lazy_static! {
    static ref SPAN_RE: Regex = Regex::new(r"(\d+|[^.])").unwrap();
}

fn parse_row(line: &str, row_number: i32) -> Vec<Cell> {
    SPAN_RE
        .captures_iter(line)
        .flat_map(
            |cap| cap.get(1).and_then(|m| match_to_cell(&m, row_number))
        )
        .collect()
}

fn parse(input: &str) -> Vec<Cell> {
    input.split("\n")
        .enumerate()
        .flat_map(|(i, line)| parse_row(line, i as i32))
        .collect()
}

fn get_symbols_and_numbers(spans: &Vec<Cell>) -> (Vec<&SymbolCell>, Vec<&NumberCell>) {
    let symbols: Vec<_> = spans.iter().filter_map(|cell| match cell {
        Cell::Number(_) => None,
        Cell::Symbol(symbol) => Some(symbol),
    }).collect();

    let numbers: Vec<_> = spans.iter().filter_map(|cell| match cell {
        Cell::Number(number) => Some(number),
        Cell::Symbol(_) => None,
    }).collect();

    (symbols, numbers)
}

pub fn part_one(input: &str) -> Option<u32> {
    let spans = parse(input);

    let (symbols, numbers) = get_symbols_and_numbers(&spans);

    let valid_numbers: Vec<_> = numbers.par_iter().filter_map(
        |&s| if has_intersections(&s.cell().grow((2, 2)), &symbols) {
            Some(s.value)
        } else {
            None
        }
    ).collect();

    Some(valid_numbers.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let spans = parse(input);

    let (symbols, numbers) = get_symbols_and_numbers(&spans);

    let gear_ratios = symbols
        .par_iter()
        .filter(|&s| s.symbol == '*')
        .map(|&s| find_intersections(&s.cell().grow((2, 2)), &numbers))
        .filter(|neighbours| neighbours.len() == 2)
        .map(|neighbours| neighbours.first().unwrap().value * neighbours.last().unwrap().value);

    return Some(gear_ratios.sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }

    #[test]
    fn test_grow() {
        let cell = GridCell {
            left: 0,
            right: 1,
            top: 0,
            bottom: 1,
        };

        assert_eq!(cell.grow((2, 2)), GridCell {
            left: -1,
            right: 2,
            top: -1,
            bottom: 2,
        });
    }

    #[test]
    fn test_intersect() {
        assert_eq!(false, has_intersections(
            &GridCell { left: 4, right: 8, top: -1, bottom: 1 },
            &vec![&SymbolCell { location: GridCell { left: 3, right: 3, top: 1, bottom: 1 }, symbol: '*' }],
        ));
    }
}
