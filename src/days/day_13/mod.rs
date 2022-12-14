mod exercise_1;
mod exercise_2;
mod packet;

use std::fs::File;
use std::io::{BufRead, BufReader};

use packet::Packet;

pub fn solve(input_file: File) {
    let input = parse_input(&input_file);

    let solution_1 = exercise_1::solve(&input);
    println!("Exercise 1: {solution_1}");

    let solution_2 = exercise_2::solve(&input);
    println!("Exercise 2: {solution_2}");
}

fn parse_input(input_file: &File) -> Vec<(Packet, Packet)> {
    let lines: Vec<String> = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    lines
        .chunks(3)
        .map(|lines| (Packet::from(&lines[0]), Packet::from(&lines[1])))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() {
        let test_file = File::open("tests/day_13.txt").expect("Couldn't find test file!");
        let input = parse_input(&test_file);
        assert_eq!(
            input[0],
            (
                vec![1.into(), 1.into(), 3.into(), 1.into(), 1.into()].into(),
                vec![1.into(), 1.into(), 5.into(), 1.into(), 1.into()].into(),
            ),
        );
        assert_eq!(
            input[1],
            (
                vec![
                    vec![1.into()].into(),
                    vec![2.into(), 3.into(), 4.into()].into(),
                ]
                .into(),
                vec![vec![1.into()].into(), 4.into()].into(),
            ),
        );
        assert_eq!(
            input[2],
            (
                vec![9.into()].into(),
                vec![vec![8.into(), 7.into(), 6.into()].into()].into(),
            ),
        );
        assert_eq!(
            input[3],
            (
                vec![vec![4.into(), 4.into(),].into(), 4.into(), 4.into(),].into(),
                vec![
                    vec![4.into(), 4.into(),].into(),
                    4.into(),
                    4.into(),
                    4.into(),
                ]
                .into(),
            ),
        );
        assert_eq!(
            input[4],
            (
                vec![7.into(), 7.into(), 7.into(), 7.into()].into(),
                vec![7.into(), 7.into(), 7.into()].into(),
            ),
        );
        assert_eq!(input[5], (vec![].into(), vec![3.into()].into(),),);
        assert_eq!(
            input[6],
            (
                vec![vec![vec![].into(),].into(),].into(),
                vec![vec![].into()].into(),
            ),
        );
        assert_eq!(
            input[7],
            (
                vec![
                    1.into(),
                    vec![
                        2.into(),
                        vec![
                            3.into(),
                            vec![4.into(), vec![5.into(), 6.into(), 7.into(),].into(),].into(),
                        ]
                        .into(),
                    ]
                    .into(),
                    8.into(),
                    9.into(),
                ]
                .into(),
                vec![
                    1.into(),
                    vec![
                        2.into(),
                        vec![
                            3.into(),
                            vec![4.into(), vec![5.into(), 6.into(), 0.into(),].into(),].into(),
                        ]
                        .into(),
                    ]
                    .into(),
                    8.into(),
                    9.into(),
                ]
                .into(),
            ),
        );
    }

    #[test]
    fn exercise_1_works() {
        let test_file = File::open("tests/day_13.txt").expect("Couldn't find test file!");
        let pairs = parse_input(&test_file);
        let solution = exercise_1::solve(&pairs);
        assert_eq!(solution, 13);
    }
}
