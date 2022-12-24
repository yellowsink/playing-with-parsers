use crate::lexer::Span;

#[derive(Debug)]
pub struct Expr {
	pub span: Span,
	pub node: Expr_
}

#[derive(Debug)]
pub enum Expr_ {
	Literal(f64),
	Add(Box<Expr>, Box<Expr>),
	Minus(Box<Expr>, Box<Expr>),
	Mult(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>)
}