mod outcome_of_round;
mod shape;

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::outcome_of_round::OutcomeOfRound;
use crate::shape::Shape;

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        let strategy_guide = File::open(file_path).expect("Failed to open strategy_guide");
        let strategy_guide = BufReader::new(strategy_guide);
        let mut total_score = 0;

        for line in strategy_guide.lines() {
            let line = line.unwrap();
            let mut choices = line.split_whitespace();
            let opponents_choice = Shape::new_from_opponent(choices.next().unwrap());
            let my_choice = Shape::new_from_me(choices.next().unwrap());

            total_score += my_choice.get_score();

            let outcome_of_round = OutcomeOfRound::new(my_choice, opponents_choice);

            total_score += outcome_of_round.get_score();
        }

        println!("Assumed total score for the game is: {total_score}")
    } else {
        panic!("Did not provide path to strategy guide file");
    }
}
