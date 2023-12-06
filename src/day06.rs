pub fn run(input: String) {
    println!("Part 1: {}", winning_score(parse_input(&input, false)));
    println!("Part 2: {}", winning_score(parse_input(&input, true)));
}

fn winning_score(race_data: Vec<Vec<usize>>) -> usize {
    (0..race_data[0].len())
        .map(|game| {
            let (time, record) = (race_data[0][game] as f64, race_data[1][game] as f64);
            let sd = (time.powf(2.) - 4. * record).sqrt();
            (((-time - sd) / -2. - 1.).ceil() - ((-time + sd) / -2. + 1.).floor() + 1.) as usize
        })
        .product()
}

fn parse_input(input: &str, fix_kerning: bool) -> Vec<Vec<usize>> {
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
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day06_part_1() {
        assert_eq!(
            winning_score(parse_input(&read_input("test/day06"), false)),
            288
        );
    }

    #[test]
    fn test_day06_part_2() {
        assert_eq!(
            winning_score(parse_input(&read_input("test/day06"), true)),
            71503
        );
    }
}
