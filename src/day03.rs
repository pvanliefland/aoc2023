use std::collections::HashSet;

pub fn run(input: String) {
    let grid = parse_input(input);
    let numbers = find_numbers(&grid);

    // Part 1
    println!("Part 1: {}", sum_adjacent_numbers(&grid, &numbers));

    // Part 2
    println!("Part 2: {}", sum_gear_ratios(&grid, &numbers));
}

fn sum_adjacent_numbers(grid: &Grid, numbers: &HashSet<PartNumber>) -> u32 {
    sum_values(
        grid,
        numbers,
        |char| !char.is_ascii_digit() && char != '.',
        None,
        |parts| parts.iter().sum(),
    )
}

fn sum_gear_ratios(grid: &Grid, numbers: &HashSet<PartNumber>) -> u32 {
    sum_values(
        grid,
        numbers,
        |char| char == '*',
        Some(|parts| parts.len() == 2),
        |parts| parts.iter().product(),
    )
}

fn sum_values(
    grid: &Grid,
    numbers: &HashSet<PartNumber>,
    char_filter: fn(char) -> bool,
    parts_filter: Option<fn(&Vec<u32>) -> bool>,
    parts_operation: fn(Vec<u32>) -> u32,
) -> u32 {
    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &char)| char_filter(char))
                .map(move |(x, _)| {
                    numbers
                        .iter()
                        .filter_map(|number| {
                            if number.is_adjacent_to((x as i32, y as i32)) {
                                Some(number.value)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .filter_map(|numbers| {
                    if parts_filter.is_none() || parts_filter.is_some_and(|filter| filter(&numbers))
                    {
                        Some(parts_operation(numbers))
                    } else {
                        None
                    }
                })
        })
        .sum()
}

fn find_numbers(grid: &Grid) -> HashSet<PartNumber> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut numbers_in_line: Vec<PartNumber> = vec![];
            let mut iter = line.iter().enumerate().peekable();
            while let Some((x, &char)) = iter.next() {
                if char.is_ascii_digit() {
                    let mut coords = vec![(x as i32, y as i32)];
                    let mut number_string = String::from(char);
                    while iter
                        .peek()
                        .is_some_and(|(_, &next_char)| next_char.is_ascii_digit())
                    {
                        let (next_x, &next_char) = iter.next().expect("🤯");
                        coords.push((next_x as i32, y as i32));
                        number_string.push(next_char);
                    }
                    numbers_in_line.push(PartNumber::new(number_string, coords));
                }
            }
            numbers_in_line
        })
        .collect()
}

fn parse_input(input: String) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

type Grid = Vec<Vec<char>>;
type Point = (i32, i32);

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct PartNumber {
    value: u32,
    coords: Vec<Point>,
}

impl PartNumber {
    fn new(number_string: String, coords: Vec<Point>) -> Self {
        Self {
            value: number_string.parse().expect("🙄"),
            coords,
        }
    }

    fn is_adjacent_to(&self, other_xy: Point) -> bool {
        self.coords.iter().any(|xy| {
            let (dx, dy) = (xy.0 - other_xy.0, xy.1 - other_xy.1);
            dx.abs() <= 1 && dy.abs() <= 1
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day03_part_1() {
        let grid = parse_input(read_input("day03.test"));
        let numbers = find_numbers(&grid);

        assert_eq!(sum_adjacent_numbers(&grid, &numbers), 4361);
    }

    #[test]
    fn test_day03_part_2() {
        let grid = parse_input(read_input("day03.test"));
        let numbers = find_numbers(&grid);

        assert_eq!(sum_gear_ratios(&grid, &numbers), 467835);
    }
}
