use crate::outcome_of_round::OutcomeOfRound;

pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn new_from_opponent(choice: &str) -> Self {
        match choice {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => unreachable!(),
        }
    }

    pub fn new_from_me(choice: &str) -> Self {
        match choice {
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }

    pub fn new_from_outcome_and_opponents_choice(
        outcome: &OutcomeOfRound,
        opponents_choice: &Shape,
    ) -> Self {
        match (outcome, opponents_choice) {
            (OutcomeOfRound::Win, Shape::Scissors)
            | (OutcomeOfRound::Draw, Shape::Rock)
            | (OutcomeOfRound::Lose, Shape::Paper) => Self::Rock,
            (OutcomeOfRound::Win, Shape::Rock)
            | (OutcomeOfRound::Draw, Shape::Paper)
            | (OutcomeOfRound::Lose, Shape::Scissors) => Self::Paper,
            _ => Self::Scissors,
        }
    }

    pub fn get_score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}
