mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader};

mod elf;
use elf::Elf;

pub fn solve(input_file: File) {
    let mut input = parse_input(input_file);

    let solution_1 = exercise_1::solve(&input);
    println!("Exercise 1: {solution_1}");

    let solution_2 = exercise_2::solve(&mut input);
    println!("Exercise 2: {solution_2}");
}

fn parse_input(input_file: File) -> Vec<Elf> {
    BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .fold(vec![Elf::default()], |mut elves, current_line| {
            if current_line == "" {
                elves.push(Elf::default());
                return elves;
            }

            let calories: u32 = current_line.parse().unwrap();

            let current_elf = elves.last_mut().unwrap();
            current_elf.add_food_item(calories);

            elves
        })
}
