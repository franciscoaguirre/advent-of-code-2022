use super::Game;

pub fn solve(game: &mut Game) -> u64 {
    game.run(10_000, true);
    game.get_monkey_business()
}
