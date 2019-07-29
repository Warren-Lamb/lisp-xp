use codespan::*;


#[derive(PartialEq,Debug)]
pub struct Token {
	pub kind : TokenKind,
	span : Span<ByteIndex>,
}

#[derive(PartialEq,Debug)]
pub enum TokenKind {
	Symbol(String),
	Number(i64),
	LeftBracket,
	RightBracket,
}

impl Token {
	pub fn with_span(kind: TokenKind, span: Span<ByteIndex>)-> Self{
		Token{ kind, span}
	}
}

pub enum Expr {
	Symbol(Token,String),
	Number(Token,i64),
	Define(Token,Token,Token,Box<Expr>,Token),
	If(Token,Token,Box<Expr>,Box<Expr>,Box<Expr>,Token),
	Call(Token,Token,Vec<Expr>,Token),
}