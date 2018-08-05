use std::io;
use std::collections;

struct Player {
    name: String,
    points: i32,
}

impl Player {
    fn play(&self) {

    }

    fn new(name: String) -> Player {
        Player {
            name,
            points: 0,
        }
    }
}

struct Puzzle {
    category: String,
    solution: String,
    dashes: String,
}

impl Puzzle {
    fn new() -> Puzzle {
        let category: String = "Fruit".to_string();
        let solution: String = "APPLE".to_string();
        let (dashes, solution) = get_dashes_from_(solution);

        Puzzle {
            category,
            solution,
            dashes,
        }
    }
}

fn main() {
    let mut continue_playing = true;
    while continue_playing {
        continue_playing = play_game();
    }
}

fn play_game() -> bool {

    print_intro_screen();
    let players = build_players(get_number_of_players());
    let puzzle = Puzzle::new();
    for player in players {

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

