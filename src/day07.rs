use std::cmp::Ordering;
use std::collections::HashMap;

const CARDS: &str = "23456789TJQKA";

pub fn run(input: String) {
    let hands = parse_input(&input);

    println!("Part 1: {}", total_winnings(hands));
    println!("Part 2: {}", 42);
}

fn total_winnings(hands: Vec<(String, usize, usize)>) -> usize {
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.2)
        .sum()
}

fn parse_input(input: &str) -> Vec<(String, usize, usize)> {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut hand = HashMap::new();
            let (card_string, bid) = line.split_once(' ').expect("😭");
            card_string.chars().for_each(|char| {
                *hand.entry(char).or_insert(0) += 1;
            });
            let rank = score(&hand);
            (
                card_string.to_string(),
                rank,
                bid.parse::<usize>().expect("🙄"),
            )
        })
        .collect::<Vec<_>>();
    hands.sort_by(|(h1, s1, _), (h2, s2, _)| {
        if s1 == s2 {
            for i in 0..6 {
                let (c1, c2) = (h1.chars().nth(i).expect("!"), h2.chars().nth(i).expect("!"));
                if c1 != c2 {
                    return CARDS
                        .find(c1)
                        .expect("😐")
                        .cmp(&CARDS.find(c2).expect("😦"));
                }
            }
            Ordering::Equal
        } else {
            s1.cmp(s2)
        }
    });
    hands
}

fn score(hand: &HashMap<char, usize>) -> usize {
    match hand.len() {
        1 => 7, // full house,
        2 => {
            match hand.values().max().expect("🤭") {
                4 => 6, // 4 of a kind
                3 => 5, // full house
                _ => panic!("🤕"),
            }
        }
        3 => {
            match hand.values().max().expect("🤭") {
                3 => 4, // 3 of a kind
                2 => 3, // 2 pairs
                _ => panic!("🤕"),
            }
        }
        4 => 2,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day07_part_1() {
        let hands = parse_input(&read_input("test/day07"));
        dbg!(&hands);
        assert_eq!(total_winnings(hands), 6440);
    }

    #[test]
    fn test_day06_part_2() {
        assert_eq!(42, 43);
    }
}
