module Ast

type Expr =
    | Number of float
    | Add of Expr * Expr
    | Minus of Expr * Expr
    | Mult of Expr * Expr
    | Div of Expr * Expr