#[derive(Debug)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    pub fn from(input: &str) -> Self {
        match input {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Not a valid letter value"),
        }
    }
}

impl RockPaperScissors {
    pub fn from(letter: &str) -> Self {
        match letter {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Not a valid letter value"),
        }
    }

    pub fn get_play_to(&self, desired_result: GameResult) -> Self {
        match desired_result {
            GameResult::Win => match self {
                Self::Paper => Self::Scissors,
                Self::Rock => Self::Paper,
                Self::Scissors => Self::Rock,
            },
            GameResult::Draw => match self {
                Self::Paper => Self::Paper,
                Self::Rock => Self::Rock,
                Self::Scissors => Self::Scissors,
            },
            GameResult::Lose => match self {
                Self::Paper => Self::Rock,
                Self::Rock => Self::Scissors,
                Self::Scissors => Self::Paper,
            },
        }
    }
}

pub struct Round {
    opponent_plays: RockPaperScissors,
    should_play: RockPaperScissors,
}

const LOSE_SCORE: u32 = 0;
const DRAW_SCORE: u32 = 3;
const WIN_SCORE: u32 = 6;

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;

impl Round {
    pub fn from(input: &str) -> Self {
        let plays: Vec<&str> = input.split_whitespace().collect();
        let opponent_plays = RockPaperScissors::from(plays[0]);
        let should_play = RockPaperScissors::from(plays[1]);

        Self {
            opponent_plays,
            should_play,
        }
    }

    /// Same as `from` but understands the second char as the desired
    /// outcome of the round and returns a play accordingly
    pub fn from_alternate(input: &str) -> Self {
        let plays: Vec<&str> = input.split_whitespace().collect();
        let opponent_plays = RockPaperScissors::from(plays[0]);
        let desired_outcome = GameResult::from(plays[1]);
        let should_play = opponent_plays.get_play_to(desired_outcome);

        Self {
            opponent_plays,
            should_play,
        }
    }

    pub fn calculate_score(&self) -> u32 {
        let mut score = 0;

        match self.should_play {
            RockPaperScissors::Rock => {
                score += ROCK_SCORE;
                match self.opponent_plays {
                    RockPaperScissors::Rock => score += DRAW_SCORE,
                    RockPaperScissors::Paper => score += LOSE_SCORE,
                    RockPaperScissors::Scissors => score += WIN_SCORE,
                }
            }
            RockPaperScissors::Paper => {
                score += PAPER_SCORE;
                match self.opponent_plays {
                    RockPaperScissors::Rock => score += WIN_SCORE,
                    RockPaperScissors::Paper => score += DRAW_SCORE,
                    RockPaperScissors::Scissors => score += LOSE_SCORE,
                }
            }
            RockPaperScissors::Scissors => {
                score += SCISSORS_SCORE;
                match self.opponent_plays {
                    RockPaperScissors::Rock => score += LOSE_SCORE,
                    RockPaperScissors::Paper => score += WIN_SCORE,
                    RockPaperScissors::Scissors => score += DRAW_SCORE,
                }
            }
        }

        score
    }
}
