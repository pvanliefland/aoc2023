use std::ops::Add;

pub fn run(input: String) {
    let games = parse_input(input);

    // Part 1
    println!(
        "Part 1: {}",
        sum_of_possible_games(&games, Draw(12, 13, 14))
    );

    // Part 2
    println!("Part 2: {}", sum_of_minimum_draw_powers(&games));
}

fn parse_input(input: String) -> Vec<(usize, Draw)> {
    input
        .lines()
        .map(|line| {
            let (game, draw_data) = line.split_once(": ").expect("😿");
            let game_id: usize = game[5..].parse().expect("🙄");
            (
                game_id,
                draw_data
                    .split("; ")
                    .map(|draw| {
                        draw.split(", ").fold(Draw(0, 0, 0), |draw, num_for_color| {
                            let (num, color) = num_for_color.split_once(' ').expect("😿");
                            let num: usize = num.parse().expect("🤯");
                            match color {
                                "red" => draw + Draw(num, 0, 0),
                                "green" => draw + Draw(0, num, 0),
                                "blue" => draw + Draw(0, 0, num),
                                _ => panic!("🤬"),
                            }
                        })
                    })
                    .reduce(|acc, e| acc.max(&e))
                    .expect("🙄"),
            )
        })
        .collect()
}

fn sum_of_possible_games(games: &[(usize, Draw)], max_possible_draw: Draw) -> usize {
    games
        .iter()
        .filter_map(|(game_id, draw)| {
            if draw.0 <= max_possible_draw.0
                && draw.1 <= max_possible_draw.1
                && draw.2 <= max_possible_draw.2
            {
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}

fn sum_of_minimum_draw_powers(games: &[(usize, Draw)]) -> usize {
    games.iter().map(|(_, max_draw)| max_draw.power()).sum()
}

#[derive(Debug)]
struct Draw(usize, usize, usize);

impl Add for Draw {
    type Output = Draw;

    fn add(self, rhs: Self) -> Self::Output {
        Draw(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Draw {
    fn max(&self, other: &Draw) -> Self {
        Self(
            self.0.max(other.0),
            self.1.max(other.1),
            self.2.max(other.2),
        )
    }

    fn power(&self) -> usize {
        self.0 * self.1 * self.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day02_part_1() {
        let games = parse_input(read_input("day02.test"));
        assert_eq!(sum_of_possible_games(&games, Draw(12, 13, 14)), 8);
    }

    #[test]
    fn test_day02_part_2() {
        let games = parse_input(read_input("day02.test"));
        assert_eq!(sum_of_minimum_draw_powers(&games), 2286);
    }
}
