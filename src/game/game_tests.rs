use super::*;
#[test]
fn test_game_creation() {
	// arrange
	test_round: u32 = 1;

	// act
	test_game: Game = Game::new(test_round);

	// assert
	assert_eq!(test_round, test_game.round);
}