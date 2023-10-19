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

    pub fn get_score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}
