use std::collections::HashSet;

pub fn run(input: String) {
    let springs_condition = parse_input(&input);
    let arrangements = find_arrangements(springs_condition);

    println!("Part 1: {}", arrangements.len());

    println!("Part 1: {}", 42);
}

fn find_arrangements(springs_conditions: Vec<(Vec<char>, Vec<usize>)>) -> Vec<Vec<char>> {
    let mut arrangements = vec![];
    for (springs, counts) in &springs_conditions {
        let unknown_count = springs.iter().filter(|&&s| s == '?').count();
        for mut permutation in permutations(unknown_count) {
            let candidate_condition = springs
                .iter()
                .map(|&s| {
                    if s == '?' {
                        permutation.pop().expect("🙄")
                    } else {
                        s
                    }
                })
                .collect::<Vec<_>>();
            // test the condition by counting consecutive broken springs
            let candidate_string = candidate_condition.iter().collect::<String>();
            let candidate_counts = candidate_string
                .split('.')
                .filter(|p| !p.is_empty())
                .map(|c| c.len())
                .collect::<Vec<_>>();
            if candidate_counts == *counts {
                arrangements.push(candidate_condition);
            }
        }
    }

    arrangements
}

fn permutations(num: usize) -> HashSet<Vec<char>> {
    if num == 0 {
        HashSet::new()
    } else if num == 1 {
        HashSet::from([vec!['.'], vec!['#']])
    } else {
        ['.', '#']
            .iter()
            .flat_map(|&c| {
                permutations(num - 1)
                    .into_iter()
                    .map(|p| [vec![c], p].concat())
                    .collect::<HashSet<_>>()
            })
            .collect()
    }
}

fn parse_input(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').expect("🙄");
            (
                springs.chars().collect(),
                counts.split(',').map(|c| c.parse().expect("☢️")).collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_permutations() {
        assert_eq!(permutations(0), HashSet::new());
        assert_eq!(permutations(1), HashSet::from([vec!['.'], vec!['#']]));
        assert_eq!(
            permutations(2),
            HashSet::from([
                vec!['.', '.'],
                vec!['.', '#'],
                vec!['#', '.'],
                vec!['#', '#'],
            ])
        );
        assert_eq!(
            permutations(3),
            HashSet::from([
                vec!['.', '.', '.'],
                vec!['#', '.', '.'],
                vec!['.', '#', '.'],
                vec!['.', '.', '#'],
                vec!['#', '#', '.'],
                vec!['.', '#', '#'],
                vec!['#', '.', '#'],
                vec!['#', '#', '#'],
            ])
        );
    }

    #[test]
    fn test_day12_part1() {
        let arrangements = find_arrangements(parse_input(&read_input("test/day12")));
        assert_eq!(arrangements.len(), 21);
    }

    #[test]
    fn test_day112_part2() {}
}
