use std::collections::HashMap;

pub fn run(input: String) {
    let navigator = parse_input(input, '-');

    println!("Part 1: {}", longest_segment(navigator));
    println!("Part 2: {}", 42);
}

fn longest_segment(mut navigator: Navigator) -> usize {
    loop {
        if let Err(steps) = navigator.step() {
            break steps;
        }
    }
}

fn parse_input(input: String, start_tile: char) -> Navigator {
    let mut start = None;
    Navigator::new(
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = Some((x as isize, y as isize));
                        }
                        ((x as isize, y as isize), c)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashMap<_, _>>(),
        start.expect("🤪"),
        start_tile,
    )
}

struct Navigator {
    map: HashMap<Position, char>,
    start: Position,
    start_tile: char,
    current: [Position; 2],
    prev: [Position; 2],
    steps: usize,
}
impl Navigator {
    pub fn new(map: HashMap<Position, char>, start: Position, start_tile: char) -> Self {
        Self {
            map,
            start,
            start_tile,
            current: [start, start],
            prev: [start, start],
            steps: 0,
        }
    }

    fn step(&mut self) -> Result<(), usize> {
        self.steps += 1;
        [0usize, 1].into_iter().for_each(|i| {
            let current = self.current[i];
            let tile = if current == self.start {
                self.start_tile
            } else {
                *self.map.get(&current).expect("😅")
            };
            let possible_moves = match tile {
                '|' => [(0, 1), (0, -1)],
                '-' => [(1, 0), (-1, 0)],
                'L' => [(0, -1), (1, 0)],
                'J' => [(0, -1), (-1, 0)],
                '7' => [(0, 1), (-1, 0)],
                'F' => [(0, 1), (1, 0)],
                _ => panic!("{} 😰", tile),
            };
            let (dx, dy) = possible_moves
                .iter()
                .find(|(dx, dy)| {
                    (self.steps > 1 || !self.current.contains(&(current.0 + dx, current.1 + dy)))
                        && !self.prev.contains(&(current.0 + dx, current.1 + dy))
                })
                .expect("🙄");
            self.prev[i] = current;
            self.current[i] = (current.0 + dx, current.1 + dy);
        });
        if self.current[0] == self.current[1] {
            Err(self.steps)
        } else {
            Ok(())
        }
    }
}
type Position = (isize, isize);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day10_part1() {
        assert_eq!(
            longest_segment(parse_input(read_input("test/day10"), 'F')),
            8
        );
    }

    #[test]
    fn test_day10_part2() {
        todo!()
    }
}
