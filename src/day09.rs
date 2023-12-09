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
            rows.iter()
                .rev()
                .map(|row| {
                    let last = *(if rev { row.first() } else { row.last() }).expect("️️☢️");
                    (last, if last == 0 { Some(0) } else { None })
                })
                .reduce(|acc, e| {
                    if rev {
                        (e.0, Some(e.0 - acc.1.unwrap()))
                    } else {
                        (e.0, Some(e.0 + acc.1.unwrap()))
                    }
                })
                .unwrap()
                .1
                .unwrap()
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
