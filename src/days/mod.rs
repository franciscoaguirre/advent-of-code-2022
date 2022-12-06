mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

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
        26..=u32::MAX => panic!("Advent of Code does not have that many days!"),
        _ => panic!("No solution found for that day yet!"),
    }
}
