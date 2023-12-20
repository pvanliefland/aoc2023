use std::collections::HashMap;

pub fn run(input: String) {
    let map = build_map(parse_input(input));

    println!("Part 1: {}", map.loop_length());
    println!("Part 2: {}", map.count_enclosed());
}

fn build_map(map_data: HashMap<(isize, isize), char>) -> Map {
    let mut tiles = map_data
        .iter()
        .map(|(p, t)| {
            (
                *p,
                Tile {
                    kind: *t,
                    position_in_loop: if *t == 'S' { Some(0) } else { None },
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
            let possible_moves = if current_position == start {
                let moves = [
                    ((0, 1), ['|', 'J', 'L']),
                    ((1, 0), ['-', 'J', '7']),
                    ((0, -1), ['|', '7', 'F']),
                    ((-1, 0), ['-', 'L', 'F']),
                ]
                .into_iter()
                .filter_map(|(m, kinds)| {
                    tiles
                        .get(&(current_position.0 + m.0, current_position.1 + m.1))
                        .and_then(|tile| kinds.contains(&tile.kind).then_some(m))
                })
                .collect::<Vec<_>>();
                if steps == 1 && i == 1 {
                    [moves[1], moves[0]]
                } else {
                    [moves[0], moves[1]]
                }
            } else {
                match tiles.get(&current_position).expect("😅").kind {
                    '|' => [(0, 1), (0, -1)],
                    '-' => [(1, 0), (-1, 0)],
                    'L' => [(0, -1), (1, 0)],
                    'J' => [(0, -1), (-1, 0)],
                    '7' => [(0, 1), (-1, 0)],
                    'F' => [(0, 1), (1, 0)],
                    o => panic!("{} 😰", o),
                }
            };

            let next_position = *possible_moves
                .map(|(dx, dy)| (current_position.0 + dx, current_position.1 + dy))
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

struct Tile {
    kind: char,
    position_in_loop: Option<usize>,
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

    fn loop_length(&self) -> usize {
        self.map
            .values()
            .filter_map(|tile| tile.position_in_loop)
            .max()
            .expect("😅")
    }

    fn count_enclosed(&self) -> usize {
        self.map
            .iter()
            .filter(|(p, t)| {
                if t.position_in_loop.is_some() {
                    false
                } else {
                    let mut changes = 0;
                    let mut current_boundary = None;
                    for x in (p.0 + 1)..=self.max_xy.0 {
                        let tile = self.map.get(&(x, p.1)).expect("🙄");
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
                }
            })
            .count()
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
            build_map(parse_input(read_input("test/day10.1"))).loop_length(),
            8
        );
    }

    #[test]
    fn test_day10_part21() {
        assert_eq!(
            build_map(parse_input(read_input("test/day10.2.1"))).count_enclosed(),
            4
        );
    }

    #[test]
    fn test_day10_part22() {
        assert_eq!(
            build_map(parse_input(read_input("test/day10.2.2"))).count_enclosed(),
            4
        );
    }

    #[test]
    fn test_day10_part23() {
        assert_eq!(
            build_map(parse_input(read_input("test/day10.2.3"))).count_enclosed(),
            8
        );
    }

    #[test]
    fn test_day10_part24() {
        assert_eq!(
            build_map(parse_input(read_input("test/day10.2.4"))).count_enclosed(),
            10
        );
    }
}
