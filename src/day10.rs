use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub fn run(input: String) {
    let raw_map = parse_input(input);

    let loop_map = compute_loop(raw_map, '-');
    println!("Part 1: {}", loop_map.max_by(|tile| tile.position_in_loop));

    println!("Part 2: {}", 42);
}

fn compute_loop(map: Map<char>, start_tile: char) -> Map<SmartTile> {
    let mut smart_map = map
        .iter()
        .map(|(p, c)| {
            (
                *p,
                SmartTile {
                    kind: *c,
                    position_in_loop: if *c == 'S' { Some(0) } else { None },
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let start = smart_map
        .iter()
        .find_map(|(p, c)| if c.kind == 'S' { Some(*p) } else { None })
        .expect("☢️");
    let mut steps = 0;
    let mut current_positions = [start, start];
    let mut previous_positions = [start, start];
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
                .find(|pipe| !previous_positions.contains(pipe))
                .expect("🙄");
            previous_positions[i] = current_position;
            current_positions[i] = next_position;
            smart_map
                .get_mut(&next_position)
                .expect("😭")
                .position_in_loop = Some(steps);
        });
        if current_positions[0] == current_positions[1] {
            break;
        }
    }
    Map::new(smart_map)
}

struct SmartTile {
    kind: char,
    position_in_loop: Option<usize>,
}

impl Display for SmartTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.position_in_loop.is_some() {
            write!(f, "\x1b[93m{}\x1b[0m", self.position_in_loop.unwrap())
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

struct Map<T: Display> {
    map: HashMap<(isize, isize), T>,
    max_xy: (isize, isize),
}

impl<T: Display> Map<T> {
    pub fn new(map: HashMap<(isize, isize), T>) -> Self {
        let max_xy = *(map.keys().max().expect("😅"));
        Self { map, max_xy }
    }

    fn max_by(&self, by: fn(&T) -> Option<usize>) -> usize {
        self.map.values().filter_map(by).max().expect("😅")
    }

    fn iter(&self) -> Iter<'_, (isize, isize), T> {
        self.map.iter()
    }
}

impl<T: Display> Display for Map<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for y in 0..=self.max_xy.1 {
            for x in 0..=self.max_xy.0 {
                let tile = self.map.get(&(x, y)).expect("😅");
                output.push_str(&format!("{}", tile));
            }
            output.push('\n');
        }
        output.push('\n');
        f.write_str(&output)
    }
}

fn parse_input(input: String) -> Map<char> {
    Map::new(
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| ((x as isize, y as isize), c))
                    .collect::<Vec<_>>()
            })
            .collect::<HashMap<_, _>>(),
    )
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
        let raw_map = parse_input(read_input("test/day10.1"));
        print!("{}", raw_map);
        let loop_map = compute_loop(raw_map, 'F');
        print!("{}", loop_map);
        assert_eq!(loop_map.max_by(|tile| tile.position_in_loop), 8);
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
