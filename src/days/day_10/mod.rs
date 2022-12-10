mod cpu;
mod exercise_1;
mod exercise_2;

use cpu::{Instruction, CPU};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(input_file: File) {
    let cpu = parse_input(&input_file);

    let solution_1 = exercise_1::solve(&cpu);
    println!("Exercise 1: {solution_1}");

    // Graphical solution
    exercise_2::solve(&cpu);
}

fn parse_input(input_file: &File) -> CPU {
    BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .fold(CPU::new(), |mut cpu, line| {
            let instruction = Instruction::from(&line);
            cpu.execute(&instruction);
            cpu
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_1_works() {
        let test_file = File::open("tests/day_10.txt").expect("Couldn't find test file!");
        let cpu = parse_input(&test_file);
        let solution = exercise_1::solve(&cpu);
        assert_eq!(solution, 13_140);
    }
}
