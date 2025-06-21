{
  open Parser	
}

let lowercase = ['a'-'z']
let uppercase = ['A'-'Z']
let digit     = ['0'-'9']
let ident_start = lowercase | uppercase | '_'
let ident_char = ident_start | digit

rule tokenize = parse
  | [' ' '\t' '\r' '\n']+ { tokenize lexbuf }  (* skip whitespace *)
  | digit+ as number { DEFAULT_NUMBER number }
  | "let" {LET}
  | "fn" {FN}
  | "type" {TYPE}
  | "val" {VAL}
  | "in"   {IN}
  | "match" {MATCH}
  | "with"  {WITH}
  | '('	    {LPAREN}
  | ')'	    {RPAREN}
  | '.'     {ACCESS}
  | '+'     {PLUS}
  | '-'     {MINUS}
  | '*'     {ASTERISK}
  | '/'     {RSLASH}
  | '!'     {FACT}
  | '&'     {REF}
  | ident_start ident_char* as id { IDENT id } 
  | eof     { EOF }


{
  let read_token = tokenize
}
