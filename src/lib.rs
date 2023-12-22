use std::fs::read_to_string;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

pub fn read_input(name: &str) -> String {
    let file_name = format!("inputs/{name}.txt",);
    read_to_string(file_name.clone())
        .unwrap_or_else(|_| panic!("File {file_name} not found"))
        .trim()
        .to_string()
}
