use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

use rayon::prelude::*;

use advent_of_code::utils::dense_grid::{DenseGrid, DOWN, LEFT, RIGHT, UP};
use advent_of_code::utils::geometry::XY;

advent_of_code::solution!(16);

#[derive(Eq, PartialEq, Hash, Debug)]
struct Splitter {
    enter_directions: [XY; 2],
    split_directions: [XY; 2],
}

const UP_DOWN_SPLITTER: Splitter = Splitter {
    enter_directions: [LEFT, RIGHT],
    split_directions: [UP, DOWN],
};

const LEFT_RIGHT_SPLITTER: Splitter = Splitter {
    enter_directions: [UP, DOWN],
    split_directions: [LEFT, RIGHT],
};

#[derive(Debug)]
struct Beam {
    start: XY,
    direction: XY,
}

#[derive(Copy, Clone, Debug)]
struct TileEnergy {
    has_up: bool,
    has_down: bool,
    has_left: bool,
    has_right: bool,
}

impl Display for TileEnergy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_powered() {
            f.write_str("#")
        } else {
            f.write_str(".")
        }
    }
}

impl TileEnergy {
    fn empty() -> TileEnergy {
        TileEnergy {
            has_up: false,
            has_down: false,
            has_left: false,
            has_right: false,
        }
    }

    fn is_powered(&self) -> bool {
        self.has_left || self.has_right || self.has_up || self.has_down
    }
}

/// A left mirror is one that turns a RIGHT beam into an UP beam (+90°)
/// and a right mirror is one that turns a RIGHT beam into a DOWN beam (-90°)
#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
enum MirrorTile {
    Splitter(&'static Splitter),
    MirrorLeft,
    MirrorRight,
    Ground,
}

fn hit_mirror_left(beam: &Beam, p: XY) -> Beam {
    let new_dir = match beam.direction {
        UP => RIGHT,
        DOWN => LEFT,
        LEFT => DOWN,
        RIGHT => UP,
        _ => panic!("Invalid beam direction: {:?}", beam.direction),
    };

    Beam {
        start: p,
        direction: new_dir,
    }
}

fn hit_mirror_right(beam: &Beam, p: XY) -> Beam {
    let new_dir = match beam.direction {
        UP => LEFT,
        DOWN => RIGHT,
        LEFT => UP,
        RIGHT => DOWN,
        _ => panic!("Invalid beam direction: {:?}", beam.direction),
    };

    Beam {
        start: p,
        direction: new_dir,
    }
}

fn hit_splitter(beam: &Beam, splitter: &Splitter, p: XY) -> Option<Vec<Beam>> {
    if splitter.enter_directions.contains(&beam.direction) {
        Some(
            splitter
                .split_directions
                .iter()
                .map(|&direction| Beam {
                    start: p,
                    direction,
                })
                .collect(),
        )
    } else {
        None
    }
}

fn handle_beam_entering_tile(beam: &Beam, energy_tile: &mut TileEnergy) -> bool {
    match beam.direction {
        UP => match energy_tile.has_up {
            true => true,
            false => {
                energy_tile.has_up = true;
                false
            }
        },
        DOWN => match energy_tile.has_down {
            true => true,
            false => {
                energy_tile.has_down = true;
                false
            }
        },
        LEFT => match energy_tile.has_left {
            true => true,
            false => {
                energy_tile.has_left = true;
                false
            }
        },
        RIGHT => match energy_tile.has_right {
            true => true,
            false => {
                energy_tile.has_right = true;
                false
            }
        },
        _ => panic!("Invalid beam direction: {:?}", beam.direction),
    }
}

fn propagate_beam(
    beam: &Beam,
    board: &DenseGrid<MirrorTile>,
    energy: &mut DenseGrid<TileEnergy>,
) -> Vec<Beam> {
    for steps in 1i64.. {
        let p = beam.start + beam.direction * steps;
        // println!("propagate {beam:?} to {p:?}");
        if let Some(energy_tile) = energy.get_mut(p) {
            if handle_beam_entering_tile(beam, energy_tile) {
                // we have entered a tile where a beam already traveled in this direction, so we can stop
                // println!("* stop - already passed");
                break;
            }
        } else {
            // println!("* stop - out of the board");
            break;
        }

        if let Some(tile) = board.get(p) {
            // println!("> tile at {p:?} is {tile:?}");
            match tile {
                MirrorTile::Splitter(s) => {
                    if let Some(beams) = hit_splitter(beam, *s, p) {
                        return beams;
                    }
                }
                MirrorTile::MirrorLeft => return vec![hit_mirror_left(beam, p)],
                MirrorTile::MirrorRight => return vec![hit_mirror_right(beam, p)],
                MirrorTile::Ground => {}
            }
        } else {
            break;
        }
    }

    vec![]
}

fn parse_mirror(c: char) -> MirrorTile {
    match c {
        '.' => MirrorTile::Ground,
        '|' => MirrorTile::Splitter(&UP_DOWN_SPLITTER),
        '-' => MirrorTile::Splitter(&LEFT_RIGHT_SPLITTER),
        '/' => MirrorTile::MirrorLeft,
        '\\' => MirrorTile::MirrorRight,
        _ => panic!("invalid tile: {c}"),
    }
}

fn parse_grid(input: &str) -> DenseGrid<MirrorTile> {
    DenseGrid::parse(input, parse_mirror, None)
}

pub fn part_one(input: &str) -> Option<usize> {
    let board = parse_grid(input);

    let total_powered = calc_total_power(
        &board,
        Beam {
            start: XY(-1, 0),
            direction: RIGHT,
        },
    );

    Some(total_powered)
}

fn calc_total_power(board: &DenseGrid<MirrorTile>, initial_beam: Beam) -> usize {
    let mut energy_grid =
        DenseGrid::new_filled(board.width, board.height(), TileEnergy::empty(), None);
    let mut active_beams = VecDeque::new();
    active_beams.push_back(initial_beam);

    while let Some(beam) = active_beams.pop_front() {
        // println!("! processing beam: {beam:?}");
        for new_beam in propagate_beam(&beam, &board, &mut energy_grid) {
            // println!("+ new beam: {new_beam:?}");
            active_beams.push_back(new_beam);
        }

        // println!("{energy_grid}");
    }

    // println!("done!");

    energy_grid.items.iter().filter(|t| t.is_powered()).count()
}

pub fn part_two(input: &str) -> Option<usize> {
    let board = parse_grid(input);

    let top_edge = (0..board.width).map(|x| Beam {
        start: XY(x as i64, -1),
        direction: DOWN,
    });

    let bottom_edge = (0..board.width).map(|x| Beam {
        start: XY(x as i64, board.height() as i64),
        direction: UP,
    });

    let left_edge = (0..board.height()).map(|y| Beam {
        start: XY(-1, y as i64),
        direction: RIGHT,
    });

    let right_edge = (0..board.height()).map(|y| Beam {
        start: XY(board.width as i64, y as i64),
        direction: LEFT,
    });

    let max_energy = top_edge
        .chain(bottom_edge)
        .chain(left_edge)
        .chain(right_edge)
        .par_bridge()
        .map(|beam| calc_total_power(&board, beam))
        .max();

    max_energy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
