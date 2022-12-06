mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader};

const WINDOW_SIZE_FOR_PACKET: usize = 4;
const WINDOW_SIZE_FOR_MESSAGE: usize = 14;

pub fn solve(input_file: File) {
    let input = parse_input(&input_file);

    let solution_1 = exercise_1::solve(&input[..]);
    println!("Exercise 1: {solution_1}");

    let solution_2 = exercise_2::solve(&input[..]);
    println!("Exercise 2: {solution_2}");
}

fn parse_input(input_file: &File) -> Vec<char> {
    let mut result = String::new();

    BufReader::new(input_file)
        .read_line(&mut result)
        .expect("Failed to read input file");

    result.chars().collect()
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::{exercise_1, exercise_2, parse_input};

    #[test]
    fn exercise_1_works() {
        let test_file = File::open("tests/day_06.txt").expect("Test file not found!");
        let input = parse_input(&test_file);

        let solution = exercise_1::solve(&input);
        assert_eq!(solution, 7);
    }

    #[test]
    fn exercise_2_works() {
        let test_file = File::open("tests/day_06.txt").expect("Test file not found!");
        let input = parse_input(&test_file);

        let solution = exercise_2::solve(&input);
        assert_eq!(solution, 19);
    }
}
