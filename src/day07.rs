use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn run(input: String) {
    let hands = parse_input(&input);

    println!("Part 1: {}", total_winnings(&hands, false));
    println!("Part 2: {}", total_winnings(&hands, true));
}

fn total_winnings(hands: &[(String, usize)], jokers: bool) -> usize {
    let card_values = if jokers {
        "J23456789TQKA"
    } else {
        "23456789TJQKA"
    };
    let mut hands_with_scores = hands
        .iter()
        .map(|(hand, bid)| {
            let score = if jokers && hand.contains('J') {
                hand.chars()
                    .collect::<HashSet<_>>()
                    .iter()
                    .filter(|&&card| card != 'J')
                    .map(|alt_card| score(&hand.replace('J', &alt_card.to_string())))
                    .max()
                    .unwrap_or(score(hand))
            } else {
                score(hand)
            };

            (hand, bid, score)
        })
        .collect::<Vec<_>>();

    hands_with_scores.sort_by(|(h1, _, s1), (h2, _, s2)| {
        if s1 == s2 {
            h1.chars()
                .zip(h2.chars())
                .map(|(c1, c2)| (card_values.find(c1).unwrap(), card_values.find(c2).unwrap()))
                .find_map(|(c1, c2)| if c1 != c2 { Some(c1.cmp(&c2)) } else { None })
                .unwrap_or(Ordering::Equal)
        } else {
            s1.cmp(s2)
        }
    });

    hands_with_scores
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, &bid, _))| acc + (i + 1) * bid)
}

fn score(hand: &str) -> usize {
    let mut counter = HashMap::new();
    hand.chars().for_each(|char| {
        *counter.entry(char).or_insert(0) += 1;
    });
    match (counter.len(), counter.values().max().expect("🤭")) {
        (1, _) => 7, // full house,
        (2, 4) => 6, // 4 of a kind
        (2, 3) => 5, // full house
        (3, 3) => 4, // 3 of a kind
        (3, 2) => 3, // 2 pairs
        (4, _) => 2, // 1 pair
        _ => 1,
    }
}

fn parse_input(input: &str) -> Vec<(String, usize)> {
    input
        .lines()
        .map(|line| {
            let (card_string, bid) = line.split_once(' ').expect("😭");
            (card_string.to_string(), bid.parse::<usize>().expect("🙄"))
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day07_part_1() {
        assert_eq!(
            total_winnings(&parse_input(&read_input("test/day07")), false),
            6440
        );
    }

    #[test]
    fn test_day06_part_2() {
        assert_eq!(
            total_winnings(&parse_input(&read_input("test/day07")), true),
            5905
        );
    }
}
