mod day_01;
mod day_02;

use std::fs::File;

pub fn solve_day(day: u32) {
    let input_file =
        File::open(format!("inputs/day_{:02}.txt", day)).expect("Input file not found");

    match day {
        1 => day_01::solve(input_file),
        2 => day_02::solve(input_file),
        26..=u32::MAX => panic!("Advent of Code does not have that many days!"),
        _ => panic!("No solution found for that day yet!"),
    }
}
