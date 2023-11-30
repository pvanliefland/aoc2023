use std::env;
use std::fs::read_to_string;
use std::time::{Duration, Instant};

mod day01;

fn main() {
    let days = [day01::run];

    let range = if let Some(day) = env::args().nth(1) {
        let day = day.parse().expect("Invalid day provided");
        day - 1..day
    } else {
        0..days.len()
    };
    let queue = range.map(|i| (i + 1, days[i]));

    for (day, day_func) in queue {
        println!("=========== Day {day} ===========");
        for sample in [true, false] {
            let now = Instant::now();
            let file_name = format!(
                "inputs/day{:0>2}{}.txt",
                day,
                if sample { ".sample" } else { "" }
            );
            let input = read_to_string(file_name.clone())
                .unwrap_or_else(|_| panic!("File {file_name} not found"))
                .trim()
                .to_string();

            day_func(&input, sample);

            let duration = now.elapsed();
            let mode = if sample { "sample mode" } else { "full mode" };
            if duration > Duration::from_secs(2) {
                println!(
                    "Finished day {day} ({mode}) in {} seconds",
                    duration.as_secs()
                );
            } else {
                println!("Finished day {day} ({mode}) in {} ms", duration.as_millis());
            }
        }
        println!();
    }
}
