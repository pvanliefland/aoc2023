use std::ops::Add;

pub fn run(input: String) {
    let possible_games = possible_games(input, Draw(12, 13, 14));

    println!("{:?}", possible_games);
    println!("{}", possible_games.iter().sum::<usize>());
}

#[derive(Debug)]
struct Draw(u8, u8, u8);

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
}

fn possible_games(input: String, max_draw: Draw) -> Vec<usize> {
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
                            let num: u8 = num.parse().expect("🤯");
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
        .filter_map(|(game_id, draw)| {
            if draw.0 <= max_draw.0 && draw.1 <= max_draw.1 && draw.2 <= max_draw.2 {
                Some(game_id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::day02::{possible_games, Draw};
    use crate::read_input;

    #[test]
    fn test_day02_part_1() {
        assert_eq!(
            possible_games(read_input("day02.test"), Draw(12, 13, 14))
                .iter()
                .sum::<usize>(),
            8
        );
    }
}
