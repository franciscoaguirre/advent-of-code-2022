mod exercise_1;
mod exercise_2;
mod game;
mod monkey;

use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

use game::Game;
use monkey::Monkey;

const MONKEY_NUMBER_OF_LINES: usize = 7;

pub fn solve(mut input_file: File) {
    let mut game_1 = parse_input(&input_file);

    let solution_1 = exercise_1::solve(&mut game_1);
    println!("Exercise 1: {solution_1}");

    input_file.rewind().unwrap();

    let mut game_2 = parse_input(&input_file);

    let solution_2 = exercise_2::solve(&mut game_2);
    println!("Exercise 2: {solution_2}");
}

fn parse_input(input_file: &File) -> Game {
    let lines: Vec<String> = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let monkeys: Vec<Monkey> = lines
        .chunks(MONKEY_NUMBER_OF_LINES)
        .map(|monkey_raw| Monkey::from(monkey_raw))
        .collect();

    Game::new(monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_1_works() {
        let test_file = File::open("tests/day_11.txt").expect("Couldn't find test file!");
        let mut game = parse_input(&test_file);
        let solution = exercise_1::solve(&mut game);
        assert_eq!(solution, 10_605);
    }

    #[test]
    fn exercise_2_works() {
        let test_file = File::open("tests/day_11.txt").expect("Couldn't find test file!");
        let mut game = parse_input(&test_file);
        let solution = exercise_2::solve(&mut game);
        assert_eq!(solution, 2_713_310_158);
    }
}
