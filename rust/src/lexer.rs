extern crate plex;
use plex::lexer;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),

    Whitespace,
    Add,
    Minus,
	Mult,
	Div,
    LParen,
    RParen,
}

lexer! {
    fn next_token(text: 'a) -> Token;

    r#"[ \t\r\n]+"# => Token::Whitespace,
    r#"\("# => Token::LParen,
    r#"\)"# => Token::RParen,
    r#"\+"# => Token::Add,
    r#"-"# => Token::Minus,
	r#"\*"# => Token::Mult,
    r#"/"# => Token::Div,

    r#"[0-9]+(\.[0-9]+)?"# => {
        if let Ok(i) = text.parse() {
            Token::Number(i)
        } else {
            panic!("number {} is out of range", text)
        }
    }
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            if let Some((tok, new_remaining)) = next_token(self.remaining) {

                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;

				if matches!(tok, Token::Whitespace) {
					continue;
				}

                return Some((tok, Span { lo, hi }));
            } else {
                return None;
            };
        }
    }
}