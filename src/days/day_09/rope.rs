use std::{collections::HashSet, iter};

use super::vec2::Vec2;

#[derive(Debug)]
pub struct Rope {
    head: Vec2,
    knots: Vec<Vec2>,
    tail: Vec2,
    tail_visited_positions: HashSet<Vec2>,
}

pub enum Instruction {
    Down(usize),
    Up(usize),
    Left(usize),
    Right(usize),
}

impl Instruction {
    pub fn from(line: &str) -> Self {
        let words: Vec<&str> = line.split_whitespace().collect();

        match words.as_slice() {
            ["D", steps] => Self::Down(steps.parse().unwrap()),
            ["U", steps] => Self::Up(steps.parse().unwrap()),
            ["L", steps] => Self::Left(steps.parse().unwrap()),
            ["R", steps] => Self::Right(steps.parse().unwrap()),
            _ => panic!("Input has wrong number of attributes!"),
        }
    }
}

impl Rope {
    pub fn new(additional_knots: usize) -> Self {
        let mut new_rope = Self {
            head: Vec2::ZERO,
            knots: vec![Vec2::ZERO; additional_knots],
            tail: Vec2::ZERO,
            tail_visited_positions: HashSet::new(),
        };

        new_rope.register_tail_position();

        new_rope
    }

    pub fn move_head(&mut self, instruction: &Instruction) {
        let (movement, times) = match instruction {
            Instruction::Down(steps) => (Vec2::DOWN, *steps),
            Instruction::Up(steps) => (Vec2::UP, *steps),
            Instruction::Left(steps) => (Vec2::LEFT, *steps),
            Instruction::Right(steps) => (Vec2::RIGHT, *steps),
        };

        for _ in 0..times {
            self.head += movement;
            self.update_knot_positions();
            self.register_tail_position();
        }
    }

    pub fn get_tail_visited_positions_count(&self) -> usize {
        self.tail_visited_positions.len()
    }

    fn update_knot_positions(&mut self) {
        let mut previous_knot = self.head;

        let tail_iter = iter::once(&mut self.tail);

        for knot in self.knots.iter_mut().chain(tail_iter) {
            if previous_knot.distance(*knot) <= 1 {
                return;
            }

            let difference = previous_knot - *knot;

            match difference {
                Vec2(2, 0) => *knot += Vec2::RIGHT,
                Vec2(0, 2) => *knot += Vec2::UP,
                Vec2(-2, 0) => *knot += Vec2::LEFT,
                Vec2(0, -2) => *knot += Vec2::DOWN,
                Vec2(2, 2) | Vec2(2, 1) | Vec2(1, 2) => *knot += Vec2::RIGHT + Vec2::UP,
                Vec2(2, -2) | Vec2(2, -1) | Vec2(1, -2) => *knot += Vec2::RIGHT + Vec2::DOWN,
                Vec2(-2, 2) | Vec2(-2, 1) | Vec2(-1, 2) => *knot += Vec2::LEFT + Vec2::UP,
                Vec2(-2, -2) | Vec2(-2, -1) | Vec2(-1, -2) => *knot += Vec2::LEFT + Vec2::DOWN,
                _ => panic!("Unsupported movement!"),
            }

            previous_knot = *knot;
        }
    }

    fn register_tail_position(&mut self) {
        self.tail_visited_positions.insert(self.tail);
    }

    #[cfg(test)]
    pub fn get_head(&self) -> Vec2 {
        self.head
    }

    #[cfg(test)]
    pub fn get_tail(&self) -> Vec2 {
        self.tail
    }
}
