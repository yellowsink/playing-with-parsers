# Playing with parsers

Implementing a parser with the following features in many langs/parser generators.

The goal is to compare tools and see which one is the comfiest to use.

## Spec

expressions are:

-   numbers, as 64bit floats, of digit+ | digit+.digit+
-   parenthesis with ( ) that can contain any expression (number or operation)
-   operations: + - _ /, where the precedence is _ / then + -

The input is a single expression

The output should be the result of the calculation.

## Evaluations of parsers

| TOC                           |
| ----------------------------- |
| [Rust: Plex](#rust-plex)      |
| [F#: FsLexYacc](#f-fslexyacc) |

### Rust: Plex

I like Plex's lexing a lot.

It is pretty easy to wrap your head around,
and while you end up implementing some of the logic yourself,
that boilerplate isn't too bad, and gives you a convenient way to discard whitespace etc.

The parser however, I really did not enjoy writing.

Looking again, it is mostly alright, but I had issues with Rust misbehaving,
and it's not particularly concise, it felt much more like just writing code than writing a grammar.

Some good bits but not overly keen to use this further.

### F#: FsLexYacc

FsLexYacc features a much more customised syntax that has patchy IDE support
and may overwhelm people unfamiliar with F#.

That said, I found the custom syntax quite nice, once I started to wrap my head around it.

I'm not hugely fond of FsLex.
It feels a bit over-magical, and you need to be comfortable with the two completely different
contexts of the lexing rules and the F# code that is called as a result.

Once you start to really understand what is happening though, it becomes mostly understandable
(common patterns such as whitespace recursively calling the lex rule as a way of skipping the token is very esoteric at first).

The parser, however, I like a LOT.

It still has a lot of custom syntax, but it follows stringent rules that make it quite easy to understand
after getting used to it:
 - `%{ code %}` will run `code` before the parser code, so you can import things for use later
 - `%xyz` is a keyword to teach FsYacc about your parser
	* `%start r` tells FsYacc about a rule that should be exposed as an entrypoint
	* `%token x y z` tells FsYacc to expose a list of tokens to the lexer
	* `%token <t> x` tells FsYacc to expose a token to the lexer, with a payload type
	* `%type <t> r` tells FsYacc what the returned type of a rule should be.
		This is only necessary on entrypoints, as the sub-rules are subject to F#'s type inference.
 - `%%` tells FsYacc that you're done describing your parser and want to start writing grammar.
 - grammar rules are of the form `rulename: token1 token2 token3 { expression } | ...`
	* the rulename allows you to use the rule in place of tokens in other rules or recursively
	* anything in `{}` is ran as F# code, and its returned value is the parsed node of that rule:
		create your AST nodes in this block!
	* the `|` specifies alternate matches, in descending preference
	* the values of each token/rule can be acquired in the expression as `$1, $2, $3, ...`

Writing parsers like this felt a lot more like writing grammar rules than writing code,
and creates incredibly compact yet very readable grammars - it's not dense, just compact.

I also liked that the tokens were specified in the parser file:
it felt much more centralised than defining a separate union just for tokens.

I would happily use FsLexYacc for more things, even if I'm not overly taken with the lexer,
because its very efficient and the parsers are clean and concise.