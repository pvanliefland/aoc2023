use std::collections::HashMap;
use std::ops::Range;

pub fn run(input: String) {
    let (seeds, maps) = parse_input(input);

    println!("Part 1: {}", lowest_location(&seeds, &maps, false));
    println!("Part 2: {}", lowest_location(&seeds, &maps, true));
}

fn lowest_location(seeds: &[isize], maps: &[Vec<Map>], range_mode: bool) -> isize {
    let seed_ranges = if range_mode {
        seeds
            .chunks(2)
            .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
            .collect::<Vec<_>>()
    } else {
        seeds.iter().map(|&seed| seed..seed + 1).collect()
    };

    let mut computed_maps: HashMap<_, _> =
        seed_ranges.iter().map(|range| (range.clone(), 0)).collect();
    for (_, level) in maps.iter().enumerate() {
        for (r1, d1) in computed_maps.clone() {
            if let Some((r2, d2)) = level.iter().find(|(range, _)| {
                range.contains(&(r1.start + d1)) || range.contains(&(r1.end + d1 - 1))
            }) {
                match (
                    r2.contains(&(r1.start + d1)),
                    r2.contains(&(r1.end + d1 - 1)),
                ) {
                    (true, true) => {
                        *computed_maps.get_mut(&r1).expect("😅") += d2;
                    }
                    (true, false) => {
                        computed_maps.remove(&r1).expect("☢️");
                        computed_maps.insert(r1.start..(r2.end - d1), d1 + d2);
                        computed_maps.insert((r2.end - d1)..r1.end, d1);
                    }
                    (false, true) => {
                        computed_maps.remove(&r1).expect("☢️");
                        computed_maps.insert(r1.start..(r2.start - d1), d1);
                        computed_maps.insert((r2.start - d1)..r1.end, d1 + d2);
                    }
                    _ => panic!("😭"),
                }
            }
        }
    }

    computed_maps
        .iter()
        .map(|(range, delta)| range.start + delta)
        .min()
        .expect("☢️")
}

fn parse_input(input: String) -> (Vec<isize>, Vec<Vec<Map>>) {
    let (seed_part, map_part) = input.split_once("\n\n").expect("🙄");
    let seeds: Vec<isize> = seed_part[7..]
        .split_whitespace()
        .map(|seed| seed.parse().expect("😅"))
        .collect();
    let maps: Vec<Vec<_>> = map_part
        .split("\n\n")
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|range_data| {
                    let range_parts: Vec<isize> = range_data
                        .split_whitespace()
                        .map(|part| part.parse().expect("🤯"))
                        .collect();
                    (
                        range_parts[1]..(range_parts[1] + range_parts[2]),
                        range_parts[0] - range_parts[1],
                    )
                })
                .collect()
        })
        .collect();
    (seeds, maps)
}

type Map = (Range<isize>, isize);

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
