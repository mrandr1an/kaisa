open Kaisa.Ast.ParsedAst

let () =
  let lexbuf = Lexing.from_string "13 + &b" in
  let res = Kaisa.Parser.main Kaisa.Lexer.read_token lexbuf in 
  print_endline (show_expr res) 


