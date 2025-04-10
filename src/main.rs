use std::io::{self, stdin, stdout, Write};
use rand::random_range;
use colored::*;
use std::fmt::Display;

const MAX_GUESSES: usize = 10;
const GOAL_LENGTH: usize = 4;
const NBR_COLORS: usize = 5;
const POSSIBILITIES: [char; NBR_COLORS] = ['R', 'G', 'B', 'W', 'Y'];
const TITLE: &str = "Mastermind";

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
    let mut tries:usize = 0;
    let mut guess: String;
    let mut guess_chars: std::str::Chars<'_>;
    let mut verified_guess: [char; GOAL_LENGTH] = [' '; GOAL_LENGTH];
    let mut colored_guess: [ColoredString; GOAL_LENGTH] = Default::default();

    for x in 0..GOAL_LENGTH {
        colored_guess[x] = "_".white();
    }

    println!("{}", TITLE.bold().underline());
    println!("Devine le code de {} couleurs (R, G, B, W ou Y)", GOAL_LENGTH);
    //Shows goal only for debuging
    //println!("{:?}", goal);

    while tries < MAX_GUESSES { 

        guess = Default::default();
        verified_guess = Default::default();

        println!("Essai {}/{} :", tries+1, MAX_GUESSES);
        stdin().read_line(&mut guess);

        guess_chars = guess.chars();

        for x in 0..GOAL_LENGTH {

            if goal[x] == guess_chars.next().unwrap() {
                verified_guess[x] = goal[x];

                match goal[x] {
                    'R' => colored_guess[x] = "R".red(),
                    'G' => colored_guess[x] = "G".green(),
                    'B' => colored_guess[x] = "B".blue(),
                    'W' => colored_guess[x] = "W".white(),
                    'Y' => colored_guess[x] = "Y".yellow(),
                    _ => todo!()
                }

            } else {
                verified_guess[x] = '_';
                colored_guess[x] = "_".white();
            }
        }
        
        print!("Résultat : ");
        for char in &colored_guess {
            print!("{char}");
        }
        println!();

        tries += 1;
    }
}
