use std::collections::{HashMap, HashSet};

pub fn run(input: String) {
    let mut navigator = parse_input(input, '-');

    println!("Part 1: {}", navigator.longest_segment());
    println!("Part 2: {}", 42);
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
    loop_coordinates: HashSet<Position>,
    max_xy: Position,
}
impl Navigator {
    pub fn new(map: HashMap<Position, char>, start: Position, start_tile: char) -> Self {
        let max_xy = *(map.keys().max().expect("☢️"));
        Self {
            map,
            start,
            start_tile,
            current: [start, start],
            prev: [start, start],
            steps: 0,
            loop_coordinates: HashSet::from([start]),
            max_xy,
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
            self.loop_coordinates.insert(self.current[i]);
        });
        if self.current[0] == self.current[1] {
            Err(self.steps)
        } else {
            Ok(())
        }
    }

    fn longest_segment(&mut self) -> usize {
        loop {
            if let Err(steps) = self.step() {
                break steps;
            }
        }
    }

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
type Position = (isize, isize);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day10_part1() {
        let mut navigator = parse_input(read_input("test/day10.1"), 'F');
        assert_eq!(navigator.longest_segment(), 8);
    }

    #[test]
    fn test_day10_part21() {
        let mut navigator = parse_input(read_input("test/day10.2.1"), 'F');
        navigator.longest_segment();
        assert_eq!(navigator.enclosed(), 4);
    }

    #[test]
    fn test_day10_part22() {
        let mut navigator = parse_input(read_input("test/day10.2.2"), 'F');
        navigator.longest_segment();
        assert_eq!(navigator.enclosed(), 4);
    }

    #[test]
    fn test_day10_part23() {
        let mut navigator = parse_input(read_input("test/day10.2.3"), 'F');
        navigator.longest_segment();
        assert_eq!(navigator.enclosed(), 8);
    }

    #[test]
    fn test_day10_part24() {
        let mut navigator = parse_input(read_input("test/day10.2.4"), '7');
        navigator.longest_segment();
        assert_eq!(navigator.enclosed(), 10);
    }
}
