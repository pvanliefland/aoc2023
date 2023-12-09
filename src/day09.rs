pub fn run(input: String) {
    let sequences = parse_input(input);

    println!("Part 1: {}", sum_of_predicted_values(sequences));
    println!("Part 2: {}", 42);
}

fn sum_of_predicted_values(sequences: Vec<Vec<isize>>) -> isize {
    sequences
        .into_iter()
        .map(|sequence| {
            let mut rows = vec![sequence.clone()];
            let mut index = 0;
            loop {
                let next = rows[index]
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<_>>();
                rows.push(next.clone());
                index += 1;
                if next.iter().all(|&value| value == 0) {
                    break;
                }
            }
            let next_value = rows
                .iter()
                .map(|row| *row.last().unwrap())
                .reduce(|acc, e| acc + e)
                .unwrap();
            next_value
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
            sum_of_predicted_values(parse_input(read_input("test/day09"))),
            114
        );
    }
    #[test]
    fn test_day09_part2() {
        todo!()
    }
}
