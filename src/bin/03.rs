use regex::{Match, Regex};

advent_of_code::solution!(3);

struct GridRow {
    row: i32,
    left: i32,
    right: i32,
}

struct NumberCell {
    location: GridRow,
    value: u32,
}

struct SymbolCell {
    location: GridRow,
    symbol: char,
}

enum Cell {
    Number(NumberCell),
    Symbol(SymbolCell),
}

trait Gridded {
    fn intersects(&self, point: &(i32, i32)) -> bool;
    fn neighbours(&self) -> Vec<(i32, i32)>;
}

impl Gridded for GridRow {
    fn intersects(&self, point: &(i32, i32)) -> bool {
        let &(row, col) = point;
        self.row == row && col >= self.left && col < self.right
    }

    fn neighbours(&self) -> Vec<(i32, i32)> {
        let top_edge = (self.left - 1..self.right + 1).map(|i| (self.row - 1, i));
        let bottom_edge = (self.left - 1..self.right + 1).map(|i| (self.row + 1, i));
        let sides = [(self.row, self.left - 1), (self.row, self.right)];

        Vec::from_iter(top_edge.chain(bottom_edge).chain(sides))
    }
}

impl Gridded for SymbolCell {
    fn intersects(&self, point: &(i32, i32)) -> bool {
        self.location.intersects(point)
    }

    fn neighbours(&self) -> Vec<(i32, i32)> {
        self.location.neighbours()
    }
}

impl Gridded for NumberCell {
    fn intersects(&self, point: &(i32, i32)) -> bool {
        self.location.intersects(point)
    }

    fn neighbours(&self) -> Vec<(i32, i32)> {
        self.location.neighbours()
    }
}

fn match_to_cell(m: &Match, row_number: i32) -> Option<Cell> {
    if m.as_str().trim().len() == 0 {
        return None;
    }

    let location = GridRow {
        row: row_number,
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

fn find_neighbours<'a, I, G>(item: &I, grid: &'a Vec<&G>) -> Vec<&'a G> where I: Gridded, G: Gridded {
    let neighbour_points = item.neighbours();

    grid
        .into_iter()
        .filter(
            |&&s| neighbour_points.iter().any(|p| s.intersects(p)))
        .cloned()
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
