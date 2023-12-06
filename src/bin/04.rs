use lazy_static::lazy_static;
use regex::Regex;
use advent_of_code::utils::parsing;
use rayon::prelude::*;

advent_of_code::solution!(4);

struct Card {
    winning_numbers: Vec<u32>,
    player_numbers: Vec<u32>,
}

lazy_static! {
    static ref CARD_RE: Regex = Regex::new(
        r"^Card .*: (?P<winning>.*) [|] (?P<player>.*)$"
    ).unwrap();
}

fn parse_card(line: &str) -> Option<Card> {
    CARD_RE.captures(line).and_then(
        |matches| Some(Card {
            winning_numbers: parsing::get_numbers(&matches["winning"]),
            player_numbers: parsing::get_numbers(&matches["player"]),
        })
    )
}

fn get_matches(card: &Card) -> usize {
    card.player_numbers.iter().filter(|n| card.winning_numbers.contains(n)).count()
}

fn get_points(card: &Card) -> usize {
    let total_matches = get_matches(card);

    if total_matches > 0 {
        usize::pow(2, (total_matches - 1) as u32)
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n")
            .par_bridge()
            .flat_map(|l| parse_card(l).and_then(|c| Some(get_points(&c))))
            .sum::<usize>() as u32
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // TODO: make this parallelized

    let cards: Vec<Card> = input.split("\n").flat_map(parse_card).collect();
    let points: Vec<usize> = cards.iter().map(get_matches).collect();
    let mut pending_cards: Vec<usize> = vec![1; cards.len()];
    let mut total_cards = pending_cards.len();

    for card_num in 0..cards.len() {
        let count = pending_cards[card_num];
        let card_points = points[card_num];
        for c in card_num + 1..=card_num + card_points {
            total_cards += count;
            pending_cards[c] += count;
        }
    }

    Some(total_cards as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
