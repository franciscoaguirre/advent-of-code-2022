use std::cmp::{Ord, Ordering, PartialOrd, Reverse};
use std::collections::HashMap;

use priority_queue::PriorityQueue;

#[derive(Debug, Default)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    pub start: Cell,
    pub end: Cell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Cell {
    position: Position,
    height: Height,
}

type Position = (usize, usize);

impl Cell {
    pub fn new(position: Position, height: Height) -> Self {
        Self { position, height }
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

fn compare_cells(a: &Cell, b: &Cell) -> Ordering {
    let height = a.get_height();
    let other_height = b.get_height();
    height.cmp(&other_height)
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_cells(self, other))
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_cells(self, other)
    }
}

type Height = u32;

const EDGE_COST: u32 = 1;

impl Grid {
    pub fn add_row(&mut self, row: Vec<Cell>) {
        self.cells.push(row);
    }

    fn neighbors(&self, cell: &Cell) -> Vec<&Cell> {
        let neighbors_to_check = match cell.position {
            (0, 0) => vec![(1, 0), (0, 1)],
            (x, 0) => vec![(x, 1), (x - 1, 0), (x + 1, 0)],
            (0, y) => vec![(1, y), (0, y - 1), (0, y + 1)],
            (x, y) => vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)],
        };

        let neighbors: Vec<&Cell> = neighbors_to_check
            .iter()
            .filter(|neighbor| neighbor.1 < self.cells.len() && neighbor.0 < self.cells[0].len())
            .map(|neighbor| &self.cells[neighbor.1][neighbor.0])
            .filter(|neighbor| neighbor.get_height() as i32 - cell.get_height() as i32 <= 1)
            .collect();

        neighbors
    }

    pub fn get_all_cells_with_height(&self, height: Height) -> Vec<Cell> {
        self.cells
            .concat()
            .iter()
            .filter(|cell| cell.get_height() == height)
            .map(|&cell| cell)
            .collect()
    }

    #[cfg(test)]
    pub fn get_cells(&self) -> Vec<Vec<Cell>> {
        self.cells.clone()
    }
}

pub struct Pathfinding {
    start: Cell,
    frontier: PriorityQueue<Cell, Reverse<u32>>,
    came_from: HashMap<Cell, Option<Cell>>,
    cost_so_far: HashMap<Cell, u32>,
}

impl Pathfinding {
    pub fn new(start: Cell) -> Self {
        let mut pathfinding = Self {
            start,
            frontier: PriorityQueue::new(),
            came_from: HashMap::new(),
            cost_so_far: HashMap::new(),
        };

        pathfinding.initialize();

        pathfinding
    }

    fn initialize(&mut self) {
        self.frontier.push(self.start, Reverse(0));
        self.came_from.insert(self.start, None);
        self.cost_so_far.insert(self.start, 0u32);
    }

    pub fn run(&mut self, grid: &Grid, end_cell: Cell) -> Vec<Cell> {
        while !self.frontier.is_empty() {
            let (current, _) = self.frontier.pop().unwrap();

            if current == end_cell {
                return self.select_path(end_cell);
            }

            for next in grid.neighbors(&current) {
                let new_cost = self.cost_so_far[&current] + EDGE_COST;

                if !self.cost_so_far.contains_key(next)
                    || new_cost < *self.cost_so_far.get(next).unwrap()
                {
                    self.cost_so_far.insert(*next, new_cost);
                    let priority = new_cost;
                    self.frontier.push(*next, Reverse(priority));
                    self.came_from.insert(*next, Some(current));
                }
            }
        }

        Vec::new()
    }

    fn select_path(&self, end: Cell) -> Vec<Cell> {
        let mut current_cell = end;
        let mut selected_cells = Vec::new();

        while current_cell != self.start {
            selected_cells.push(current_cell);

            if self.came_from.get(&current_cell) == None {
                // println!("Did not find path!");
                break;
            }

            current_cell = self.came_from.get(&current_cell).unwrap().unwrap();
        }

        selected_cells
    }
}
