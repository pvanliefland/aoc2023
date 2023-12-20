use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub fn run(input: String) {
    let map = build_map(parse_input(input), '-');

    println!("Part 1: {}", map.max_by(|tile| tile.position_in_loop));
    println!("Part 2: {}", find_enclosed(map).count_where(|t| t.enclosed));
}

fn build_map(map_data: HashMap<(isize, isize), char>, start_tile: char) -> Map {
    let mut tiles = map_data
        .iter()
        .map(|(p, t)| {
            (
                *p,
                Tile {
                    kind: *t,
                    position_in_loop: if *t == 'S' { Some(0) } else { None },
                    enclosed: false,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let start = tiles
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
                tiles.get(&current_position).expect("😅").kind
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
            tiles.get_mut(&next_position).expect("😭").position_in_loop = Some(steps);
        });
        if current_positions[0] == current_positions[1] {
            break;
        }
    }
    Map::new(tiles)
}

fn find_enclosed(map: Map) -> Map {
    let smarter_map = map
        .iter()
        .map(|(p, t)| {
            let enclosed = if t.position_in_loop.is_some() {
                false
            } else {
                let mut changes = 0;
                let mut current_boundary = None;
                for x in (p.0 + 1)..=map.max_xy.0 {
                    let tile = map.map.get(&(x, p.1)).expect("🙄");
                    if tile.position_in_loop.is_some() {
                        match (current_boundary, tile.kind) {
                            (_, '|') => {
                                changes += 1;
                                current_boundary = None;
                            }
                            (None, 'L' | 'F') => {
                                changes += 1;
                                current_boundary = Some(tile.kind);
                            }
                            (Some('F'), '7') | (Some('L'), 'J') => {
                                changes += 1;
                                current_boundary = None;
                            }
                            (Some('F'), 'J') | (Some('L'), '7') => {
                                current_boundary = None;
                            }
                            _ => {}
                        }
                    }
                }

                changes % 2 == 1
            };
            (
                *p,
                Tile {
                    kind: t.kind,
                    position_in_loop: t.position_in_loop,
                    enclosed,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    Map::new(smarter_map)
}
#[derive(Debug)]
struct Tile {
    kind: char,
    position_in_loop: Option<usize>,
    enclosed: bool,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.position_in_loop.is_some() {
            write!(f, "\x1b[93m{}\x1b[0m", self.kind)
        } else if self.enclosed {
            write!(f, "\x1b[97m{}\x1b[0m", self.position_in_loop.unwrap())
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

struct Map {
    map: HashMap<(isize, isize), Tile>,
    max_xy: (isize, isize),
}

impl Map {
    pub fn new(map: HashMap<(isize, isize), Tile>) -> Self {
        let max_xy = *(map.keys().max().expect("😅"));
        Self { map, max_xy }
    }

    fn max_by(&self, by_fn: fn(&Tile) -> Option<usize>) -> usize {
        self.map.values().filter_map(by_fn).max().expect("😅")
    }

    fn count_where(&self, where_fn: fn(&&Tile) -> bool) -> usize {
        self.map.values().filter(where_fn).count()
    }

    fn iter(&self) -> Iter<'_, (isize, isize), Tile> {
        self.map.iter()
    }
}

impl Display for Map {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day10_part1() {
        assert_eq!(
            build_map(parse_input(read_input("test/day10.1")), 'F')
                .max_by(|tile| tile.position_in_loop),
            8
        );
    }

    #[test]
    fn test_day10_part21() {
        assert_eq!(
            find_enclosed(build_map(parse_input(read_input("test/day10.2.1")), 'F'))
                .count_where(|t| t.enclosed),
            4
        );
    }

    #[test]
    fn test_day10_part22() {
        assert_eq!(
            find_enclosed(build_map(parse_input(read_input("test/day10.2.2")), 'F'))
                .count_where(|t| t.enclosed),
            4
        );
    }

    #[test]
    fn test_day10_part23() {
        assert_eq!(
            find_enclosed(build_map(parse_input(read_input("test/day10.2.3")), 'F'))
                .count_where(|t| t.enclosed),
            8
        );
    }

    #[test]
    fn test_day10_part24() {
        assert_eq!(
            find_enclosed(build_map(parse_input(read_input("test/day10.2.4")), '7'))
                .count_where(|t| t.enclosed),
            10
        );
    }
}
