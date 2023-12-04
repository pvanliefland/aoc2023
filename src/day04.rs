pub fn run(input: String) {
    let cards = parse_input(input);

    println!("Part 1: {}", total_card_points(&cards));
    println!("Part 2: {}", "?");
}

fn total_card_points(cards: &[Card]) -> i32 {
    cards
        .iter()
        .map(|card| {
            card.2.iter().fold(0, |acc, e| {
                if card.1.contains(e) {
                    match acc {
                        0 => 1,
                        other => 2 * other,
                    }
                } else {
                    acc
                }
            })
        })
        .sum()
}

fn parse_input(input: String) -> Vec<Card> {
    input
        .replace("  ", " ")
        .lines()
        .map(|line| {
            let (card_info, numbers) = line.split_once(':').expect("🤪");
            let numbers: Vec<Vec<i32>> = numbers
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
                card_info.to_string(),
                numbers[0].clone(),
                numbers[1].clone(),
            )
        })
        .collect()
}

type Card = (String, Vec<i32>, Vec<i32>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day04_part_1() {
        assert_eq!(
            total_card_points(&parse_input(read_input("day04.test"))),
            13
        );
    }

    #[test]
    fn test_day04_part_2() {}
}
