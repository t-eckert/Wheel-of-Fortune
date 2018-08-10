

pub struct Puzzle {
    category: String,
    solution: String,
    current_puzzle_board: String,
    pub guesses: Vec<String>,
}

// associated functions
impl Puzzle {
    pub fn new() -> Puzzle {
        // Creates a new Puzzle object.

        let category: String = String::from("Fruit");
        let solution: String = String::from("APPLE");
        let (current_puzzle_board, solution) = get_dashes_from_(solution);
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

        let mut puzzle_board = Vec::new();
        for character in self.current_puzzle_board.chars() {
            puzzle_board.push(character);
            puzzle_board.push(' ');
        }

        let puzzle_board_string: String = puzzle_board.into_iter().collect();
        println!("{}\n", puzzle_board_string);
    }

    pub fn contains(&self, guess: String) -> bool {
        // Checks if the guess is single char or string and if correct.

        let trimmed_guess = guess.trim().to_string();
        
        if trimmed_guess.len() == 1 {
            self.check_guess_char(guess.chars().next().unwrap())
        }
        else {
            self.check_guess_string(trimmed_guess)
        }
    }

    fn check_guess_char(&self, guess: char) -> bool {
        self.solution.contains(guess)
    }

    fn check_guess_string(&self, guess: String) -> bool {
        guess == self.solution
    }

    pub fn update(&mut self, guess: String) -> i32 {
        // TASK: This can be a lot cleaner. Probably a Rust matching way of doing this.
        let mut char_indicies: Vec<char> = Vec::new();
        let mut index: usize;
        let mut num_in_solution: i32 = 0;

        for character in self.current_puzzle_board.chars() {
            char_indicies.push(character);
        }

        for character in guess.trim().chars() {
            index = 0;
            for solution_character in self.solution.chars() {
                if character == solution_character {
                    char_indicies[index] = character;
                    num_in_solution += 1;
                }
                index += 1;
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

fn get_dashes_from_(solution: String) -> (String, String) {
    let mut dashes_char_vec = vec![];

    for character in solution.chars() {
        if character == ' ' {
            dashes_char_vec.push(' ');
        }
        else {
            dashes_char_vec.push('_');
        }
    };

    let dashes: String = dashes_char_vec.into_iter().collect();

    (dashes, solution)
}