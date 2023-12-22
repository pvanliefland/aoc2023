pub fn run(input: String) {
    let coords = parse_input(&input);
    println!("Part 1: {}", sum_of_distances(&coords, 2));

    println!("Part 1: {}", sum_of_distances(&coords, 1000000));
}

fn sum_of_distances(coords: &[Position], expand_factor: usize) -> usize {
    let mut expanded_coords = coords.to_vec();
    let max_x = expanded_coords.iter().map(|c| c.0).max().expect("🙄");
    let max_y = expanded_coords.iter().map(|c| c.1).max().expect("🙄");

    let empty_cols = (0..=max_x)
        .filter(|&x| !expanded_coords.iter().any(|c| c.0 == x))
        .collect::<Vec<_>>();
    let empty_rows = (0..=max_y)
        .filter(|&y| !expanded_coords.iter().any(|c| c.1 == y))
        .collect::<Vec<_>>();

    for (i, x) in empty_cols.iter().enumerate() {
        expanded_coords
            .iter_mut()
            .filter(|c| c.0 > x + i * (expand_factor - 1))
            .for_each(|c| {
                c.0 += expand_factor - 1;
            });
    }
    for (i, y) in empty_rows.iter().enumerate() {
        expanded_coords
            .iter_mut()
            .filter(|c| c.1 > y + i * (expand_factor - 1))
            .for_each(|c| {
                c.1 += expand_factor - 1;
            });
    }

    let mut pairs = vec![];
    for i in 0..expanded_coords.len() {
        let c1 = vec![expanded_coords[i]; expanded_coords.len()];
        let c2s = expanded_coords[(i + 1)..expanded_coords.len()].to_vec();
        let mut zipped = c1.iter().copied().zip(c2s).collect::<Vec<_>>();
        pairs.append(&mut zipped);
    }
    pairs
        .into_iter()
        .map(|(c1, c2)| c1.0.abs_diff(c2.0) + c1.1.abs_diff(c2.1))
        .sum()
}

fn parse_input(input: &str) -> Vec<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c != '.')
                .map(|(x, _)| (x, y))
                .collect::<Vec<_>>()
        })
        .collect()
}

type Position = (usize, usize);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day11_part1() {
        assert_eq!(
            sum_of_distances(&parse_input(&read_input("test/day11")), 2),
            374
        );
    }

    #[test]
    fn test_day11_part2() {
        assert_eq!(
            sum_of_distances(&parse_input(&read_input("test/day11")), 10),
            1030
        );

        assert_eq!(
            sum_of_distances(&parse_input(&read_input("test/day11")), 100),
            8410
        );
    }
}
