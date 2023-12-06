pub fn run(input: String) {
    println!(
        "Part 1: {}",
        product_of_winning_strategies(parse_input(&input, false))
    );
    println!(
        "Part 2: {}",
        product_of_winning_strategies(parse_input(&input, true))
    );
}
fn parse_input(input: &str, fix_kerning: bool) -> [Vec<usize>; 2] {
    input
        .lines()
        .map(|line| {
            let mut race_data = line.split_once(':').expect("🤞").1.to_string();
            if fix_kerning {
                race_data = race_data.replace(' ', "");
            }
            race_data
                .split_whitespace()
                .map(|part| part.parse().expect("🤷"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("🙏")
}

fn product_of_winning_strategies(race_data: [Vec<usize>; 2]) -> usize {
    (0..race_data[0].len())
        .map(|game| {
            (1..race_data[0][game])
                .filter(|speed| speed * (race_data[0][game] - speed) > race_data[1][game])
                .count()
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day06_part_1() {
        assert_eq!(
            product_of_winning_strategies(parse_input(&read_input("test/day06"), false)),
            288
        );
    }

    #[test]
    fn test_day06_part_2() {
        assert_eq!(
            product_of_winning_strategies(parse_input(&read_input("test/day06"), true)),
            71503
        );
    }
}
