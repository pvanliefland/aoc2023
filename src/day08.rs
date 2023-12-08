use std::collections::HashMap;

pub fn run(input: String) {
    let (directions, map) = parse_input(input);

    println!("Part 1: {}", count_steps("AAA", "ZZZ", &directions, &map));
    println!("Part 2: {}", count_steps_for_ghosts(&directions, &map));
}

fn count_steps(start: &str, ending: &str, directions: &[char], map: &Map) -> usize {
    let mut current = map.get_key_value(start).expect("😅");
    directions
        .iter()
        .cycle()
        .enumerate()
        .find(|(_, direction)| {
            current = map
                .get_key_value(match direction {
                    'L' => &current.1 .0,
                    'R' => &current.1 .1,
                    _ => panic!("🤕"),
                })
                .expect("🤯");
            current.0.ends_with(ending)
        })
        .expect("☢️")
        .0
        + 1
}

fn count_steps_for_ghosts(directions: &[char], map: &Map) -> usize {
    map.iter()
        .filter(|(node, _)| node.ends_with('A'))
        .map(|(node, _)| count_steps(node, "Z", directions, map))
        .reduce(|a, b| (a * b) / gcd(a, b))
        .expect("😅")
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a.rem_euclid(b))
    }
}

type Map = HashMap<String, (String, String)>;

fn parse_input(input: String) -> (Vec<char>, HashMap<String, (String, String)>) {
    (
        input.lines().next().expect("☢️").chars().collect::<Vec<_>>(),
        input
            .lines()
            .skip(2)
            .map(|line| {
                (
                    line[0..3].to_string(),
                    (line[7..10].to_string(), line[12..15].to_string()),
                )
            })
            .collect::<HashMap<_, _>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day08_part_1a() {
        let (directions, map) = parse_input(read_input("test/day08.1a"));
        assert_eq!(count_steps("AAA", "ZZZ", &directions, &map), 2);
    }
    #[test]
    fn test_day08_part_1b() {
        let (directions, map) = parse_input(read_input("test/day08.1b"));
        assert_eq!(count_steps("AAA", "ZZZ", &directions, &map), 6);
    }

    #[test]
    fn test_day08_part_2() {
        let (directions, map) = parse_input(read_input("test/day08.2"));
        assert_eq!(count_steps_for_ghosts(&directions, &map), 6);
    }
}
