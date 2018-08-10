#[cfg(test)]
mod main_tests;

mod game;
use game::Game;

fn main() {
    let mut another_round = true;
    let mut round = 1;

    while another_round {
        let mut game = Game::new(round);
        another_round = game.play();
        round += 1;
    }
}