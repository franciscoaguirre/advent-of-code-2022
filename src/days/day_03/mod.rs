mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

mod rucksack;
use rucksack::Rucksack;

mod group;
use group::Group;

pub fn solve(mut input_file: File) {
    let input = parse_input(&input_file);
    let solution_1 = exercise_1::solve(&input);
    println!("Exercise 1: {solution_1}");

    input_file.rewind().expect("Couldn't rewind input file!");

    let input_grouped = parse_input_grouped(&input_file);
    let solution_2 = exercise_2::solve(&input_grouped);
    println!("Exercise 2: {solution_2}");
}

fn parse_input(input_file: &File) -> Vec<Rucksack> {
    BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Rucksack::from(&line, 2))
        .collect()
}

fn parse_input_grouped(input_file: &File) -> Vec<Group> {
    BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Rucksack::from(&line, 2))
        .collect::<Vec<Rucksack>>()
        .chunks(3)
        .map(|chunk| Group::new(chunk))
        .collect()
}
