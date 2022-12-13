mod exercise_1;
mod exercise_2;
mod grid;

use std::fs::File;
use std::io::{BufRead, BufReader};

use grid::{Cell, Grid};

pub fn solve(input_file: File) {
    let input = parse_input(&input_file);

    let solution_1 = exercise_1::solve(&input);
    println!("Exercise 1: {solution_1}");

    let solution_2 = exercise_2::solve(&input);
    println!("Exercise 2: {solution_2}");
}

fn parse_input(input_file: &File) -> Grid {
    let mut start = Cell::default();
    let mut end = Cell::default();

    let cells: Vec<Vec<Cell>> = BufReader::new(input_file)
        .lines()
        .enumerate()
        .map(|(column_index, line)| (column_index, line.unwrap()))
        .map(|(column_index, line)| {
            line.chars()
                .enumerate()
                .map(|(row_index, char)| {
                    let position = (row_index, column_index);

                    if char == 'S' {
                        start = Cell::new(position, 0);
                        start
                    } else if char == 'E' {
                        end = Cell::new(position, 25);
                        end
                    } else {
                        Cell::new(position, char as u32 - 'a' as u32)
                    }
                })
                .collect::<Vec<Cell>>()
        })
        .collect();

    cells.into_iter().fold(Grid::default(), |mut grid, row| {
        grid.add_row(row);
        grid.start = start;
        grid.end = end;
        grid
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() {
        let test_file = File::open("tests/day_12.txt").expect("Couldn't find test file!");
        let grid = parse_input(&test_file);
        assert_eq!(grid.start, Cell::new((0, 0), 0));
        assert_eq!(grid.end, Cell::new((5, 2), 25));
        assert_eq!(
            grid.get_cells(),
            vec![
                vec![
                    Cell::new((0, 0), 0),
                    Cell::new((1, 0), 0),
                    Cell::new((2, 0), 1),
                    Cell::new((3, 0), 16),
                    Cell::new((4, 0), 15),
                    Cell::new((5, 0), 14),
                    Cell::new((6, 0), 13),
                    Cell::new((7, 0), 12),
                ],
                vec![
                    Cell::new((0, 1), 0),
                    Cell::new((1, 1), 1),
                    Cell::new((2, 1), 2),
                    Cell::new((3, 1), 17),
                    Cell::new((4, 1), 24),
                    Cell::new((5, 1), 23),
                    Cell::new((6, 1), 23),
                    Cell::new((7, 1), 11)
                ],
                vec![
                    Cell::new((0, 2), 0),
                    Cell::new((1, 2), 2),
                    Cell::new((2, 2), 2),
                    Cell::new((3, 2), 18),
                    Cell::new((4, 2), 25),
                    Cell::new((5, 2), 25),
                    Cell::new((6, 2), 23),
                    Cell::new((7, 2), 10),
                ],
                vec![
                    Cell::new((0, 3), 0),
                    Cell::new((1, 3), 2),
                    Cell::new((2, 3), 2),
                    Cell::new((3, 3), 19),
                    Cell::new((4, 3), 20),
                    Cell::new((5, 3), 21),
                    Cell::new((6, 3), 22),
                    Cell::new((7, 3), 9)
                ],
                vec![
                    Cell::new((0, 4), 0),
                    Cell::new((1, 4), 1),
                    Cell::new((2, 4), 3),
                    Cell::new((3, 4), 4),
                    Cell::new((4, 4), 5),
                    Cell::new((5, 4), 6),
                    Cell::new((6, 4), 7),
                    Cell::new((7, 4), 8)
                ]
            ]
        );
    }

    #[test]
    fn exercise_1_works() {
        let test_file = File::open("tests/day_12.txt").expect("Couldn't find test file!");
        let grid = parse_input(&test_file);
        let solution = exercise_1::solve(&grid);
        assert_eq!(solution, 31);
    }

    #[test]
    fn exercise_2_works() {
        let test_file = File::open("tests/day_12.txt").expect("Couldn't find test file!");
        let grid = parse_input(&test_file);
        let solution = exercise_2::solve(&grid);
        assert_eq!(solution, 29);
    }
}
