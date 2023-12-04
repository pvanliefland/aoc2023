use std::collections::{HashMap, HashSet};

pub fn run(input: String) {
    let cards = parse_input(input);

    println!("Part 1: {}", total_card_points(&cards));
    println!("Part 2: {}", total_scratch_cards(&cards));
}

fn total_card_points(cards: &[Card]) -> usize {
    cards
        .iter()
        .map(|card| match card.1 {
            0 => 0,
            n => 2usize.pow((n - 1) as u32),
        })
        .sum()
}

fn total_scratch_cards(cards: &[Card]) -> usize {
    let mut counts: HashMap<usize, usize> = cards.iter().map(|card| (card.0, 1)).collect();
    cards.iter().for_each(|card| {
        ((card.0 + 1)..=(card.0 + card.1)).for_each(|card_number| {
            let factor = *counts.get(&card.0).expect("🙄");
            *counts.get_mut(&card_number).expect("🙄") += factor;
        })
    });
    counts.values().sum()
}

fn parse_input(input: String) -> Vec<Card> {
    input
        .replace("  ", " ")
        .lines()
        .map(|line| {
            let (card_info, numbers) = line.split_once(':').expect("🤪");
            let numbers: Vec<HashSet<i32>> = numbers
                .trim()
                .split(" | ")
                .map(|numbers| {
                    numbers
                        .split(' ')
                        .map(|number| number.parse().expect("🤬"))
                        .collect()
                })
                .collect();
            (
                card_info[5..].trim().parse().expect("🙏"),
                numbers[1].intersection(&numbers[0]).count(),
            )
        })
        .collect()
}

type Card = (usize, usize);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day04_part_1() {
        assert_eq!(
            total_card_points(&parse_input(read_input("test/day04"))),
            13
        );
    }

    #[test]
    fn test_day04_part_2() {
        assert_eq!(
            total_scratch_cards(&parse_input(read_input("test/day04"))),
            30
        );
    }
}
