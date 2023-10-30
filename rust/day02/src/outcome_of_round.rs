use crate::shape::Shape;

pub enum OutcomeOfRound {
    Win,
    Draw,
    Lose,
}

impl OutcomeOfRound {
    pub fn new(my_choice: &Shape, opponents_choice: &Shape) -> Self {
        match (my_choice, opponents_choice) {
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => Self::Win,
            (Shape::Rock, Shape::Rock)
            | (Shape::Paper, Shape::Paper)
            | (Shape::Scissors, Shape::Scissors) => Self::Draw,
            _ => Self::Lose,
        }
    }

    pub fn new_from_guide(value: &str) -> Self {
        match value {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }

    pub fn get_score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}
