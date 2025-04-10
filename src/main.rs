use std::io::{self, Write};
use rand::random_range;
use colored::*;

const MAX_GUESSES: usize = 10;
const GOAL_LENGTH: usize = 4;
const NBR_COLORS: usize = 5;
const POSSIBILITIES: [char; NBR_COLORS] = ['R', 'G', 'B', 'W', 'Y'];

//Generates the goal code
fn generate_code(possibilities: [char; NBR_COLORS]) -> [char; GOAL_LENGTH]{

    let mut goal: [char; GOAL_LENGTH] = ['a'; GOAL_LENGTH];

    for x in 0..GOAL_LENGTH{

        goal[x] = possibilities[random_range(1..possibilities.len())];
    }

    goal
}

fn main() {

    let goal = generate_code(POSSIBILITIES);

    for x in goal {
        println!("{x}");
    }
}
