use std::fmt;

const DEFAULT_REGISTER_X_VALUE: i32 = 1;

const CRT_MONITOR_WIDTH: usize = 40;
const SPRITE_WIDTH: usize = 3;

#[derive(Debug)]
pub struct CPU {
    current_cycle: u32,
    register_x: i32,
    register_x_log: Vec<i32>,
    crt_monitor: CRTMonitor,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            current_cycle: 0,
            register_x: DEFAULT_REGISTER_X_VALUE,
            register_x_log: Vec::new(),
            crt_monitor: CRTMonitor::default(),
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => self.advance_cycle(None),
            Instruction::AddX(amount) => {
                self.advance_cycle(None);
                self.advance_cycle(Some(*amount));
            }
        }
    }

    pub fn get_log(&self) -> Vec<i32> {
        self.register_x_log.clone()
    }

    pub fn get_pixels(&self) -> Vec<Vec<Pixel>> {
        self.crt_monitor
            .pixels
            .chunks(CRT_MONITOR_WIDTH)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    fn advance_cycle(&mut self, add_amount: Option<i32>) {
        self.start_cycle();
        self.during_cycle();
        self.end_of_cycle(add_amount);
    }

    #[inline]
    fn start_cycle(&mut self) {
        self.current_cycle += 1;
    }

    #[inline]
    fn during_cycle(&mut self) {
        self.add_log_entry();
        self.crt_monitor.draw_pixel(self.register_x);
    }

    #[inline]
    fn end_of_cycle(&mut self, add_amount: Option<i32>) {
        if let Some(amount) = add_amount {
            self.register_x += amount;
        }
    }

    fn add_log_entry(&mut self) {
        self.register_x_log.push(self.register_x);
    }
}

#[derive(Debug)]
pub enum Instruction {
    AddX(i32),
    Noop,
}

impl Instruction {
    pub fn from(line: &str) -> Self {
        let words: Vec<&str> = line.split_whitespace().collect();

        match words.as_slice() {
            ["noop"] => Self::Noop,
            ["addx", amount] => Self::AddX(amount.parse().unwrap()),
            _ => panic!("Inoput has wrong number of attributes!"),
        }
    }
}

#[derive(Debug, Default)]
struct CRTMonitor {
    pixels: Vec<Pixel>,
    next_index_to_draw: usize,
}

impl CRTMonitor {
    pub fn draw_pixel(&mut self, sprite_middle_position: i32) {
        let sprite_positions: Vec<i32> = (0..SPRITE_WIDTH)
            .map(|offset| offset as i32 - (SPRITE_WIDTH as i32 / 2))
            .map(|offset| sprite_middle_position - offset)
            .collect();

        let pixel_to_draw = if sprite_positions
            .iter()
            .all(|&position| position != self.next_index_to_draw as i32)
        {
            Pixel::Dark
        } else {
            Pixel::Lit
        };

        self.pixels.push(pixel_to_draw);
        self.next_index_to_draw = (self.next_index_to_draw + 1) % CRT_MONITOR_WIDTH;
    }
}

#[derive(Debug, Clone)]
pub enum Pixel {
    Lit,
    Dark,
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let character = match self {
            Pixel::Lit => '#',
            Pixel::Dark => '.',
        };

        write!(f, "{}", character)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_works() {
        let mut cpu = CPU::new();
        let instructions = vec![
            Instruction::Noop,
            Instruction::AddX(3),
            Instruction::AddX(-5),
            Instruction::Noop,
        ];
        for instruction in &instructions {
            cpu.execute(&instruction);
        }
        assert_eq!(cpu.get_log(), vec![1, 1, 1, 4, 4, -1]);
    }
}
