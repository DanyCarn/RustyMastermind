use std::{default, io::{self, stdin, stdout, Write}, str::Chars};
use rand::random_range;
use colored::*;
use std::fmt::Display;

const MAX_GUESSES: usize = 10;
const GOAL_LENGTH: usize = 4;
const NBR_COLORS: usize = 7;
const POSSIBILITIES: [char; NBR_COLORS] = ['R', 'G', 'B', 'W', 'Y', 'M', 'C'];
const TITLE: &str = "Mastermind";



//Generates the goal code
fn generate_code(possibilities: [char; NBR_COLORS]) -> [char; GOAL_LENGTH]{

    let mut goal: [char; GOAL_LENGTH] = ['a'; GOAL_LENGTH];

    for x in 0..GOAL_LENGTH{ 

        goal[x] = possibilities[random_range(1..possibilities.len())];
    }

    goal
}

//Verifies the entry of the user
fn verify_len(guess: &String) -> Result<bool, ColoredString>{
    
    for char in guess.trim().chars() {
        if POSSIBILITIES.contains(&char) {
            continue;
        } else {
            return Err("L'essai ne doit contenir que les caractères possibles.".red())
        }
    }

    if guess.trim().chars().count() == 4 {
        Ok(true)
    } else {
        return Err("L'essai ne doit faire que 4 caractères de long.".red())
    }
}

fn main() {

    let goal = generate_code(POSSIBILITIES);
    let mut tries:usize = 0;
    let mut guess: String = Default::default();
    let mut guess_chars: std::str::Chars<'_>;
    let mut verified_guess: [char; GOAL_LENGTH] = [' '; GOAL_LENGTH];
    let mut colored_guess: [ColoredString; GOAL_LENGTH] = Default::default();
    let mut guess_valid: bool = false;

    for x in 0..GOAL_LENGTH {
        colored_guess[x] = "_".white();
    }

    println!("{}", TITLE.bold().underline());
    println!("Devine le code de {} couleurs (R, G, B, W ou Y)", GOAL_LENGTH);
    //Shows goal only for debuging
    //println!("{:?}", goal);

    while tries < MAX_GUESSES { 

        guess_valid = false;
        verified_guess = Default::default();
        
        while !guess_valid {
            guess = Default::default();

            println!("Essai {}/{} :", tries+1, MAX_GUESSES);
            stdin().read_line(&mut guess);
    
            guess = guess.to_uppercase();
    
            match verify_len(&guess){
                Ok(x) => {guess_valid = x},
                Err(x) => println!("{x}")
            }
        }

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
                    'C' => colored_guess[x] = "C".cyan(),
                    'M' => colored_guess[x] = "M".magenta(),
                    _ => todo!()
                }

            } else {
                verified_guess[x] = '_';
                colored_guess[x] = "_".white();
            }
        }
        
        if verified_guess.contains(&'_'){

            print!("Résultat : ");
            for char in &colored_guess {
                print!("{char}");
            }
            println!();

        } else {
            print!("Bravo, vous avez gagné ! Le code était ");
            
            for char in goal {
                print!("{char}");
            }

            println!();
            break;
        }
        
        tries += 1;
    }

    if tries == MAX_GUESSES {

        print!("Mince, vous avez perdu ! Le code était ");
    
        for char in goal {
            print!("{char}");
        }
        println!();
    }

}
