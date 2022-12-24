open System
open System.IO
open FSharp.Text.Lexing

let stdinReader = new StreamReader(Console.OpenStandardInput())
let stdin = stdinReader.ReadToEnd()

let lexbuf =  LexBuffer<char>.FromString stdin

let parsed = Parser.expr Lexer.expr lexbuf

let rec evaluate =
    function
    | Ast.Number n -> n
    | Ast.Add(n1, n2) -> evaluate n1 + evaluate n2
    | Ast.Minus(n1, n2) -> evaluate n1 - evaluate n2
    | Ast.Mult(n1, n2) -> evaluate n1 * evaluate n2
    | Ast.Div(n1, n2) -> evaluate n1 / evaluate n2

Console.WriteLine(evaluate parsed)