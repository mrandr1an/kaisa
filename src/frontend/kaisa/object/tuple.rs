use crate::frontend::kaisa::tree::Expr;

#[derive(PartialEq,Debug)]
pub struct Tuple<'a>(pub Box<[Expr<'a>]>,pub usize);
