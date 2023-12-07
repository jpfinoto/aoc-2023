use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(7);


#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
    Joker,
}

fn parse_card(card: &str) -> Result<Card, ()> {
    match card {
        "A" => Ok(Card::A),
        "K" => Ok(Card::K),
        "Q" => Ok(Card::Q),
        "J" => Ok(Card::J),
        "T" => Ok(Card::T),
        "9" => Ok(Card::N9),
        "8" => Ok(Card::N8),
        "7" => Ok(Card::N7),
        "6" => Ok(Card::N6),
        "5" => Ok(Card::N5),
        "4" => Ok(Card::N4),
        "3" => Ok(Card::N3),
        "2" => Ok(Card::N2),
        _ => Err(()),
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    High,
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

fn get_hand_type(cards: &[Card; 5]) -> HandType {
    let mut cards_by_type = HashMap::new();

    for card in cards {
        *cards_by_type.entry(card).or_insert(0) += 1;
    }

    let ordered = cards_by_type.values().sorted().rev().cloned().collect_vec();

    match ordered.as_slice() {
        [5] => HandType::FiveOfAKind,
        [4, 1] => HandType::FourOfAKind,
        [3, 2] => HandType::FullHouse,
        [3, 1, 1] => HandType::ThreeOfAKind,
        [2, 2, 1] => HandType::TwoPair,
        [2, 1, 1, 1] => HandType::Pair,
        [1, 1, 1, 1, 1] => HandType::High,
        _ => panic!(),
    }
}


impl Hand {
    fn get_type(&self) -> HandType {
        let num_jokers =
            self.cards
                .into_iter()
                .filter(|&c| c == Card::Joker)
                .count();

        let base_hand = get_hand_type(&self.cards);

        match (base_hand, num_jokers) {
            (HandType::FiveOfAKind, _) => HandType::FiveOfAKind,

            (HandType::FourOfAKind, 4) => HandType::FiveOfAKind,
            (HandType::FourOfAKind, 1) => HandType::FullHouse,
            (HandType::FourOfAKind, 0) => HandType::FourOfAKind,

            (HandType::FullHouse, 3) => HandType::FiveOfAKind,
            (HandType::FullHouse, 2) => HandType::FiveOfAKind,
            (HandType::FullHouse, 0) => HandType::FullHouse,

            (HandType::TwoPair, 2) => HandType::FourOfAKind,
            (HandType::TwoPair, 1) => HandType::ThreeOfAKind,
            (HandType::TwoPair, 0) => HandType::TwoPair,

            (HandType::ThreeOfAKind, 3) => HandType::FourOfAKind,
            (HandType::ThreeOfAKind, 1) => HandType::FourOfAKind,
            (HandType::ThreeOfAKind, 0) => HandType::FourOfAKind,

            (HandType::Pair, 2) => HandType::Pair,
            (HandType::Pair, 1) => HandType::ThreeOfAKind,
            (HandType::Pair, 0) => HandType::Pair,

            (HandType::High, 1) => HandType::Pair,
            (HandType::High, 0) => HandType::High,

            (_, _) => panic!("not possible"),
        }
    }

    fn parse(line: &str) -> Result<Hand, ()> {
        let parts = line.split(" ").collect_vec();

        if parts.len() != 2 {
            return Err(());
        }

        let cards = parts[0]
            .split("")
            .flat_map(parse_card)
            .collect_vec()
            .try_into()
            .unwrap();

        Ok(
            Hand {
                cards,
                bid: u32::from_str_radix(parts[1], 10).unwrap(),
            }
        )
    }

    fn change_jokers(&self) -> Hand {
        Hand {
            cards: self.cards.iter().map(|c| match c {
                Card::J => Card::Joker,
                other => *other,
            }).collect_vec().try_into().unwrap(),
            bid: self.bid,
        }
    }

    fn compare_high_card(&self, other: &Hand) -> Ordering {
        self.cards.cmp(&other.cards)
    }
}

fn sort_hands(
    (hand1, hand1_type): &(&Hand, HandType),
    (hand2, hand2_type): &(&Hand, HandType),
) -> Ordering {
    match hand1_type.cmp(hand2_type) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => hand1.compare_high_card(hand2)
    }
}

fn get_sorted_hands(hands: &Vec<Hand>) -> Vec<(usize, (&Hand, HandType))> {
    hands
        .iter()
        .map(|hand| (hand, hand.get_type()))
        .sorted_by(sort_hands)
        .rev()
        .enumerate()
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands = input.split("\n")
        .flat_map(Hand::parse)
        .collect_vec();

    let sorted_hands = get_sorted_hands(&hands);

    // for x in sorted_hands {
    //     println!("{:?}", x);
    // }

    Some(
        sorted_hands
            .iter()
            .map(|(rank, (hand, _))| (*rank as u32 + 1) * hand.bid)
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands = input.split("\n")
        .flat_map(|s| Hand::parse(s).and_then(|hand| Ok(hand.change_jokers())))
        .collect_vec();

    let sorted_hands = get_sorted_hands(&hands);

    Some(
        sorted_hands
            .iter()
            .map(|(rank, (hand, _))| (*rank as u32 + 1) * hand.bid)
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
