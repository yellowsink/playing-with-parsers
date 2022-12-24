mod lexer;
mod parser;
mod ast;

use crate::lexer::Lexer;
use crate::parser::parse;
use crate::ast::*;
use std::io::Read;

fn evaluate(e: Expr) -> f64 {
	match e.node {
		Expr_::Literal(f) => f,
		Expr_::Add(e1, e2) => evaluate(*e1) + evaluate(*e2),
		Expr_::Minus(e1, e2) => evaluate(*e1) - evaluate(*e2),
		Expr_::Mult(e1, e2) => evaluate(*e1) * evaluate(*e2),
		Expr_::Div(e1, e2) => evaluate(*e1) / evaluate(*e2)
	}
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();

	let parsed = parse(&mut Lexer::new(&s));

	println!("{:?}", evaluate(parsed));
}
