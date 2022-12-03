mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

mod round;
use round::Round;

pub fn solve(mut input_file: File) {
    let input = parse_input(&input_file);

    let solution_1 = exercise_1::solve(&input);
    println!("Exercise 1: {solution_1}");

    input_file.rewind().expect("Couldn't rewind input file");

    let input_alternate = parse_input_alternate(&input_file);

    let solution_2 = exercise_2::solve(&input_alternate);
    println!("Exercise 2: {solution_2}");
}

fn parse_input(input_file: &File) -> Vec<Round> {
    BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Round::from(&line))
        .collect()
}

/// Parse the input with the alternate parsing method
fn parse_input_alternate(input_file: &File) -> Vec<Round> {
    BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Round::from_alternate(&line))
        .collect()
}
