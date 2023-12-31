use std::collections::HashMap;

use advent_of_code::utils::dense_grid::{DenseGrid, DOWN, LEFT, RIGHT, UP};
use advent_of_code::utils::geometry;
use advent_of_code::utils::geometry::{Direction, XY};

advent_of_code::solution!(10);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Pipe {
    TwoWay(XY, XY),
    Empty,
    Start,
}

fn parse_pipe(c: char) -> Pipe {
    match c {
        '|' => Pipe::TwoWay(UP, DOWN),
        '-' => Pipe::TwoWay(LEFT, RIGHT),
        'L' => Pipe::TwoWay(UP, RIGHT),
        'J' => Pipe::TwoWay(UP, LEFT),
        '7' => Pipe::TwoWay(DOWN, LEFT),
        'F' => Pipe::TwoWay(DOWN, RIGHT),
        '.' => Pipe::Empty,
        'S' => Pipe::Start,
        _ => panic!("Invalid char {c}"),
    }
}

fn find_cycle(start_xy: XY, grid: &DenseGrid<Pipe>) -> (u32, HashMap<XY, Direction>) {
    let mut boundary_pipes = HashMap::new();

    for start_dir in [UP, DOWN, LEFT, RIGHT] {
        boundary_pipes.clear();

        let mut steps = 1u32;
        let mut xy = start_xy + start_dir;
        let mut prev_xy = start_xy;

        while let Some(pipe) = grid.get(xy) {
            match pipe {
                Pipe::TwoWay(a, b) => {
                    let dest_a = xy + *a;
                    let dest_b = xy + *b;

                    let next_step = if dest_a == prev_xy {
                        dest_b
                    } else if dest_b == prev_xy {
                        dest_a
                    } else {
                        // entered from the wrong direction
                        break;
                    };

                    let pipe_direction = match (a, b) {
                        (&UP, &DOWN) => Direction::UpDown,
                        (&LEFT, &RIGHT) => Direction::LeftRight,
                        (&UP, &RIGHT) => Direction::Corner(-1),
                        (&UP, &LEFT) => Direction::Corner(1),
                        (&DOWN, &LEFT) => Direction::Corner(-1),
                        (&DOWN, &RIGHT) => Direction::Corner(1),
                        _ => panic!("Invalid pipe"),
                    };

                    boundary_pipes.insert(xy, pipe_direction);

                    prev_xy = xy;
                    xy = next_step;
                }
                Pipe::Empty => break,
                Pipe::Start => return (steps, boundary_pipes),
            };

            steps += 1;
        }
    }

    panic!("Couldn't find route");
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = DenseGrid::parse(input, parse_pipe, Some(Pipe::Empty));
    let Some((_, start_xy)) = grid.find(|pipe| *pipe == Pipe::Start) else {
        panic!()
    };

    let (cycle_length, _) = find_cycle(start_xy, &grid);

    Some(cycle_length / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = DenseGrid::parse(input, parse_pipe, Some(Pipe::Empty));
    let Some((_, start_xy)) = grid.find(|pipe| *pipe == Pipe::Start) else {
        panic!()
    };

    let (_, boundary) = find_cycle(start_xy, &grid);
    let inside = geometry::get_odd(&boundary, 0..(grid.width as i64), 0..(grid.height() as i64));

    let total = inside.len();

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use sdl2::pixels::Color;

    use advent_of_code::utils::visuals::grid::GridRenderer;

    use super::*;

    struct PipeRenderer {}

    impl GridRenderer<Pipe> for PipeRenderer {
        fn render(&self, tile: &Pipe) -> Color {
            match tile {
                Pipe::TwoWay(_, _) => Color::WHITE,
                Pipe::Empty => Color::BLACK,
                Pipe::Start => Color::RED,
            }
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn plot() {
        let input = advent_of_code::template::read_file("inputs", DAY);
        let grid = DenseGrid::parse(&input, parse_pipe, Some(Pipe::Empty));
        let Some((_, start_xy)) = grid.find(|pipe| *pipe == Pipe::Start) else {
            panic!()
        };
        let (_, boundary) = find_cycle(start_xy, &grid);

        // plot_grid(&GridOptions {
        //     window: WindowOptions {
        //         width: 800,
        //         height: 800,
        //         title: "Pipe Dream",
        //         background_color: Color::RGB(0, 0, 0),
        //     },
        //     grid_scale: 0.0,
        // }, &PipeRenderer {}, vec![].as_slice());
    }
}
