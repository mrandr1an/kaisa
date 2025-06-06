use crate::frontend::kaisa::tree::Expr;

use super::ty::Type;

#[derive(Debug)]
pub struct List<'a>(Box<Type<'a>>,pub Box<[Expr<'a>]>,usize);
