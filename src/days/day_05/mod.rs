mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader};

mod instruction;
use instruction::Instruction;

mod crates;
use crates::Crates;

pub fn solve(input_file: File) {
    let (mut crates, instructions) = parse_input(&input_file);

    // Need to clone it for the second exercise, they both mutate it
    let mut cloned_crates = crates.clone();

    let solution_1 = exercise_1::solve(&mut crates, &instructions);
    println!("Exercise 1: {solution_1}");

    let solution_2 = exercise_2::solve(&mut cloned_crates, &instructions);
    println!("Exercise 2: {solution_2}");
}

#[cfg(not(test))]
mod consts {
    pub const STACKS: usize = 9;
    pub const MAX_STACK_HEIGHT: usize = 8;
    pub const STACK_WIDTH: usize = 3;
}

#[cfg(test)]
mod consts {
    pub const STACKS: usize = 3;
    pub const MAX_STACK_HEIGHT: usize = 3;
    pub const STACK_WIDTH: usize = 3;
}

use consts::*;

fn parse_input(input_file: &File) -> (Crates<STACKS>, Vec<Instruction>) {
    let mut result = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .enumerate()
        .fold(
            (Crates::new(), Vec::new()),
            |(mut crates, mut instructions), (index, line)| {
                if index < MAX_STACK_HEIGHT {
                    // +1 to get each crate with its following space character
                    let crate_row = line
                        .chars()
                        .collect::<Vec<char>>()
                        .chunks(STACK_WIDTH + 1)
                        .map(|chunk| chunk.iter().collect::<String>())
                        .collect::<Vec<String>>();

                    for (index, crate_spot) in crate_row.iter().enumerate() {
                        if crate_spot.contains("[") {
                            crates.add_crate(
                                index,
                                crate_spot.chars().nth(1).expect("No char in crate!"),
                            );
                        }
                    }
                } else if index < (MAX_STACK_HEIGHT + 2) as usize {
                    // Nothing
                } else {
                    instructions.push(Instruction::from(&line));
                }

                (crates, instructions)
            },
        );

    result.0.invert_order();

    result
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::{exercise_1, exercise_2, parse_input, Instruction};

    #[test]
    fn parsing_works() {
        let test_file = File::open("tests/day_05.txt").expect("Test file not found!");
        let (crates, instructions) = parse_input(&test_file);

        assert_eq!(
            instructions,
            vec![
                Instruction::new(1, 2, 1),
                Instruction::new(3, 1, 3),
                Instruction::new(2, 2, 1),
                Instruction::new(1, 1, 2)
            ]
        );

        assert_eq!(
            crates.stacks(),
            &[vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        );
    }

    #[test]
    fn exercise_1_works() {
        let test_file = File::open("tests/day_05.txt").expect("Test file not found!");
        let (mut crates, instructions) = parse_input(&test_file);

        let solution = exercise_1::solve(&mut crates, &instructions);

        assert_eq!(
            crates.stacks(),
            &[vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']]
        );

        assert_eq!(solution, String::from("CMZ"));
    }

    #[test]
    fn exercise_2_works() {
        let test_file = File::open("tests/day_05.txt").expect("Test file not found!");
        let (mut crates, instructions) = parse_input(&test_file);

        let solution = exercise_2::solve(&mut crates, &instructions);

        assert_eq!(
            crates.stacks(),
            &[vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']]
        );

        assert_eq!(solution, String::from("MCD"));
    }
}
