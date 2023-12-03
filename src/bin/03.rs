use regex::{Match, Regex};

advent_of_code::solution!(3);

#[derive(Debug)]
enum SpanValue {
    Number(u32),
    Symbol(char),
}

#[derive(Debug)]
struct Span {
    row: i32,
    left: i32,
    right: i32,
    value: SpanValue,
}

trait Gridded {
    fn intersects(&self, point: &(i32, i32)) -> bool;
    fn neighbours(&self) -> Vec<(i32, i32)>;
}

impl Gridded for Span {
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

fn get_span_from_match(m: &Match, row_number: i32) -> Option<Span> {
    if m.as_str().trim().len() == 0 {
        return None;
    }

    let value = match u32::from_str_radix(m.as_str(), 10) {
        Ok(value) => SpanValue::Number(value),
        Err(_) => SpanValue::Symbol(m.as_str().chars().next().unwrap())
    };

    Some(Span {
        row: row_number,
        left: m.start() as i32,
        right: m.end() as i32,
        value,
    })
}

fn parse_row(line: &str, row_number: i32) -> Vec<Span> {
    let span_re = Regex::new(r"(\d+|[^.])").unwrap();

    span_re
        .captures_iter(line)
        .map(
            |cap| cap.get(1).and_then(|m| get_span_from_match(&m, row_number))
        )
        .flatten()
        .collect()
}

fn parse(input: &str) -> Vec<Span> {
    input.split("\n")
        .enumerate()
        .map(|(i, line)| parse_row(line, i as i32))
        .flatten()
        .collect()
}

fn find_neighbours<'a>(span: &Span, all_spans: &'a Vec<&Span>) -> Vec<&'a Span> {
    let neighbour_points = span.neighbours();

    all_spans
        .into_iter()
        .filter(
            |&&s| neighbour_points.iter().any(|p| s.intersects(p)))
        .cloned()
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let spans = parse(input);

    let symbols: Vec<_> = spans.iter().filter(|&s| match s.value {
        SpanValue::Number(_) => false,
        SpanValue::Symbol(_) => true,
    }).collect();

    let numbers: Vec<_> = spans.iter().filter(|&s| match s.value {
        SpanValue::Number(_) => true,
        SpanValue::Symbol(_) => false,
    }).collect();

    let valid_numbers = numbers.into_iter().filter_map(
        |s| match find_neighbours(s, &symbols).len() > 0 {
            true => match s.value {
                SpanValue::Number(val) => Some(val),
                SpanValue::Symbol(_) => panic!(),
            },
            false => None
        }
    );

    Some(valid_numbers.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let spans = parse(input);

    let numbers: Vec<_> = spans.iter().filter(|&s| match s.value {
        SpanValue::Number(_) => true,
        SpanValue::Symbol(_) => false,
    }).collect();

    let gear_ratios = spans
        .iter()
        .filter_map(|s| {
            if let SpanValue::Symbol(symbol) = s.value {
                if symbol == '*' {
                    let neighbours = find_neighbours(&s, &numbers);
                    if neighbours.len() == 2 {
                        let SpanValue::Number(a) = neighbours.first().unwrap().value else { panic!() };
                        let SpanValue::Number(b) = neighbours.last().unwrap().value else { panic!() };

                        Some(a * b)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        });

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
