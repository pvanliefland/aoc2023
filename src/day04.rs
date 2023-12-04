pub fn run(input: String) {
    println!("Part 1: {}", total_card_points(input));
    println!("Part 2: {}", "?");
}

fn total_card_points(input: String) -> i32 {
    input
        .replace("  ", " ")
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(':').expect("🤪");
            let numbers: Vec<Vec<i32>> = numbers
                .trim()
                .split(" | ")
                .map(|numbers| {
                    numbers
                        .split(' ')
                        .map(|number| number.parse().expect("🤬"))
                        .collect()
                })
                .collect();
            numbers[1].iter().fold(0, |acc, e| {
                if numbers[0].contains(e) {
                    match acc {
                        0 => 1,
                        other => 2 * other,
                    }
                } else {
                    acc
                }
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day04_part_1() {
        assert_eq!(total_card_points(read_input("day04.test")), 13);
    }

    #[test]
    fn test_day04_part_2() {}
}
