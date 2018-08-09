use super::*;
#[test]
fn test_player_creation() {
	// arrange
	let expected_name = String::from("Thomas");
	let expected_points = 0;

	// act
	let test_player = Player::new("Thomas".to_string());

	// assert
	assert_eq!(expected_name, test_player.name);
	assert_eq!(expected_points, test_player.points);
}