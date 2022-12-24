#![allow(unused_braces)]

use crate::ast::*;
use crate::lexer::Token::*;
use crate::lexer::*;
use plex::parser;

parser! {
	fn parse_(Token, Span);

	(a, b) {
			Span {
			lo: a.lo,
			hi: b.hi
		}
	}

	expression: Expr {
		expression[lhs] Add fact[rhs] => Expr {
			span: span!(),
			node: Expr_::Add(Box::new(lhs), Box::new(rhs))
		},

		expression[lhs] Minus fact[rhs] => Expr {
			span: span!(),
			node: Expr_::Minus(Box::new(lhs), Box::new(rhs))
		},

		fact[f] => f
	}

	fact: Expr {
		fact[lhs] Mult atom[rhs] => Expr {
			span: span!(),
			node: Expr_::Mult(Box::new(lhs), Box::new(rhs))
		},

		fact[lhs] Div atom[rhs] => Expr {
			span: span!(),
			node: Expr_::Div(Box::new(lhs), Box::new(rhs))
		},

		atom[f] => f
	}

	atom: Expr {
		Number(f) => Expr {
			span: span!(),
			node: Expr_::Literal(f)
		},

		LParen expression[e] RParen => e,
	}
}

pub fn parse(lexer: &mut Lexer) -> Expr {
	let res = parse_(lexer);
	return res.unwrap();
}
