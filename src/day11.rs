pub fn run(input: String) {
    let mut coords = parse_input(&input);
    expand_space(&mut coords);
    let distances = distances(&coords);

    println!(
        "Part 1: {}",
        distances.iter().map(|(_, _, d)| d).sum::<usize>()
    );
    println!("Part 2: {}", 42);
}

fn distances(coords: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize), usize)> {
    let mut pairs = vec![];
    for i in 0..coords.len() {
        let c1 = vec![coords[i]; coords.len()];
        let c2s = coords[(i + 1)..coords.len()].to_vec();
        let mut zipped = c1.iter().copied().zip(c2s).collect::<Vec<_>>();
        pairs.append(&mut zipped);
    }
    pairs
        .into_iter()
        .map(|(c1, c2)| {
            let distance = c1.0.abs_diff(c2.0) + c1.1.abs_diff(c2.1);
            (c1, c2, distance)
        })
        .collect::<Vec<_>>()
}

fn expand_space(coords: &mut [(usize, usize)]) {
    let max_x = coords.iter().map(|c| c.0).max().expect("🙄");
    let max_y = coords.iter().map(|c| c.1).max().expect("🙄");

    let empty_cols = (0..=max_x)
        .filter(|&x| !coords.iter().any(|c| c.0 == x))
        .collect::<Vec<_>>();
    let empty_rows = (0..=max_y)
        .filter(|&y| !coords.iter().any(|c| c.1 == y))
        .collect::<Vec<_>>();

    for (i, x) in empty_cols.iter().enumerate() {
        coords.iter_mut().filter(|c| c.0 > x + i).for_each(|c| {
            c.0 += 1;
        });
    }
    for (i, y) in empty_rows.iter().enumerate() {
        coords.iter_mut().filter(|c| c.1 > y + i).for_each(|c| {
            c.1 += 1;
        });
    }
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_day11_part1() {
        let mut coords = parse_input(&read_input("test/day11"));
        expand_space(&mut coords);
        let distances = distances(&coords);
        assert_eq!(distances.iter().map(|(_, _, d)| d).sum::<usize>(), 374);
    }

    #[test]
    fn test_day11_part2() {}
}
