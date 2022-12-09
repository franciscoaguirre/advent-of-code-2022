mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

mod rope;
use rope::{Instruction, Rope};

mod vec2;

pub fn solve(mut input_file: File) {
    let short_rope = parse_input(&input_file, 0);

    let solution_1 = exercise_1::solve(&short_rope);
    println!("Exercise 1: {solution_1}");

    input_file.rewind().unwrap();

    let long_rope = parse_input(&input_file, 8); // 8 = 10 total knots - head - tail

    let solution_2 = exercise_2::solve(&long_rope);
    println!("Exercise 2: {solution_2}");
}

fn parse_input(input_file: &File, additional_knots: usize) -> Rope {
    BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Instruction::from(&line))
        .fold(Rope::new(additional_knots), |mut rope, instruction| {
            rope.move_head(&instruction);
            rope
        })
}

#[cfg(test)]
mod tests {
    use super::vec2::Vec2;
    use super::*;

    #[test]
    fn parsing_works() {
        let test_file = File::open("tests/day_09.txt").expect("Couldn't find test file!");
        let input = parse_input(&test_file, 0);
        assert_eq!(input.get_head(), Vec2(2, 2));
        assert_eq!(input.get_tail(), Vec2(1, 2));
    }

    #[test]
    fn exercise_1_works() {
        let test_file = File::open("tests/day_09.txt").expect("Couldn't find test file!");
        let input = parse_input(&test_file, 0);
        assert_eq!(input.get_tail_visited_positions_count(), 13);
    }

    #[test]
    fn exercise_2_works() {
        let test_file = File::open("tests/day_09.txt").expect("Couldn't find test file!");
        let input = parse_input(&test_file, 8);
        assert_eq!(input.get_tail_visited_positions_count(), 1);
    }
}
