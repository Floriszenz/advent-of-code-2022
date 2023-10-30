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
        let mut assumed_total_score = 0;
        let mut actual_total_score = 0;

        for line in strategy_guide.lines() {
            let line = line.unwrap();
            let mut columns = line.split_whitespace();
            let opponents_choice = Shape::new_from_opponent(columns.next().unwrap());
            let second_column_value = columns.next().unwrap();

            assumed_total_score += {
                let my_choice = Shape::new_from_me(second_column_value);
                let outcome_of_round = OutcomeOfRound::new(&my_choice, &opponents_choice);

                my_choice.get_score() + outcome_of_round.get_score()
            };
            actual_total_score += {
                let outcome_of_round = OutcomeOfRound::new_from_guide(second_column_value);
                let my_choice = Shape::new_from_outcome_and_opponents_choice(
                    &outcome_of_round,
                    &opponents_choice,
                );

                my_choice.get_score() + outcome_of_round.get_score()
            }
        }

        println!("Assumed total score for the game is: {assumed_total_score}");
        println!("Actual total score for the game is: {actual_total_score}");
    } else {
        panic!("Did not provide path to strategy guide file");
    }
}
