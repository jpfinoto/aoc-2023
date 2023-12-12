use lazy_static::lazy_static;
use regex::Regex;
advent_of_code::solution!(2);

struct CubeDraw {
    red: u32,
    blue: u32,
    green: u32,
}

struct Game {
    number: u32,
    draws: Vec<CubeDraw>,
}

lazy_static! {
    static ref CUBE_RE: Regex = Regex::new(r"(?P<count>\d+) (?P<color>red|green|blue)").unwrap();
    static ref BASE_RE: Regex = Regex::new(r"^Game (?P<n>\d+):(?P<draws>.*)$").unwrap();
}

fn parse_draw(draw: &str) -> CubeDraw {
    let mut cubes = CubeDraw {
        red: 0,
        blue: 0,
        green: 0,
    };

    for draw in CUBE_RE.captures_iter(draw) {
        let count = u32::from_str_radix(&draw["count"], 10).unwrap();
        match &draw["color"] {
            "red" => cubes.red = count,
            "green" => cubes.green = count,
            "blue" => cubes.blue = count,
            _ => {}
        }
    }

    cubes
}

fn parse_game(line: &str) -> Option<Game> {
    let base_caps = match BASE_RE.captures(line) {
        Some(value) => value,
        None => return None,
    };

    let game_num = u32::from_str_radix(&base_caps["n"], 10).unwrap();

    let draws: Vec<_> = base_caps["draws"].split(";").map(parse_draw).collect();

    Some(Game {
        number: game_num,
        draws,
    })
}

fn is_game_valid(game: &Game) -> bool {
    game.draws
        .iter()
        .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
}

fn get_game_power(game: &Game) -> u32 {
    let min_red = game.draws.iter().map(|draw| draw.red).max().unwrap();
    let min_green = game.draws.iter().map(|draw| draw.green).max().unwrap();
    let min_blue = game.draws.iter().map(|draw| draw.blue).max().unwrap();

    min_red * min_green * min_blue
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.split("\n").collect();
    let games: Vec<_> = lines.into_iter().map(parse_game).flatten().collect();
    let valid_games: Vec<_> = games.into_iter().filter(is_game_valid).collect();

    Some(valid_games.iter().map(|game| game.number).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.split("\n").collect();
    let games: Vec<_> = lines.into_iter().map(parse_game).flatten().collect();
    let powers: Vec<_> = games.iter().map(get_game_power).collect();

    Some(powers.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
