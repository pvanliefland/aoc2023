pub fn run(input: String) {
    // Part 1
    println!("{}", sum_calibration_values(input.clone(), false));

    // Part 2
    println!("{}", sum_calibration_values(input, true));
}

fn sum_calibration_values(input: String, parse_text_digits: bool) -> u32 {
    input
        .lines()
        .map(|line| {
            if parse_text_digits {
                line.replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e")
            } else {
                String::from(line)
            }
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .map(|l| 10 * l.first().unwrap() + l.last().unwrap())
        .sum()
}
#[cfg(test)]
mod tests {
    use super::sum_calibration_values;
    use crate::read_input;

    #[test]
    fn test_day01_part_1() {
        assert_eq!(
            sum_calibration_values(read_input("day01.test.part1"), false),
            142
        );
    }

    #[test]
    fn test_day01_part_2() {
        assert_eq!(
            sum_calibration_values(read_input("day01.test.part2"), true),
            281
        );
    }
}
