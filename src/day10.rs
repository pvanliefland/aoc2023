use std::collections::HashMap;

pub fn run(input: String) {
    let raw_map = parse_input(input);

    let loop_map = compute_loop(raw_map, '-');
    let steps = max_by(&loop_map, |tile| tile.position_in_loop);
    println!("Part 1: {}", steps);

    println!("Part 2: {}", 42);
}

fn compute_loop(
    map: HashMap<(isize, isize), char>,
    start_tile: char,
) -> HashMap<(isize, isize), SmartTile> {
    let mut smart_map = map
        .into_iter()
        .map(|(p, c)| {
            (
                p,
                SmartTile {
                    kind: c,
                    position_in_loop: if c == 'S' { Some(0) } else { None },
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let start = smart_map
        .iter()
        .find_map(|(p, c)| if c.kind == 'S' { Some(*p) } else { None })
        .expect("☢️");
    let mut steps = 0;
    let mut current_positions = vec![start, start];
    let mut previous_positions = vec![start, start];
    loop {
        steps += 1;
        [0usize, 1].iter().for_each(|&i| {
            let current_position = current_positions[i];
            let tile = if current_position == start {
                start_tile
            } else {
                smart_map.get(&current_position).expect("😅").kind
            };
            let mut possible_next_positions = (match tile {
                '|' => [(0, 1), (0, -1)],
                '-' => [(1, 0), (-1, 0)],
                'L' => [(0, -1), (1, 0)],
                'J' => [(0, -1), (-1, 0)],
                '7' => [(0, 1), (-1, 0)],
                'F' => [(0, 1), (1, 0)],
                _ => panic!("{} 😰", tile),
            })
            .map(|(dx, dy)| (current_position.0 + dx, current_position.1 + dy));
            if steps == 1 && i == 1 {
                possible_next_positions.reverse();
            }

            let next_position = *possible_next_positions
                .iter()
                .find(|pipe| !previous_positions.contains(&pipe))
                .expect("🙄");
            previous_positions[i] = current_position;
            current_positions[i] = next_position;
            smart_map.insert(
                current_position,
                SmartTile {
                    kind: 'x',
                    position_in_loop: Some(steps),
                },
            );
        });
        if current_positions[0] == current_positions[1] {
            break;
        }
    }
    smart_map
}

struct SmartTile {
    kind: char,
    position_in_loop: Option<usize>,
}

struct Map<T> {
    map: HashMap<(isize, isize), T>,
}

fn max_by<T>(map: &HashMap<(isize, isize), T>, by: fn(&T) -> Option<usize>) -> usize {
    map.values().filter_map(by).max().expect("😅")
}

fn parse_input(input: String) -> HashMap<(isize, isize), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| ((x as isize, y as isize), c))
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>()
}

/*

    fn enclosed(&self) -> usize {
        let enclosed = self
            .map
            .iter()
            .filter(|(p, _)| !self.loop_coordinates.contains(p))
            .filter(|(p, _)| {
                (p.0..self.max_xy.0)
                    .filter(|&x| {
                        let ray_pos = (x, p.1);
                        self.loop_coordinates.contains(&ray_pos)
                            && self.map.get(&ray_pos).is_some_and(|t| {
                                ['|', 'F', '7'].contains(t)
                                    || ray_pos == self.start
                                        && ['|', 'F', '7'].contains(&self.start_tile)
                            })
                    })
                    .count()
                    % 2
                    == 1
            })
            .collect::<Vec<_>>();
        dbg!(&enclosed);

        enclosed.len()
    }
}
*/
#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day10_part1() {
        assert_eq!(
            max_by(
                &compute_loop(parse_input(read_input("test/day10.1")), 'F'),
                |tile| tile.position_in_loop
            ),
            8
        );
    }

    /*#[ignore]
    #[test]
    fn test_day10_part21() {
        let mut navigator = parse_input(read_input("test/day10.2.1"), 'F');
        navigator.longest_segment();
        assert_eq!(navigator.enclosed(), 4);
    }

    #[ignore]
    #[test]
    fn test_day10_part22() {
        let mut navigator = parse_input(read_input("test/day10.2.2"), 'F');
        navigator.longest_segment();
        assert_eq!(navigator.enclosed(), 4);
    }

    #[ignore]
    #[test]
    fn test_day10_part23() {
        let mut navigator = parse_input(read_input("test/day10.2.3"), 'F');
        navigator.longest_segment();
        assert_eq!(navigator.enclosed(), 8);
    }

    #[ignore]
    #[test]
    fn test_day10_part24() {
        let mut navigator = parse_input(read_input("test/day10.2.4"), '7');
        navigator.longest_segment();
        assert_eq!(navigator.enclosed(), 10);
    }*/
}
