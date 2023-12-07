use std::cmp::Ordering;
use std::collections::HashMap;

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
            let mut counter = HashMap::new();
            hand.chars().for_each(|char| {
                *counter.entry(char).or_insert(0) += 1;
            });

            let score = if jokers && hand.contains('J') {
                counter
                    .keys()
                    .filter(|&&card| card != 'J')
                    .map(|card| {
                        let mut counter = HashMap::new();
                        hand.replace('J', &card.to_string())
                            .chars()
                            .for_each(|char| {
                                *counter.entry(char).or_insert(0) += 1;
                            });
                        score(&counter)
                    })
                    .max()
                    .unwrap_or(score(&counter))
            } else {
                score(&counter)
            };

            (hand, bid, score)
        })
        .collect::<Vec<_>>();

    hands_with_scores.sort_by(|(h1, _, s1), (h2, _, s2)| {
        if s1 == s2 {
            for i in 0..6 {
                let (c1, c2) = (h1.chars().nth(i).expect("!"), h2.chars().nth(i).expect("!"));
                if c1 != c2 {
                    return card_values
                        .find(c1)
                        .expect("😐")
                        .cmp(&card_values.find(c2).expect("😦"));
                }
            }
            Ordering::Equal
        } else {
            s1.cmp(s2)
        }
    });

    hands_with_scores
        .iter()
        .enumerate()
        .map(|(rank, (_, &bid, _))| (rank + 1) * bid)
        .sum()
}

fn score(hand: &HashMap<char, usize>) -> usize {
    match (hand.len(), hand.values().max().expect("🤭")) {
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
