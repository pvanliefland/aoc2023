use std::ops::Range;

pub fn run(input: String) {
    let (seeds, maps) = parse_input(input);

    println!("Part 1: {}", lowest_location(&seeds, &maps, false));
    println!("Part 2: {}", lowest_location(&seeds, &maps, true));
}
fn parse_input(input: String) -> (Vec<Seed>, Maps) {
    let (seed_part, map_part) = input.split_once("\n\n").expect("🙄");
    let seeds: Vec<usize> = seed_part[7..]
        .split_whitespace()
        .map(|seed| seed.parse().expect("😅"))
        .collect();
    let maps: Vec<Vec<_>> = map_part
        .split("\n\n")
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|range_data| {
                    let range_parts: Vec<usize> = range_data
                        .split_whitespace()
                        .map(|part| part.parse().expect("🤯"))
                        .collect();
                    (
                        range_parts[1]..(range_parts[1] + range_parts[2]),
                        range_parts[0]..(range_parts[0] + range_parts[2]),
                    )
                })
                .collect()
        })
        .collect();
    (seeds, maps.try_into().expect("😿"))
}
fn lowest_location(seeds: &[Seed], maps: &Maps, range_mode: bool) -> usize {
    let seed_ranges = if range_mode {
        seeds
            .chunks(2)
            .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
            .collect::<Vec<_>>()
    } else {
        seeds.iter().map(|&seed| seed..seed + 1).collect()
    };

    seed_ranges
        .iter()
        .flat_map(|seed_range| seed_range.clone().collect::<Vec<_>>())
        .map(|seed| {
            maps.iter().fold(seed, |value, map| {
                if let Some(ranges) = map.iter().find(|ranges| ranges.0.contains(&value)) {
                    ranges.1.start + (value - ranges.0.start)
                } else {
                    value
                }
            })
        })
        .min()
        .expect("🙄")
}

type Seed = usize;
type Map = Vec<(Range<usize>, Range<usize>)>;
type Maps = [Map; 7];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day05_part_1() {
        let (seeds, maps) = parse_input(read_input("test/day05"));
        assert_eq!(lowest_location(&seeds, &maps, false), 35);
    }

    #[test]
    fn test_day05_part_2() {
        let (seeds, maps) = parse_input(read_input("test/day05"));
        assert_eq!(lowest_location(&seeds, &maps, true), 46);
    }
}
