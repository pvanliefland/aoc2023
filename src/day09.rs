pub fn run(input: String) {
    let sequences = parse_input(input);

    println!("Part 1: {}", sum_of_predicted_values(&sequences, false));
    println!("Part 2: {}", sum_of_predicted_values(&sequences, true));
}

fn sum_of_predicted_values(sequences: &[Vec<isize>], rev: bool) -> isize {
    sequences
        .iter()
        .map(|sequence| {
            let mut rows = vec![sequence.clone()];
            while !rows[rows.len() - 1].iter().all(|&value| value == 0) {
                rows.push(
                    rows[rows.len() - 1]
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect::<Vec<_>>(),
                );
            }
            rows.iter()
                .rev()
                .map(|row| *(if rev { row.first() } else { row.last() }).expect("️️☢️"))
                .fold(0, |acc, e| if rev { e - acc } else { e + acc })
        })
        .sum()
}

fn parse_input(input: String) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|n| n.parse().expect("🤕")).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day09_part1() {
        assert_eq!(
            sum_of_predicted_values(&parse_input(read_input("test/day09")), false),
            114
        );
    }

    #[test]
    fn test_day09_part2() {
        assert_eq!(
            sum_of_predicted_values(&parse_input(read_input("test/day09")), true),
            2
        );
    }
}
