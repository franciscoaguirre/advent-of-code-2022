use super::Monkey;

pub struct Game {
    players: Vec<Monkey>,
    current_round: usize,
    too_worried: bool,
}

impl Game {
    pub fn new(mut players: Vec<Monkey>) -> Self {
        let mut divisors = Vec::new();
        for player in players.iter() {
            divisors.push(player.divisible_by);
        }
        let common_divisor: u64 = divisors.iter().product();
        for player in players.iter_mut() {
            player.set_common_divisor(common_divisor);
        }

        Self {
            players,
            current_round: 0,
            too_worried: false,
        }
    }

    pub fn run(&mut self, number_of_rounds: usize, too_worried: bool) {
        self.too_worried = too_worried;
        for _ in 0..number_of_rounds {
            self.play_round();
        }
    }

    pub fn get_monkey_business(&self) -> u64 {
        let mut inspection_counts: Vec<u32> = self
            .players
            .iter()
            .map(|monkey| monkey.get_inspection_count())
            .collect();

        inspection_counts.sort();
        inspection_counts.reverse();

        inspection_counts[0] as u64 * inspection_counts[1] as u64
    }

    fn play_round(&mut self) {
        self.current_round += 1;

        for monkey_index in 0..self.players.len() {
            let items_thrown = self.players[monkey_index].inspect_items(self.too_worried);

            for (item, to_whom) in items_thrown {
                self.players[to_whom].add_item(item);
            }
        }
    }
}
