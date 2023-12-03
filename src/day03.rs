pub fn run(input: String) {
    let grid = parse_input(input);

    // Part 1
    println!("Part 1: {}", find_part_numbers(grid).iter().sum::<usize>());
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_part_numbers(grid: Vec<Vec<char>>) -> Vec<usize> {
    let (min_x, max_x, min_y, max_y) = (
        0isize,
        grid[0].len() as isize - 1,
        0isize,
        grid.len() as isize - 1,
    );

    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut parts = vec![];
            let mut iter = line.iter().enumerate().peekable();
            while let Some((x, &char)) = iter.next() {
                if char.is_ascii_digit() {
                    let mut coords = vec![(x, y)];
                    let mut part = String::from(char);
                    while iter
                        .peek()
                        .is_some_and(|(_, &next_char)| next_char.is_ascii_digit())
                    {
                        let (next_x, &next_char) = iter.next().expect("🤯");
                        coords.push((next_x, y));
                        part.push(next_char);
                    }
                    if coords.iter().any(|&(x, y)| {
                        [
                            (1, 0),
                            (1, -1),
                            (0, -1),
                            (-1, -1),
                            (-1, 0),
                            (-1, 1),
                            (0, 1),
                            (1, 1),
                        ]
                        .iter()
                        .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
                        .filter_map(|(x, y)| {
                            if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                                Some((x as usize, y as usize))
                            } else {
                                None
                            }
                        })
                        .filter(|adjacent_coords| !coords.contains(adjacent_coords))
                        .any(|adjacent_coords| {
                            let adjacent_char = grid[adjacent_coords.1][adjacent_coords.0];
                            adjacent_char.is_ascii_digit() || adjacent_char != '.'
                        })
                    }) {
                        parts.push(part.parse::<usize>().expect("🙄"));
                    }
                }
            }
            parts
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day03_part_1() {
        let grid = parse_input(read_input("day03.test"));
        assert_eq!(find_part_numbers(grid).iter().sum::<usize>(), 4361);
    }
}
