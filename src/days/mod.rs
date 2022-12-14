mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;

use std::fs::File;

pub fn solve_day(day: u32) {
    let input_file =
        File::open(format!("inputs/day_{:02}.txt", day)).expect("Input file not found");

    match day {
        1 => day_01::solve(input_file),
        2 => day_02::solve(input_file),
        3 => day_03::solve(input_file),
        4 => day_04::solve(input_file),
        5 => day_05::solve(input_file),
        6 => day_06::solve(input_file),
        7 => day_07::solve(input_file),
        8 => day_08::solve(input_file),
        9 => day_09::solve(input_file),
        10 => day_10::solve(input_file),
        11 => day_11::solve(input_file),
        12 => day_12::solve(input_file),
        13 => day_13::solve(input_file),
        26.. => panic!("Advent of Code does not have that many days!"),
        _ => panic!("No solution found for that day yet!"),
    }
}
