use codespan::*;
use super ast::Token;

enum TokenizerState {
	Start,
	Rparen,
	Lparen,
	Symbol,
	Number,
	Whitespace,
	Comment,
}

fn tokenize(source: &str)-> Vec<ast::Token>{
	let mut result = Vec::new();
	let mut start = 0;

	loop {
	let mut state = Start;
	let mut end = start

		for c in source[start..].chars() {
			let next = match state {
				Start => match c {
					'('=> Some(Lparen),
					')'=> Some(Rparen),
					'0'...'9'=> Some(Number)
					'a'...'z'

				}
			}

		}
	}
}

