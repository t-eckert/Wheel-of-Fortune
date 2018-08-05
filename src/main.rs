use std::io;
use std::collections;

struct Player {
    name: String,
    points: i32,
}

impl Player {
    fn new(name: String) -> Player {
        // Creates a new Player object with name, player starts with 0 points.

        Player {
            name,
            points: 0,
        }
    }

    fn play(&self, mut puzzle: Puzzle) -> Puzzle {
        // Player can guess a letter or phrase. If correct, updates puzzle_board object.

        println!("{}, guess a letter or the phrase: ", self.name.trim());
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        if puzzle.contains(guess.clone()) {
            puzzle.update(guess);
        }

        puzzle
    }
}

struct Puzzle {
    category: String,
    solution: String,
    current_puzzle_board: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        // Creates a new Puzzle object.

        let category: String = String::from("Fruit");
        let solution: String = String::from("APPLE");
        let (current_puzzle_board, solution) = get_dashes_from_(solution);

        Puzzle {
            category,
            solution,
            current_puzzle_board,
        }
    }

    fn print(&self) {
        // Prints the category and current state of puzzle.

        println!("\nThe category is {}", self.category);
        println!("{}\n", self.current_puzzle_board);
    }

    fn contains(&self, guess: String) -> bool {
        // Checks if the guess is single char or string and if correct.

        let trimmed_guess = guess.trim().to_string();
        
        if trimmed_guess.len() == 1 {
            self.check_guess_char(guess.chars().next().unwrap())
        }
        else {
            self.check_guess_string(trimmed_guess)
        }
    }

    fn check_guess_string(&self, guess: String) -> bool {
        guess == self.solution
    }

    fn check_guess_char(&self, guess: char) -> bool {
        self.solution.contains(guess)
    }

    fn update(&mut self, guess: String) {
        let mut i: usize = 0;
        let mut current_puzzle_board_char_vec = Vec::new();

        for character in self.current_puzzle_board.chars() {
            current_puzzle_board_char_vec.push(character);
        }

        for guess_character in guess.chars() {
            i = 0;
            for solution_character in self.solution.chars() {
                if guess_character == solution_character {
                    current_puzzle_board_char_vec[i] = guess_character;
                }
                i += 1;
            }
        }
    }

    fn solved(&self) -> bool {
        self.current_puzzle_board == self.solution
    }
}

fn main() {
    let mut continue_playing = true;
    while continue_playing {
        continue_playing = play_game();
    }
}

fn play_game() -> bool {

    let mut solved = false;

    print_intro_screen();
    let players = build_players(get_number_of_players());
    let mut puzzle = Puzzle::new();

    for player in players {
        puzzle.print();
        puzzle = player.play(puzzle);
        solved = puzzle.solved();
        if solved {
            break;
        }
    }
    
    false
}

fn print_intro_screen() {
    println!("Welcome to Wheel of Fortune!");
}

fn build_players(n: u32) -> Vec<Player> {
    // instantiates n Player objects 

    let mut players: Vec<Player> = Vec::new();

    for i in 0..n {
        let mut player_name = String::new();
        println!("Player {}, what is your name?", i +1);
        io::stdin().read_line(&mut player_name).expect("Failed to read line");

        let player = Player::new(player_name);

        players.push(player);
    }

    players
}

fn get_number_of_players() -> u32 {
    // asks how many players are playing the game.

    println!("How many players? ");
    let mut num_players = String::new();
    io::stdin().read_line(&mut num_players).expect("Failed to read line");
    let num_players: u32 = num_players.trim().parse().expect("Please type a number!");

    // TASK: Limit to 3 players

    num_players
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

// fn spinWheel -> integer value of pts. (eventually a lose a turn result)

// fn getPhrase() returns a phrase from a JSON file

