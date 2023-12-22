use std::env;
use std::time::{Duration, Instant};

use aoc2023::read_input;

fn main() {
    let days = [
        aoc2023::day01::run,
        aoc2023::day02::run,
        aoc2023::day03::run,
        aoc2023::day04::run,
        aoc2023::day05::run,
        aoc2023::day06::run,
        aoc2023::day07::run,
        aoc2023::day08::run,
        aoc2023::day09::run,
        aoc2023::day10::run,
        aoc2023::day11::run,
    ];

    let range = if let Some(day) = env::args().nth(1) {
        let day = day.parse().expect("Invalid day provided");
        day - 1..day
    } else {
        0..days.len()
    };
    let queue = range.map(|i| (i + 1, days[i]));

    for (day, day_func) in queue {
        println!("================ Day {day} ================");
        let now = Instant::now();

        println!("Outputs:");
        day_func(read_input(&format!("day{:0>2}", day)));

        let duration = now.elapsed();
        if duration > Duration::from_secs(2) {
            println!("Execution time: {}s", duration.as_secs());
        } else {
            println!("Execution time: {}ms", duration.as_millis());
        }
        println!();
    }
}
