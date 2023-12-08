use std::collections::HashMap;

pub fn run(input: String) {
    println!("Part 1: {}", count_steps(&input));
    println!("Part 2: {}", todo!());
}

fn count_steps(input: &str) -> usize {
    let directions = input.lines().next().expect("☢️").chars().cycle();
    let nodes = input
        .lines()
        .skip(2)
        .map(|line| (&line[0..3], (&line[7..10], &line[12..15])))
        .collect::<HashMap<_, _>>();

    let mut current_node = "AAA";
    directions
        .enumerate()
        .find_map(|(step, direction)| {
            let directions = nodes.get(current_node).expect("🤭");
            current_node = match direction {
                'L' => directions.0,
                'R' => directions.1,
                _ => panic!("🤕"),
            };
            if current_node == "ZZZ" {
                Some(step + 1)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day08_part_1() {
        assert_eq!(count_steps(&read_input("test/day08.1a")), 2);
        assert_eq!(count_steps(&read_input("test/day08.1b")), 6);
    }

    #[test]
    fn test_day08_part_2() {
        todo!()
    }
}
