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
    let mut adjacent_numbers: HashSet<PartNumber> = HashSet::new();
    grid.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, &char)| {
            if !char.is_ascii_digit() && char != '.' {
                numbers
                    .iter()
                    .filter(|number| number.is_adjacent_to((x as i32, y as i32)))
                    .for_each(|number| {
                        adjacent_numbers.insert(number.clone());
                    });
            }
        });
    });
    adjacent_numbers
        .iter()
        .map(|part_number| part_number.number)
        .sum()
}

fn sum_gear_ratios(grid: &Grid, numbers: &HashSet<PartNumber>) -> u32 {
    let mut sum = 0;
    grid.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, &char)| {
            if char == '*' {
                let mut adjacent_numbers = vec![];
                numbers
                    .iter()
                    .filter(|number| number.is_adjacent_to((x as i32, y as i32)))
                    .for_each(|part_number| {
                        adjacent_numbers.push(part_number.number);
                    });
                if adjacent_numbers.len() == 2 {
                    sum += adjacent_numbers.iter().product::<u32>();
                }
            }
        });
    });

    sum
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
    number: u32,
    coords: Vec<Point>,
}

impl PartNumber {
    fn new(number_string: String, coords: Vec<Point>) -> Self {
        Self {
            number: number_string.parse().expect("🙄"),
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
