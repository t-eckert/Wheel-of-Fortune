pub struct Announcer {
	pub welcome: String
}

impl Announcer {
	pub fn new(round: u32) -> Announcer {
		Announcer {
			welcome : format!("Welcome to Wheel of Fortune! Round {}", round),
		}
	}
}