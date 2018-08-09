extern crate rand;
use rand::Rng;
use std::io;
// use std::collections;

#[cfg(test)]
mod main_tests;

mod game;
use game::Game;

fn main() {
    let mut another_round = true;
    let mut round = 1;

    while another_round {
        let game = Game::new(round);
        another_round = game.play();
        round += 1;
    }
}


fn spin_wheel() -> i32 {
    let mut rng = rand::thread_rng();
    let wheel = vec![2500, 600, 700, 600, 650, 500, 700, 600, 550, 500, 600, 650, 700, 800, 500, 650, 500, 900];
    let index = rng.gen_range(0, wheel.len());

    wheel[index]
}

// fn spinWheel -> integer value of pts. (eventually a lose a turn result)

// fn getPhrase() returns a phrase from a JSON file

