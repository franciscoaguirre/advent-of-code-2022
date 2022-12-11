use super::Game;

pub fn solve(game: &mut Game) -> u64 {
    game.run(20, false);
    game.get_monkey_business()
}
