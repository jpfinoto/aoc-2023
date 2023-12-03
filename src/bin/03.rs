use regex::{Match, Regex};

use advent_of_code::utils::grid::{BoundedArea, find_neighbours, GridCell, Gridded};

advent_of_code::solution!(3);

struct NumberCell {
    location: GridCell,
    value: u32,
}

struct SymbolCell {
    location: GridCell,
    symbol: char,
}

enum Cell {
    Number(NumberCell),
    Symbol(SymbolCell),
}

impl Gridded<(i32, i32)> for SymbolCell {
    fn neighbours(&self) -> Vec<(i32, i32)> {
        self.location.neighbours()
    }
}

impl BoundedArea<(i32, i32)> for SymbolCell {
    fn contains(&self, point: &(i32, i32)) -> bool {
        self.location.contains(point)
    }
}

impl Gridded<(i32, i32)> for NumberCell {
    fn neighbours(&self) -> Vec<(i32, i32)> {
        self.location.neighbours()
    }
}

impl BoundedArea<(i32, i32)> for NumberCell {
    fn contains(&self, point: &(i32, i32)) -> bool {
        self.location.contains(point)
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
        right: m.end() as i32,
    };

    if let Ok(value) = u32::from_str_radix(m.as_str(), 10) {
        Some(Cell::Number(NumberCell { location, value }))
    } else if let Some(symbol) = m.as_str().chars().next() {
        Some(Cell::Symbol(SymbolCell { location, symbol }))
    } else {
        None
    }
}

fn parse_row(line: &str, row_number: i32) -> Vec<Cell> {
    let span_re = Regex::new(r"(\d+|[^.])").unwrap();

    span_re
        .captures_iter(line)
        .map(
            |cap| cap.get(1).and_then(|m| match_to_cell(&m, row_number))
        )
        .flatten()
        .collect()
}

fn parse(input: &str) -> Vec<Cell> {
    input.split("\n")
        .enumerate()
        .map(|(i, line)| parse_row(line, i as i32))
        .flatten()
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

    let valid_numbers = numbers.into_iter().filter_map(
        |s| if find_neighbours(s, &symbols).len() > 0 { Some(s.value) } else { None }
    );

    Some(valid_numbers.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let spans = parse(input);

    let (symbols, numbers) = get_symbols_and_numbers(&spans);

    let gear_ratios = symbols.iter().filter_map(
        |&s| {
            if s.symbol == '*' {
                let neighbours = find_neighbours(s, &numbers);
                if neighbours.len() == 2 {
                    Some(neighbours.first().unwrap().value * neighbours.last().unwrap().value)
                } else {
                    None
                }
            } else {
                None
            }
        }
    );

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
}
