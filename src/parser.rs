use codespan::*;
use super ast::Token;

enum TokenizerState {
	Start,
	RightBracket,
	LeftBracket,
	Symbol,
	Number,
	Whitespace,
	Comment,
}

fn tokenize(source: &str)-> Vec<ast::Token>{
	let result = Vec.new();
	let mut start = 0;

	loop {
	let mut state = Start;
	let mut end = start

		for c in source[start..].chars() {

		}
	}
}

