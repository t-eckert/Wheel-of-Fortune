use std::io;

struct Player {
    name: String,
    points: i32,
}

fn main() {
    let mut continue_playing = true;
    while continue_playing {
        continue_playing = play_game();
    }
}

fn play_game() -> bool {

    print_intro_screen();

    println!("How many players? ");
    let mut num_players = String::new();
    io::stdin().read_line(&mut num_players).expect("Failed to read line");

    false
}

fn print_intro_screen() {
    println!("Welcome to Wheel of Fortune!");
}

// fn spinWheel -> integer value of pts. (eventually a lose a turn result)

// struct Player -> able to play the game
// fn createPlayer -> initialize instance of Player

// fn getPhrase() returns a phrase from a JSON file

