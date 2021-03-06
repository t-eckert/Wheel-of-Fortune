use std::collections::HashMap;
extern crate rand;
use self::rand::Rng;

pub struct Puzzle {
    category: String,
    solution: String,
    current_puzzle_board: String,
    pub guesses: Vec<String>,
}

fn get_puzzle() -> (String, String) {
    let puzzles: HashMap<&str, Vec<&str>> = [
        ("Fruit", vec!["APPLE", "PEACH"]),
        ("Phrases", vec!["THREES A CROWD."]),
    ].iter()
        .cloned()
        .collect();

    let mut rng = rand::thread_rng();

    let categories: Vec<&str> = puzzles.keys().cloned().collect();
    let category = rng.choose(&categories).unwrap();
    let solution = rng.choose(&puzzles[category]).unwrap();

    (String::from(*category), String::from(*solution))
}

// associated functions
impl Puzzle {
    pub fn new() -> Puzzle {
        // Creates a new Puzzle object.
        let (category, solution) = get_puzzle();
        let current_puzzle_board = get_dashes_from(&solution);
        let guesses = Vec::new();

        Puzzle {
            category,
            solution,
            current_puzzle_board,
            guesses,
        }
    }
}

// methods
impl Puzzle {
    pub fn print(&self) {
        // Prints the category and current state of puzzle.

        println!("\nThe category is {}", self.category);

        for character in self.current_puzzle_board.chars() {
            println!("{}", character)
        }

        println!();
    }

    pub fn contains(&self, guess: &str) -> bool {
        // Checks if the guess is single char or string and if correct.

        let trimmed_guess = guess.trim();

        if trimmed_guess.len() == 1 {
            self.check_guess_char(guess.chars().next().unwrap())
        } else {
            self.check_guess_string(trimmed_guess)
        }
    }

    fn check_guess_char(&self, guess: char) -> bool {
        self.solution.contains(guess)
    }

    fn check_guess_string(&self, guess: &str) -> bool {
        guess == self.solution
    }

    pub fn update(&mut self, guess: &str) -> i32 {
        // TASK: This can be a lot cleaner. Probably a Rust matching way of doing this.
        let mut num_in_solution: i32 = 0;

        let mut char_indicies: Vec<char> = self.current_puzzle_board.chars().collect();

        for character in guess.trim().chars() {
            for (solution_character, indices_character) in self.solution.chars().zip(char_indicies.iter_mut()) {
                if character == solution_character {
                    *indices_character = character;
                    num_in_solution += 1;
                }
            }

            // index = self.solution.chars().position(|c| c == character).unwrap();
            // println!("{}", index);
            // self.current_puzzle_board.replace_range(index..index+1, &character.to_string());
        }

        self.current_puzzle_board = char_indicies.into_iter().collect();

        num_in_solution
    }

    pub fn solved(&self) -> bool {
        self.current_puzzle_board == self.solution
    }
}

fn get_dashes_from(solution: &str) -> String {
    solution.chars()
        .map(|c|
            match c {
                ' ' | '.' => c,
                _ => '_'
            }
        )
        .collect()
}
