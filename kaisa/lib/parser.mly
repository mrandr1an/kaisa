%{
  open Ast.ParsedAst
  open Lexing  
%}


/* Some sort of ID */
%token <string> IDENT
/* Default Number (decimal u64) */
%token <string> DEFAULT_NUMBER
/* Create expr */
%token LET
%token FN TYPE VAL
/* Continue expr */
%token IN
 /* PATTERN MATCH  */
%token MATCH WITH
%token LPAREN RPAREN
%token PLUS MINUS ASTERISK RSLASH ACCESS FACT REF DEREF 
%token EOF

%start <Ast.ParsedAst.expr> main

%left PLUS
%left MINUS
%left ASTERISK
%left RSLASH
%left FACT
%left ACCESS
%left DEREF
%left REF


%%

main:
  |  expr EOF { $1 }

expr:
  | IDENT { Id (mk_node $1 $startpos.pos_cnum $endpos.pos_cnum) }
  | operation {Operation (mk_node $1 $startpos.pos_cnum $endpos.pos_cnum) }
  | value {Val (mk_node $1 $startpos.pos_cnum $endpos.pos_cnum)}

value:
  | DEFAULT_NUMBER { DefaultNumber (mk_node $1 $startpos.pos_cnum $endpos.pos_cnum) }

operation:
  | expr PLUS expr {Binary ($1,Plus,$3) }
  | expr MINUS expr {Binary ($1,Minus,$3) }
  | expr ASTERISK expr {Binary ($1,Times,$3) }
  | expr RSLASH expr {Binary ($1,Div,$3) }
  | expr ACCESS expr {Binary ($1,Access,$3) }
  | DEREF expr { UnaryL (Deref,$2) }
  | REF expr { UnaryL (Ref,$2) }
  | expr FACT { UnaryR ($1,Fact) }
