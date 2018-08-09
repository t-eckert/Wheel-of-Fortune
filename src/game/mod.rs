use std::io;

mod puzzle;
use self::puzzle::Puzzle;

mod player;
use self::player::Player;

mod wheel;
use self::wheel::Wheel;

struct Announcer {
	welcome: String
}

impl Announcer {
	fn new(round: u32) -> Announcer {
		Announcer {
			welcome : format!("Welcome to Wheel of Fortune! Round {}", round),
		}
	}
}

pub struct Game {
	puzzle: Puzzle,
	round: u32,
	players: Vec<Player>,
	wheel: Wheel,
}

// initialization of Game
impl Game {
	pub fn new(round: u32) -> Game {
		Game {
			puzzle: Puzzle::new(),
			round,
			players: Vec::new(),
			wheel: Wheel::new(),
		}
	}
}

// Game functions
impl Game {
	pub fn play(&self) -> bool {
		// Begins a new round, returns if a player wants to play again.
		let mut puzzle: Puzzle = Puzzle::new();

		let announcer: Announcer = Announcer::new(self.round);
		let mut solved = false;

		println!("{}", announcer.welcome);

		let players: Vec<Player> = self.init_players();
		let mut guess: String;

		while !solved {
        	for player in &players {
            	puzzle.print();
            	guess = player.play();

				if puzzle.guesses.contains(&guess) {
            		println!("That has already been guessed.");
        		}
        		else {
            		puzzle.guesses.push(guess.clone());
        		}

        		if puzzle.contains(guess.clone()) {
            		puzzle.update(guess);
        		}

            	solved = puzzle.solved();
        	}
    	}

		false
	}

	fn init_players(&self) -> Vec<Player> {
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
}