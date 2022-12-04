mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader};

mod pair;
use pair::Pair;

pub fn solve(input_file: File) {
    let input = parse_input(&input_file);
    let solution_1 = exercise_1::solve(&input);
    println!("Exercise 1: {solution_1}");

    let solution_2 = exercise_2::solve(&input);
    println!("Exercise 2: {solution_2}");
}

fn parse_input(input_file: &File) -> Vec<Pair> {
    BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Pair::from(&line))
        .collect()
}
