use tree::{Expr, Let, Statement, AST};

pub mod object;
pub mod tree;

pub trait Visitor<T>
{
    type Error;
    fn visit_ast<'a>(&'a mut self,ast: &'a Box<AST<'a>>) -> Result<T,Self::Error>;
    fn visit_stm<'a>(&'a mut self,ast: &'a Box<Statement<'a>>) -> Result<T,Self::Error>;
    fn visit_let<'a>(&'a mut self,ast: &'a Box<Let<'a>>) -> Result<T,Self::Error>;
    fn visit_expr<'a>(&'a mut self,ast: &'a Box<Expr<'a>>) -> Result<T,Self::Error>;
}
