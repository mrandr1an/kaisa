use crate::frontend::kaisa::tree::{Expr, AST};

#[derive(Debug)]
pub struct Function<'a>
{
  name: &'a str,
  args: Box<[Expr<'a>]>,
  body: Box<AST<'a>>,
}
