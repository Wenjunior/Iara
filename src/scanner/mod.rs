pub struct Scanner {
	source_code: String
}

#[derive(Debug)]
pub struct Token;

impl Scanner {
	pub fn new(source_code: String) -> Self {
		Self {
			source_code
		}
	}

	pub fn scan(&self) -> Vec<Token> {
		let tokens = Vec::new();

		tokens
	}
}