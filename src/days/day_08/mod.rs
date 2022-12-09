mod exercise_1;
mod exercise_2;

use std::fs::File;
use std::io::{BufRead, BufReader};

mod forest;
use forest::{Forest, Tree};

const RADIX: u32 = 10;

pub fn solve(input_file: File) {
    let input = parse_input(&input_file);

    let solution_1 = exercise_1::solve(&input);
    println!("Exercise 1: {solution_1}");

    let solution_2 = exercise_2::solve(&input);
    println!("Exercise 2: {solution_2}");
}

fn parse_input(input_file: &File) -> Forest {
    BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .fold(Forest::default(), |mut forest, line| {
            let row = line
                .chars()
                .map(|char| char.to_digit(RADIX).unwrap())
                .map(|digit| Tree::new(digit))
                .collect();
            forest.add_row(row);
            forest
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() {
        let test_file = File::open("tests/day_08.txt").expect("Couldn't find test file!");
        let input = parse_input(&test_file);
        assert_eq!(
            input.cells(),
            vec![
                vec![
                    Tree::new(3),
                    Tree::new(0),
                    Tree::new(3),
                    Tree::new(7),
                    Tree::new(3)
                ],
                vec![
                    Tree::new(2),
                    Tree::new(5),
                    Tree::new(5),
                    Tree::new(1),
                    Tree::new(2)
                ],
                vec![
                    Tree::new(6),
                    Tree::new(5),
                    Tree::new(3),
                    Tree::new(3),
                    Tree::new(2)
                ],
                vec![
                    Tree::new(3),
                    Tree::new(3),
                    Tree::new(5),
                    Tree::new(4),
                    Tree::new(9)
                ],
                vec![
                    Tree::new(3),
                    Tree::new(5),
                    Tree::new(3),
                    Tree::new(9),
                    Tree::new(0)
                ],
            ]
        );
    }

    #[test]
    fn exercise_1_works() {
        let test_file = File::open("tests/day_08.txt").expect("Couldn't find test file!");
        let input = parse_input(&test_file);
        let solution = exercise_1::solve(&input);
        assert_eq!(solution, 21);
    }

    #[test]
    fn exercise_2_works() {
        let test_file = File::open("tests/day_08.txt").expect("Couldn't find test file!");
        let input = parse_input(&test_file);
        let solution = exercise_2::solve(&input);
        assert_eq!(solution, 8);
    }
}
