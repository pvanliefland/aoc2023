use std::fs::read_to_string;

pub mod day01;

pub fn read_input(day: usize, test: bool) -> String {
    let file_name = format!(
        "inputs/day{:0>2}{}.txt",
        day,
        if test { ".test" } else { "" }
    );
    read_to_string(file_name.clone())
        .unwrap_or_else(|_| panic!("File {file_name} not found"))
        .trim()
        .to_string()
}
