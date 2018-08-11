use std::io;


pub struct Player {
    name: String,
	points: i32,
}

// associated functions
impl Player {
    pub fn new(name: String) -> Player {
        // Creates a new Player object with name, player starts with 0 points.
        Player {
            name: name.trim().to_string(),
            points: 0,
        }
    }
}

// methods
impl Player {
    pub fn play(&self) -> String {
        // Player can guess a letter or phrase. If correct, updates puzzle_board object.

        // let mut money_earned = 0;
        println!("{}, you have {} points, guess a letter or the phrase: ", self.name,self.points);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        guess = guess.to_uppercase();

        guess
    }

    pub fn print_points(&self) {
        println!("{} you now have ${}", self.name, self.points);
    }
}
