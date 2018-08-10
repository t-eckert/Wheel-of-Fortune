use std::io;

mod puzzle;
use self::puzzle::Puzzle;

mod player;
use self::player::Player;

mod announcer;
use self::announcer::Announcer;

mod wheel;
use self::wheel::Wheel;

pub struct Game {
	puzzle: Puzzle,
	round: u32,
	players: Vec<Player>,
	announcer: Announcer,
	wheel: Wheel,
}

// initialization of Game
impl Game {
	pub fn new(round: u32) -> Game {
		let players: Vec<Player> = init_players();

		Game {
			puzzle: Puzzle::new(),
			round,
			players,
			announcer: Announcer::new(round),
			wheel: Wheel::new(),
		}
	}
}

// Game functions
impl Game {
	pub fn play(&mut self) -> bool {
		// Begins a new round, returns if a player wants to play again.

		let mut solved = false;

		println!("{}", self.announcer.welcome);

		let mut guess: String;

		while !solved {
        	for player in &self.players {
            	self.puzzle.print();
            	guess = player.play();

				// Check if a guess has already been made.
				if self.puzzle.guesses.contains(&guess) {
            		println!("That has already been guessed.");
        		}
        		else {
            		self.puzzle.guesses.push(guess.clone());
        		}

				// Update the puzzle. 
        		if self.puzzle.contains(guess.clone()) {
            		self.puzzle.update(guess);
        		}

            	solved = self.puzzle.solved();
        	}
    	}

		println!("Would you like to play again?");
        let mut replay = String::new();
        io::stdin().read_line(&mut replay).expect("Failed to read line");

		if replay.chars().next().unwrap() == 'y' {
			true
		}
		else {
			false
		}
		
	}
}

fn init_players() -> Vec<Player> {
		let mut players: Vec<Player> = Vec::new();

		println!("How many players? ");
    	let mut num_players = String::new();
		io::stdin().read_line(&mut num_players).expect("Failed to read line");
		let num_players: u32 = num_players.trim().parse().expect("Please type a number!");

		for i in 0..num_players {
        	let mut player_name = String::new();
        	println!("Player {}, what is your name?", i +1);
        	io::stdin().read_line(&mut player_name).expect("Failed to read line");

        	let player = Player::new(player_name);

        	players.push(player);
    	}

    	players
	}