mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader};

mod tree;
use tree::GeneralTree;

mod command;
use command::Command;

const MAX_DIRECTORY_SIZE: u32 = 100_000;
const TOTAL_DISK_SPACE: u32 = 70_000_000;
const TARGET_FREE_SPACE: u32 = 30_000_000;

pub fn solve(input_file: File) {
    let input = parse_input(&input_file);

    let solution_1 = exercise_1::solve(&input);
    println!("Exercise 1: {solution_1}");

    let solution_2 = exercise_2::solve(&input);
    println!("Exercise 2: {solution_2}");
}

fn parse_input(input_file: &File) -> GeneralTree {
    let mut current_directory = String::new();

    BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .fold(GeneralTree::new(), |mut tree, line| {
            if line.starts_with("$") {
                let command = Command::from(&line);
                current_directory = command.execute(&current_directory);
            } else {
                let words: Vec<&str> = line.split(" ").collect();
                let (child_name, value) = match words.as_slice() {
                    ["dir", directory_name] => (directory_name, None),
                    [size, file_name] => (
                        file_name,
                        Some(size.parse().expect("File size was not a number!")),
                    ),
                    _ => panic!("Input has wrong number of arguments!"),
                };
                tree.insert(&format!("{current_directory}{child_name}"), value);
            }

            tree
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() {}

    #[test]
    fn exercise_1_works() {
        let test_file = File::open("tests/day_07.txt").expect("Test file not found!");
        let input = parse_input(&test_file);

        let solution = exercise_1::solve(&input);

        assert_eq!(solution, 95_437);
    }

    #[test]
    fn exercise_2_works() {
        let test_file = File::open("tests/day_07.txt").expect("Test file not found!");
        let input = parse_input(&test_file);

        let solution = exercise_1::solve(&input);

        assert_eq!(solution, 24_933_642);
    }
}
