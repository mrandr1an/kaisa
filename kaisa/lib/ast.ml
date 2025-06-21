module ParsedAst = struct
  type 'a node = {
    value: 'a;
    loc: int*int;
  }[@@deriving show]

  let mk_node value start_pos end_pos =
    {value; loc = (start_pos,end_pos)}

  type id = string
  [@@deriving show]
  and
    variable = id*id
    [@@deriving show]
  and
    value =
    | DefaultNumber of string node
    | DecimalNumber of string node
    | BinaryNumber of string node
    | HexNumber of string node
    | Character of string node
    | String of string node
    | List of expr list node
    | Tuple of expr list node
    [@@deriving show]
  and expr =
    | Id of string node
    | Val of value node
    | FuncCall of string*expr list
    | Operation of operation node
    | PatternMatch of expr*(expr list)*(expr) node
    [@@deriving show]
  and operation =
    | Binary of expr*binop*expr 
    | UnaryL of prefixop*expr 
    | UnaryR of expr*postfixop 
    [@@deriving show]
  and binop =
    | Plus
    | Minus
    | Times
    | Div
    | Access
  and prefixop =
    | Deref
    | Ref
    [@@deriving show]
  and postfixop =
    | Fact
    [@@deriving show]
  and stm =
    | LetVal of id
    | LetFn of id*(variable list option)*expr
    | LetType of id*(variable list option)*expr
    | LetIn of stm*(stm option)
  and ast = stm list
    [@@deriving show]
end

module SemanticAst = struct

end
