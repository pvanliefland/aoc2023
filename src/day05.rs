use std::collections::HashMap;
use std::ops::Range;

pub fn run(input: String) {
    let (seeds, maps) = parse_input(input);

    println!("Part 1: {}", lowest_location(&seeds, &maps, false));
    println!("Part 2: {}", lowest_location(&seeds, &maps, true));
}

fn lowest_location(
    seeds: &[isize],
    maps: &Vec<Vec<(Range<isize>, isize)>>,
    range_mode: bool,
) -> isize {
    let seed_ranges = if range_mode {
        seeds
            .chunks(2)
            .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
            .collect::<Vec<_>>()
    } else {
        seeds.iter().map(|&seed| seed..seed + 1).collect()
    };

    // dbg!(maps);

    let mut computed_maps: HashMap<_, _> =
        seed_ranges.iter().map(|range| (range.clone(), 0)).collect();
    for (_, level) in maps.iter().enumerate() {
        /*println!();
        match i {
            0 => println!("seed-to-soil"),
            1 => println!("soil-to-fertilizer"),
            2 => println!("fertilizer-to-water"),
            3 => println!("water-to-light"),
            4 => println!("light-to-temperature"),
            5 => println!("temperature-to-humidity"),
            6 => println!("humidity-to-location"),
            _ => panic!("🤷"),
        }*/

        for (range, delta) in computed_maps.clone() {
            let mapped_range = (range.start + delta)..range.end + delta;
            if let Some((new_range, new_delta)) = level.iter().find(|(new_range, _)| {
                new_range.contains(&mapped_range.start)
                    || new_range.contains(&(mapped_range.end - 1))
            }) {
                match (
                    new_range.contains(&mapped_range.start),
                    new_range.contains(&(mapped_range.end - 1)),
                ) {
                    (true, true) => {
                        /*println!(
                            "Range {:?}, mapped to {:?}, is fully contained within {:?}",
                            range, mapped_range, new_range
                        );*/
                        *computed_maps.get_mut(&range).expect("😅") += new_delta;
                    }
                    (true, false) => {
                        /*println!(
                            "The start of range {:?}, mapped to {:?}, is contained within {:?}, but not the end",
                            range, mapped_range, new_range
                        );*/
                        let r1 = range.start..(new_range.end - delta);
                        let r2 = (new_range.end - delta)..range.end;
                        /*println!(
                            "Splitting original range {:?} into new range: {:?} and {:?}",
                            range, r1, r2
                        );*/

                        computed_maps.remove(&range).expect("☢️");
                        computed_maps.insert(r1, delta + new_delta);
                        computed_maps.insert(r2, delta);
                    }
                    (false, true) => {
                        /*println!(
                            "The end of range {:?} mapped to {:?}, is contained within {:?}, but not the start",
                            range, mapped_range, new_range
                        );*/
                        let r1 = range.start..(new_range.start - delta);
                        let r2 = (new_range.start - delta)..range.end;
                        /*println!(
                            "Splitting original range {:?} to new range: {:?} and {:?}",
                            range, r1, r2
                        );*/

                        computed_maps.remove(&range).expect("☢️");
                        computed_maps.insert(r1, delta);
                        computed_maps.insert(r2, delta + new_delta);
                    }
                    _ => panic!("😭"),
                }
            } else {
                /*println!(
                    "No mapping range for {:?}, mapped to {:?}, will be mapped as is",
                    range, mapped_range
                );*/
            };
        }
    }

    let mut best = computed_maps
        .iter()
        .map(|(range, delta)| (range, delta, range.start + delta))
        .collect::<Vec<_>>();
    best.sort_by_key(|&(_, _, score)| score);
    best.first().expect("🙄").2
}

fn parse_input(input: String) -> (Vec<isize>, Vec<Vec<(Range<isize>, isize)>>) {
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
