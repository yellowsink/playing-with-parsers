# Playing with parsers

Implementing a parser with the following features in many langs/parser generators.

The goal is to compare tools and see which one is the comfiest to use.

## Spec

expressions are:
- numbers, as 64bit floats, of digit+ | digit+.digit+
- parenthesis with ( ) that can contain any expression (number or operation)
- operations: + - * /, where the precedence is * / then + -

The input is a single expression

The output should be the result of the calculation.

## Evaluations of parsers